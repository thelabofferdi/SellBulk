use axum::{
    Router,
    routing::{get, post},
};
use crate::api::handlers::{self, AppState};

/// Create main API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // Metrics endpoint (Prometheus)
        .route("/metrics", get(handlers::metrics))
        
        // Decision Engine routes
        .route("/api/v1/decision", post(handlers::make_decision))
        
        // Validation routes
        .route("/api/v1/validate", post(handlers::validate_text))
        
        // Quota routes
        .route("/api/v1/quota/check", post(handlers::check_quota))
        .route("/api/v1/quota/record", post(handlers::record_message))
        .route("/api/v1/quota/status", get(handlers::get_quota_status))
        .route("/api/v1/quota/reset/daily", post(handlers::reset_daily_quota))
        .route("/api/v1/quota/reset/weekly", post(handlers::reset_weekly_quota))
        
        // Conversation routes
        .route("/api/v1/conversation/transition", post(handlers::transition_state))
        
        // Knowledge Base routes
        .route("/api/v1/products", get(handlers::list_products))
        .route("/api/v1/products/:id", get(handlers::get_product))
        
        // Audit routes
        .route("/api/v1/audit/log", post(handlers::log_audit))
}
