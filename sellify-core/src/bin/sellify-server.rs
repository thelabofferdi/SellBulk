use sellify_core::api::{create_app, setup_auto_reset};
use std::sync::Arc;
use tokio::sync::Mutex;
use sellify_core::engines::quota::{QuotaEngine, QuotaLimits};

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    
    // Create quota engine that will be shared with the scheduler
    let quota_engine = Arc::new(Mutex::new(QuotaEngine::new(QuotaLimits {
        messages_per_day: 200,
        messages_per_week: 1000,
        images_per_day: 50,
        videos_per_week: 20,
    })));
    
    // Setup automatic quota resets (daily at 00:00, weekly on Monday 00:00)
    log::info!("🕐 Setting up automatic quota reset scheduler...");
    let _scheduler = setup_auto_reset(Arc::clone(&quota_engine))
        .await
        .expect("Failed to setup quota scheduler");
    
    // Create application
    let app = create_app();
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    
    println!("🚀 Sellify Core API Server starting on http://0.0.0.0:3000");
    println!("📖 Health check: http://localhost:3000/health");
    println!("📡 API endpoints: http://localhost:3000/api/v1/");
    println!("🕐 Quota resets: Daily 00:00 UTC, Weekly Monday 00:00 UTC");
    
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
