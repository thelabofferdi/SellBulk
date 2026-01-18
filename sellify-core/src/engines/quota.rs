use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Quota tracking structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaLimits {
    pub messages_per_day: u32,
    pub messages_per_week: u32,
    pub images_per_day: u32,
    pub videos_per_week: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaUsage {
    pub messages_today: u32,
    pub messages_this_week: u32,
    pub images_today: u32,
    pub videos_this_week: u32,
    pub last_reset: chrono::DateTime<chrono::Utc>,
}

/// Quota & Anti-Ban Engine - Prevents automated-looking behavior
pub struct QuotaEngine {
    limits: QuotaLimits,
    usage: QuotaUsage,
}

impl QuotaEngine {
    pub fn new(limits: QuotaLimits) -> Self {
        Self {
            limits,
            usage: QuotaUsage {
                messages_today: 0,
                messages_this_week: 0,
                images_today: 0,
                videos_this_week: 0,
                last_reset: chrono::Utc::now(),
            },
        }
    }

    /// Check if sending a message would exceed quotas
    pub fn can_send_message(&self) -> bool {
        self.usage.messages_today < self.limits.messages_per_day
            && self.usage.messages_this_week < self.limits.messages_per_week
    }

    /// Check if sending media would exceed quotas
    pub fn can_send_media(&self, is_video: bool) -> bool {
        if is_video {
            self.usage.videos_this_week < self.limits.videos_per_week
        } else {
            self.usage.images_today < self.limits.images_per_day
        }
    }

    /// Record message sent
    pub fn record_message(&mut self) -> Result<()> {
        self.usage.messages_today += 1;
        self.usage.messages_this_week += 1;
        Ok(())
    }

    /// Calculate delay to avoid detection (random jitter + minimum delay)
    pub fn calculate_delay(&self) -> u64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Base delay: 2-8 seconds
        let base_delay = rng.gen_range(2..8);
        
        // Progressive delay based on usage
        let usage_ratio = self.usage.messages_today as f64 / self.limits.messages_per_day as f64;
        let progressive_factor = if usage_ratio > 0.8 {
            3 // Near limit: triple delay
        } else if usage_ratio > 0.5 {
            2 // Half capacity: double delay
        } else {
            1 // Low usage: normal delay
        };
        
        base_delay * progressive_factor
    }
    
    /// Record media sent
    pub fn record_media(&mut self, is_video: bool) -> Result<()> {
        if is_video {
            self.usage.videos_this_week += 1;
        } else {
            self.usage.images_today += 1;
        }
        Ok(())
    }
    
    /// Get current usage
    pub fn get_usage(&self) -> &QuotaUsage {
        &self.usage
    }
    
    /// Reset daily quotas (messages_today, images_today)
    pub fn reset_daily(&mut self) {
        self.usage.messages_today = 0;
        self.usage.images_today = 0;
        self.usage.last_reset = chrono::Utc::now();
    }
    
    /// Reset weekly quotas (messages_this_week, videos_this_week)
    pub fn reset_weekly(&mut self) {
        self.usage.messages_this_week = 0;
        self.usage.videos_this_week = 0;
        self.usage.last_reset = chrono::Utc::now();
    }
    
    /// Check if daily reset is needed (based on last_reset)
    pub fn needs_daily_reset(&self) -> bool {
        let now = chrono::Utc::now();
        let last_reset_date = self.usage.last_reset.date_naive();
        let today = now.date_naive();
        
        today > last_reset_date
    }
    
    /// Check if weekly reset is needed (Monday 00:00)
    pub fn needs_weekly_reset(&self) -> bool {
        use chrono::Datelike;
        
        let now = chrono::Utc::now();
        let last_reset = self.usage.last_reset;
        
        // Get week number for both dates
        let current_week = now.iso_week().week();
        let last_reset_week = last_reset.iso_week().week();
        
        // Also check year to handle year transitions
        let current_year = now.year();
        let last_reset_year = last_reset.year();
        
        (current_year > last_reset_year) || (current_week > last_reset_week)
    }
}

