use axum::{Router, middleware};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use crate::engines::*;
use crate::api::{routes, handlers::AppState, auth, rate_limit::RateLimiter};

/// Create and configure the Axum application
pub fn create_app() -> Router {
    create_app_with_config(None, None)
}

/// Create app with custom rate limiter config
pub fn create_app_with_config(
    api_key: Option<String>,
    rate_limiter: Option<RateLimiter>,
) -> Router {
    // Set API key in environment if provided
    if let Some(key) = api_key {
        std::env::set_var("SELLIFY_API_KEY", key);
    }
    
    // Initialize Prometheus metrics
    let _ = crate::api::metrics::init_metrics();
    
    // Initialize engines
    let decision_engine = Arc::new(DecisionEngine::new());
    let anti_hallucination = Arc::new(AntiHallucinationEngine::new());
    let conversation_engine = Arc::new(ConversationEngine::new());
    let quota_engine = Arc::new(Mutex::new(QuotaEngine::default()));
    let knowledge_base = Arc::new(Mutex::new(KnowledgeBaseEngine::new()));
    let audit_engine = Arc::new(AuditEngine::new());
    
    let state = AppState {
        decision_engine,
        anti_hallucination,
        conversation_engine,
        quota_engine,
        knowledge_base,
        audit_engine,
    };
    
    // Create router with middleware
    routes::create_router()
        // Authentication middleware (checks API key)
        .layer(middleware::from_fn(auth::auth_middleware))
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        // Tracing
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    
    #[tokio::test]
    async fn test_health_check() {
        let app = create_app();
        
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_decision_endpoint_requires_auth() {
        let app = create_app();
        
        let request_body = serde_json::json!({
            "incoming_message": "Bonjour",
            "conversation_state": "Discovery",
            "quotas_available": true,
            "is_active_hours": true,
            "sentiment_detected": null
        });
        
        // Without API key - should fail
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/decision")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap()
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[tokio::test]
    async fn test_decision_endpoint_with_auth() {
        let app = create_app_with_config(Some("test-api-key".to_string()), None);
        
        let request_body = serde_json::json!({
            "incoming_message": "Bonjour",
            "conversation_state": "Discovery",
            "quotas_available": true,
            "is_active_hours": true,
            "sentiment_detected": null
        });
        
        // With valid API key - should succeed
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/decision")
                    .method("POST")
                    .header("content-type", "application/json")
                    .header("X-API-Key", "test-api-key")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap()
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_metrics_endpoint_public() {
        let app = create_app();
        
        let response = app
            .oneshot(Request::builder().uri("/metrics").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        // Metrics endpoint should be public (no auth required)
        assert_eq!(response.status(), StatusCode::OK);
    }
}
