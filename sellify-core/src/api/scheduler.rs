use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use anyhow::Result;

/// Quota reset scheduler - handles daily and weekly resets
pub struct QuotaScheduler {
    scheduler: JobScheduler,
}

impl QuotaScheduler {
    /// Create new quota scheduler
    pub async fn new() -> Result<Self> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self { scheduler })
    }

    /// Start daily reset job (00:00 UTC)
    /// Resets: messages_today, images_today
    pub async fn start_daily_reset<F>(&mut self, callback: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let callback = Arc::new(callback);
        
        let job = Job::new_async("0 0 0 * * *", move |_uuid, _l| {
            let callback = Arc::clone(&callback);
            Box::pin(async move {
                log::info!("🔄 Running daily quota reset (00:00 UTC)");
                callback();
                log::info!("✅ Daily quota reset completed");
            })
        })?;

        self.scheduler.add(job).await?;
        log::info!("📅 Daily reset job scheduled (00:00 UTC)");
        Ok(())
    }

    /// Start weekly reset job (Monday 00:00 UTC)
    /// Resets: messages_this_week, videos_this_week
    pub async fn start_weekly_reset<F>(&mut self, callback: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let callback = Arc::new(callback);
        
        let job = Job::new_async("0 0 0 * * Mon", move |_uuid, _l| {
            let callback = Arc::clone(&callback);
            Box::pin(async move {
                log::info!("🔄 Running weekly quota reset (Monday 00:00 UTC)");
                callback();
                log::info!("✅ Weekly quota reset completed");
            })
        })?;

        self.scheduler.add(job).await?;
        log::info!("📅 Weekly reset job scheduled (Monday 00:00 UTC)");
        Ok(())
    }

    /// Start the scheduler (begin running jobs)
    pub async fn start(&self) -> Result<()> {
        self.scheduler.start().await?;
        log::info!("🚀 Quota scheduler started");
        Ok(())
    }

    /// Shutdown the scheduler
    pub async fn shutdown(&mut self) -> Result<()> {
        self.scheduler.shutdown().await?;
        log::info!("🛑 Quota scheduler stopped");
        Ok(())
    }
}

/// Shared state for quota engine with thread-safe access
pub type SharedQuotaEngine = Arc<Mutex<crate::engines::quota::QuotaEngine>>;

/// Setup automatic quota resets for a quota engine
/// Returns the scheduler (must be kept alive)
pub async fn setup_auto_reset(quota_engine: SharedQuotaEngine) -> Result<QuotaScheduler> {
    let mut scheduler = QuotaScheduler::new().await?;

    // Daily reset job
    {
        let quota_engine = Arc::clone(&quota_engine);
        scheduler
            .start_daily_reset(move || {
                let quota_engine = Arc::clone(&quota_engine);
                tokio::spawn(async move {
                    let mut engine = quota_engine.lock().await;
                    engine.reset_daily();
                    
                    // Record metric
                    crate::api::metrics::QUOTA_RESETS_TOTAL
                        .with_label_values(&["daily"])
                        .inc();
                    
                    // Update gauges
                    let usage = engine.get_usage();
                    crate::api::metrics::QUOTA_MESSAGES_TODAY.set(usage.messages_today as f64);
                    crate::api::metrics::QUOTA_IMAGES_TODAY.set(usage.images_today as f64);
                });
            })
            .await?;
    }

    // Weekly reset job
    {
        let quota_engine = Arc::clone(&quota_engine);
        scheduler
            .start_weekly_reset(move || {
                let quota_engine = Arc::clone(&quota_engine);
                tokio::spawn(async move {
                    let mut engine = quota_engine.lock().await;
                    engine.reset_weekly();
                    
                    // Record metric
                    crate::api::metrics::QUOTA_RESETS_TOTAL
                        .with_label_values(&["weekly"])
                        .inc();
                    
                    // Update gauges
                    let usage = engine.get_usage();
                    crate::api::metrics::QUOTA_MESSAGES_WEEK.set(usage.messages_this_week as f64);
                    crate::api::metrics::QUOTA_VIDEOS_WEEK.set(usage.videos_this_week as f64);
                });
            })
            .await?;
    }

    scheduler.start().await?;
    Ok(scheduler)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scheduler_creation() {
        let scheduler = QuotaScheduler::new().await;
        assert!(scheduler.is_ok());
    }

    #[tokio::test]
    async fn test_daily_reset_job_added() {
        let mut scheduler = QuotaScheduler::new().await.unwrap();
        let result = scheduler.start_daily_reset(|| {}).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_weekly_reset_job_added() {
        let mut scheduler = QuotaScheduler::new().await.unwrap();
        let result = scheduler.start_weekly_reset(|| {}).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_setup_auto_reset() {
        use crate::engines::quota::{QuotaEngine, QuotaLimits};
        
        let quota_engine = Arc::new(Mutex::new(QuotaEngine::new(QuotaLimits {
            messages_per_day: 100,
            messages_per_week: 500,
            images_per_day: 20,
            videos_per_week: 10,
        })));

        let scheduler = setup_auto_reset(quota_engine).await;
        assert!(scheduler.is_ok());
        
        // Shutdown to clean up
        let mut scheduler = scheduler.unwrap();
        scheduler.shutdown().await.unwrap();
    }
}
