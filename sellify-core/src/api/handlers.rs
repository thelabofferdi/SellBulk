use axum::{
    Json,
    http::StatusCode,
    extract::{State, Path},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::engines::*;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub decision_engine: Arc<DecisionEngine>,
    pub anti_hallucination: Arc<AntiHallucinationEngine>,
    pub conversation_engine: Arc<ConversationEngine>,
    pub quota_engine: Arc<tokio::sync::Mutex<QuotaEngine>>,
    pub knowledge_base: Arc<tokio::sync::Mutex<KnowledgeBaseEngine>>,
    pub audit_engine: Arc<AuditEngine>,
}

// ============== REQUEST/RESPONSE MODELS ==============

#[derive(Debug, Deserialize)]
pub struct DecisionRequest {
    pub incoming_message: String,
    pub conversation_state: String,
    pub quotas_available: bool,
    pub is_active_hours: bool,
    pub sentiment_detected: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DecisionResponse {
    pub action: String,
    pub details: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ValidationRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub validated_text: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QuotaCheckRequest {
    pub message_type: String, // "text", "image", "video"
}

#[derive(Debug, Serialize)]
pub struct QuotaCheckResponse {
    pub can_send: bool,
    pub delay_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct TransitionRequest {
    pub current_state: String,
    pub event: String,
}

#[derive(Debug, Serialize)]
pub struct TransitionResponse {
    pub new_state: String,
}

// ============== HANDLERS ==============

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "healthy",
        "service": "sellify-core",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

/// Make a decision based on context
pub async fn make_decision(
    State(state): State<AppState>,
    Json(req): Json<DecisionRequest>,
) -> Result<Json<DecisionResponse>, (StatusCode, String)> {
    let context = decision::DecisionContext {
        incoming_message: req.incoming_message,
        conversation_state: req.conversation_state.clone(),
        quotas_available: req.quotas_available,
        is_active_hours: req.is_active_hours,
        sentiment_detected: req.sentiment_detected,
    };
    
    match state.decision_engine.decide(context) {
        Ok(action) => {
            let (action_type, details) = match action {
                decision::Action::RespondText { text } => ("RespondText", Some(text)),
                decision::Action::RespondWithMedia { text, media_id } => {
                    ("RespondWithMedia", Some(format!("{} (media: {})", text, media_id)))
                }
                decision::Action::Ignore => ("Ignore", None),
                decision::Action::Delay { seconds } => {
                    ("Delay", Some(format!("{} seconds", seconds)))
                }
                decision::Action::AlertHuman { reason } => ("AlertHuman", Some(reason)),
                decision::Action::StopAutomation => ("StopAutomation", None),
            };
            
            // Record decision metric
            crate::api::metrics::DECISIONS_TOTAL
                .with_label_values(&[action_type])
                .inc();
            
            // Update conversation state gauge
            crate::api::metrics::CONVERSATION_STATE
                .with_label_values(&[&req.conversation_state])
                .set(1.0);
            
            Ok(Json(DecisionResponse {
                action: action_type.to_string(),
                details,
            }))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Validate AI-generated text
pub async fn validate_text(
    State(state): State<AppState>,
    Json(req): Json<ValidationRequest>,
) -> Result<Json<ValidationResponse>, (StatusCode, String)> {
    match state.anti_hallucination.validate_after_ai(&req.text) {
        Ok(validated) => {
            // Record successful validation
            crate::api::metrics::VALIDATION_RESULTS
                .with_label_values(&["valid"])
                .inc();
            
            Ok(Json(ValidationResponse {
                valid: true,
                validated_text: Some(validated),
                error: None,
            }))
        }
        Err(e) => {
            // Record failed validation
            crate::api::metrics::VALIDATION_RESULTS
                .with_label_values(&["invalid"])
                .inc();
            
            // Check if it was forbidden words
            if e.to_string().contains("forbidden word") {
                crate::api::metrics::FORBIDDEN_WORDS_DETECTED.inc();
            }
            
            Ok(Json(ValidationResponse {
                valid: false,
                validated_text: None,
                error: Some(e.to_string()),
            }))
        }
    }
}

/// Check if message can be sent (quota)
pub async fn check_quota(
    State(state): State<AppState>,
    Json(req): Json<QuotaCheckRequest>,
) -> Result<Json<QuotaCheckResponse>, (StatusCode, String)> {
    let quota = state.quota_engine.lock().await;
    
    let can_send = match req.message_type.as_str() {
        "text" => quota.can_send_message(),
        "image" => quota.can_send_media(false),
        "video" => quota.can_send_media(true),
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid message type".to_string())),
    };
    
    let delay = if can_send {
        Some(quota.calculate_delay())
    } else {
        None
    };
    
    Ok(Json(QuotaCheckResponse {
        can_send,
        delay_seconds: delay,
    }))
}

/// Record message sent
pub async fn record_message(
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut quota = state.quota_engine.lock().await;
    
    quota.record_message()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Update quota metrics
    let usage = quota.get_usage();
    crate::api::metrics::QUOTA_MESSAGES_TODAY.set(usage.messages_today as f64);
    crate::api::metrics::QUOTA_MESSAGES_WEEK.set(usage.messages_this_week as f64);
    
    Ok(StatusCode::OK)
}

/// Transition conversation state
pub async fn transition_state(
    State(state): State<AppState>,
    Json(req): Json<TransitionRequest>,
) -> Result<Json<TransitionResponse>, (StatusCode, String)> {
    // Parse state
    let current_state = match req.current_state.as_str() {
        "Discovery" => conversation::ConversationState::Discovery,
        "Interest" => conversation::ConversationState::Interest,
        "Intent" => conversation::ConversationState::Intent,
        "Objection" => conversation::ConversationState::Objection,
        "Negative" => conversation::ConversationState::Negative,
        "Escalated" => conversation::ConversationState::Escalated,
        "Frozen" => conversation::ConversationState::Frozen,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid state".to_string())),
    };
    
    // Parse event
    let event = match req.event.as_str() {
        "ProductQuestion" => conversation::ConversationEvent::ProductQuestion,
        "PriceInterest" => conversation::ConversationEvent::PriceInterest,
        "PurchaseIntent" => conversation::ConversationEvent::PurchaseIntent,
        "ObjectionRaised" => conversation::ConversationEvent::ObjectionRaised,
        "NegativeResponse" => conversation::ConversationEvent::NegativeResponse,
        "ThreatDetected" => conversation::ConversationEvent::ThreatDetected,
        "Freeze" => conversation::ConversationEvent::Freeze,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid event".to_string())),
    };
    
    let new_state = state.conversation_engine.transition(&current_state, event);
    let state_name = format!("{:?}", new_state);
    
    Ok(Json(TransitionResponse {
        new_state: state_name,
    }))
}

/// List all products
pub async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<knowledge_base::Product>>, (StatusCode, String)> {
    let kb = state.knowledge_base.lock().await;
    let products = kb.get_all_products().to_vec();
    Ok(Json(products))
}

/// Get product by ID
pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<knowledge_base::Product>, (StatusCode, String)> {
    let kb = state.knowledge_base.lock().await;
    
    match kb.get_product(&id) {
        Some(product) => Ok(Json(product.clone())),
        None => Err((StatusCode::NOT_FOUND, "Product not found".to_string())),
    }
}

/// Log audit entry
pub async fn log_audit(
    State(state): State<AppState>,
    Json(log): Json<audit::AuditLog>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.audit_engine.log_message_flow(log)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::CREATED)
}

// ============== QUOTA RESET HANDLERS ==============

#[derive(Debug, Serialize)]
pub struct QuotaResetResponse {
    pub reset_type: String,
    pub timestamp: String,
}

/// Manual daily quota reset
pub async fn reset_daily_quota(
    State(state): State<AppState>,
) -> Result<Json<QuotaResetResponse>, (StatusCode, String)> {
    let mut quota = state.quota_engine.lock().await;
    quota.reset_daily();
    
    // Record reset metric
    crate::api::metrics::QUOTA_RESETS_TOTAL
        .with_label_values(&["daily"])
        .inc();
    
    // Update quota gauges
    let usage = quota.get_usage();
    crate::api::metrics::QUOTA_MESSAGES_TODAY.set(usage.messages_today as f64);
    crate::api::metrics::QUOTA_IMAGES_TODAY.set(usage.images_today as f64);
    
    Ok(Json(QuotaResetResponse {
        reset_type: "daily".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }))
}

/// Manual weekly quota reset
pub async fn reset_weekly_quota(
    State(state): State<AppState>,
) -> Result<Json<QuotaResetResponse>, (StatusCode, String)> {
    let mut quota = state.quota_engine.lock().await;
    quota.reset_weekly();
    
    // Record reset metric
    crate::api::metrics::QUOTA_RESETS_TOTAL
        .with_label_values(&["weekly"])
        .inc();
    
    // Update quota gauges
    let usage = quota.get_usage();
    crate::api::metrics::QUOTA_MESSAGES_WEEK.set(usage.messages_this_week as f64);
    crate::api::metrics::QUOTA_VIDEOS_WEEK.set(usage.videos_this_week as f64);
    
    Ok(Json(QuotaResetResponse {
        reset_type: "weekly".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }))
}

/// Get quota status
#[derive(Debug, Serialize)]
pub struct QuotaStatusResponse {
    pub messages_today: u32,
    pub messages_this_week: u32,
    pub images_today: u32,
    pub videos_this_week: u32,
    pub last_reset: String,
    pub needs_daily_reset: bool,
    pub needs_weekly_reset: bool,
}

pub async fn get_quota_status(
    State(state): State<AppState>,
) -> Result<Json<QuotaStatusResponse>, (StatusCode, String)> {
    let quota = state.quota_engine.lock().await;
    let usage = quota.get_usage();
    
    // Update quota gauges for monitoring
    crate::api::metrics::QUOTA_MESSAGES_TODAY.set(usage.messages_today as f64);
    crate::api::metrics::QUOTA_MESSAGES_WEEK.set(usage.messages_this_week as f64);
    crate::api::metrics::QUOTA_IMAGES_TODAY.set(usage.images_today as f64);
    crate::api::metrics::QUOTA_VIDEOS_WEEK.set(usage.videos_this_week as f64);
    
    Ok(Json(QuotaStatusResponse {
        messages_today: usage.messages_today,
        messages_this_week: usage.messages_this_week,
        images_today: usage.images_today,
        videos_this_week: usage.videos_this_week,
        last_reset: usage.last_reset.to_rfc3339(),
        needs_daily_reset: quota.needs_daily_reset(),
        needs_weekly_reset: quota.needs_weekly_reset(),
    }))
}

// ============== METRICS HANDLER ==============

/// Prometheus metrics endpoint
pub async fn metrics() -> Result<String, (StatusCode, String)> {
    crate::api::metrics::gather_metrics()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
