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
        // TODO: Implement progressive delay logic
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(2..8) // 2-8 seconds
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