impl Default for QuotaEngine {
    fn default() -> Self {
        Self::new(QuotaLimits {
            messages_per_day: 200,
            messages_per_week: 1000,
            images_per_day: 50,
            videos_per_week: 20,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_send_message_when_under_quota() {
        let engine = QuotaEngine::default();
        assert!(engine.can_send_message());
    }

    #[test]
    fn test_cannot_send_when_day_quota_exceeded() {
        let mut engine = QuotaEngine::new(QuotaLimits {
            messages_per_day: 2,
            messages_per_week: 1000,
            images_per_day: 50,
            videos_per_week: 20,
        });
        
        engine.record_message().unwrap();
        engine.record_message().unwrap();
        
        assert!(!engine.can_send_message());
    }

    #[test]
    fn test_can_send_media_image() {
        let engine = QuotaEngine::default();
        assert!(engine.can_send_media(false));
    }

    #[test]
    fn test_can_send_media_video() {
        let engine = QuotaEngine::default();
        assert!(engine.can_send_media(true));
    }

    #[test]
    fn test_record_message_increments() {
        let mut engine = QuotaEngine::default();
        let before = engine.usage.messages_today;
        engine.record_message().unwrap();
        assert_eq!(engine.usage.messages_today, before + 1);
    }

    #[test]
    fn test_calculate_delay_in_range() {
        let engine = QuotaEngine::default();
        for _ in 0..10 {
            let delay = engine.calculate_delay();
            assert!(delay >= 2 && delay <= 24); // Max 8*3 for progressive
        }
    }
    
    #[test]
    fn test_reset_daily() {
        let mut engine = QuotaEngine::default();
        
        // Record some usage
        engine.record_message().unwrap();
        engine.record_media(false).unwrap(); // image
        
        assert_eq!(engine.usage.messages_today, 1);
        assert_eq!(engine.usage.images_today, 1);
        
        // Reset daily
        engine.reset_daily();
        
        assert_eq!(engine.usage.messages_today, 0);
        assert_eq!(engine.usage.images_today, 0);
        // Weekly counters should NOT be reset
        assert_eq!(engine.usage.messages_this_week, 1);
    }
    
    #[test]
    fn test_reset_weekly() {
        let mut engine = QuotaEngine::default();
        
        // Record some usage
        engine.record_message().unwrap();
        engine.record_media(true).unwrap(); // video
        
        assert_eq!(engine.usage.messages_this_week, 1);
        assert_eq!(engine.usage.videos_this_week, 1);
        
        // Reset weekly
        engine.reset_weekly();
        
        assert_eq!(engine.usage.messages_this_week, 0);
        assert_eq!(engine.usage.videos_this_week, 0);
        // Daily counters should NOT be reset
        assert_eq!(engine.usage.messages_today, 1);
    }
    
    #[test]
    fn test_needs_daily_reset_false_when_same_day() {
        let engine = QuotaEngine::default();
        assert!(!engine.needs_daily_reset()); // Just created, same day
    }
    
    #[test]
    fn test_needs_daily_reset_true_when_different_day() {
        let mut engine = QuotaEngine::default();
        
        // Simulate last reset was yesterday
        engine.usage.last_reset = chrono::Utc::now() - chrono::Duration::days(1);
        
        assert!(engine.needs_daily_reset());
    }
    
    #[test]
    fn test_needs_weekly_reset_false_when_same_week() {
        let engine = QuotaEngine::default();
        assert!(!engine.needs_weekly_reset()); // Just created, same week
    }
    
    #[test]
    fn test_needs_weekly_reset_true_when_different_week() {
        let mut engine = QuotaEngine::default();
        
        // Simulate last reset was 8 days ago (different week)
        engine.usage.last_reset = chrono::Utc::now() - chrono::Duration::days(8);
        
        assert!(engine.needs_weekly_reset());
    }
}