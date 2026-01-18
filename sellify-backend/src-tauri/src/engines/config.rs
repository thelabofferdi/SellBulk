use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Global configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Active hours for automated responses
    pub active_hours: ActiveHours,
    
    /// Response delay configuration
    pub response_delay: ResponseDelay,
    
    /// AI activation flag
    pub ai_enabled: bool,
    
    /// Alert WhatsApp numbers
    pub alert_numbers: Vec<String>,
    
    /// Anti-ban strategy settings
    pub anti_ban: AntiBanConfig,
    
    /// Escalation thresholds
    pub escalation_threshold: EscalationThreshold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveHours {
    pub start: String, // Format: "09:00"
    pub end: String,   // Format: "18:00"
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDelay {
    pub min_seconds: u64,
    pub max_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiBanConfig {
    pub max_messages_per_day: u32,
    pub max_messages_per_hour: u32,
    pub typing_simulation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationThreshold {
    pub max_misunderstandings: u32,
    pub sensitive_keywords: Vec<String>,
}

/// Config Engine - Centralizes all global parameters
pub struct ConfigEngine {
    config: GlobalConfig,
}

impl ConfigEngine {
    /// Creates a new Config Engine with default values
    pub fn new() -> Self {
        Self {
            config: Self::default_config(),
        }
    }

    /// Returns default configuration
    fn default_config() -> GlobalConfig {
        GlobalConfig {
            active_hours: ActiveHours {
                start: "09:00".to_string(),
                end: "18:00".to_string(),
                timezone: "UTC".to_string(),
            },
            response_delay: ResponseDelay {
                min_seconds: 2,
                max_seconds: 8,
            },
            ai_enabled: true,
            alert_numbers: vec![],
            anti_ban: AntiBanConfig {
                max_messages_per_day: 200,
                max_messages_per_hour: 30,
                typing_simulation: true,
            },
            escalation_threshold: EscalationThreshold {
                max_misunderstandings: 3,
                sensitive_keywords: vec![
                    "avocat".to_string(),
                    "tribunal".to_string(),
                    "police".to_string(),
                ],
            },
        }
    }

    /// Loads configuration from storage
    pub fn load(&mut self, _config_data: &str) -> Result<()> {
        // TODO: Load from encrypted storage
        Ok(())
    }

    /// Gets the current configuration
    pub fn get_config(&self) -> &GlobalConfig {
        &self.config
    }

    /// Updates configuration
    pub fn update_config(&mut self, config: GlobalConfig) -> Result<()> {
        self.config = config;
        // TODO: Persist to encrypted storage
        Ok(())
    }

    /// Checks if current time is within active hours
    pub fn is_active_now(&self) -> bool {
        // TODO: Implement time check with timezone
        true
    }
}

impl Default for ConfigEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_engine_creation() {
        let engine = ConfigEngine::new();
        assert!(engine.get_config().ai_enabled);
    }

    #[test]
    fn test_default_config() {
        let engine = ConfigEngine::new();
        let config = engine.get_config();
        assert_eq!(config.active_hours.start, "09:00");
        assert_eq!(config.response_delay.min_seconds, 2);
    }
}
