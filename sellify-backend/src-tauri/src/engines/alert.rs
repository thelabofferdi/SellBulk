use anyhow::Result;

/// Alert triggers
#[derive(Debug, Clone)]
pub enum AlertTrigger {
    Anger,
    Threat,
    RepeatedMisunderstanding,
    SensitiveWord(String),
    LegalSituation,
}

/// Alert Engine - Notifies humans without revealing to client
pub struct AlertEngine {
    alert_numbers: Vec<String>,
}

impl AlertEngine {
    pub fn new(alert_numbers: Vec<String>) -> Self {
        Self { alert_numbers }
    }

    /// Send alert to internal WhatsApp number
    /// Client sees silence or neutral response
    pub async fn send_alert(&self, trigger: AlertTrigger, conversation_id: &str) -> Result<()> {
        let message = format!(
            "🚨 ALERT - Conversation: {}\nTrigger: {:?}\nRequires human intervention",
            conversation_id, trigger
        );

        // TODO: Send via WhatsApp Gateway to alert_numbers
        log::warn!("Alert: {}", message);
        Ok(())
    }

    /// Check if message contains sensitive content
    pub fn detect_trigger(&self, message: &str) -> Option<AlertTrigger> {
        let message_lower = message.to_lowercase();

        // Check for legal keywords
        let legal_keywords = ["avocat", "tribunal", "police", "procès"];
        for keyword in legal_keywords {
            if message_lower.contains(keyword) {
                return Some(AlertTrigger::LegalSituation);
            }
        }

        // Check for threats
        let threat_keywords = ["tuer", "frapper", "détruire"];
        for keyword in threat_keywords {
            if message_lower.contains(keyword) {
                return Some(AlertTrigger::Threat);
            }
        }

        None
    }
}

impl Default for AlertEngine {
    fn default() -> Self {
        Self::new(vec![])
    }
}
