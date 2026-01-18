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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_legal_trigger() {
        let engine = AlertEngine::default();
        let message = "Je vais contacter mon avocat";
        let trigger = engine.detect_trigger(message);
        assert!(matches!(trigger, Some(AlertTrigger::LegalSituation)));
    }

    #[test]
    fn test_detect_threat_trigger() {
        let engine = AlertEngine::default();
        let message = "Je vais te frapper";
        let trigger = engine.detect_trigger(message);
        assert!(matches!(trigger, Some(AlertTrigger::Threat)));
    }

    #[test]
    fn test_no_trigger_on_normal_message() {
        let engine = AlertEngine::default();
        let message = "Bonjour, je voudrais des informations";
        let trigger = engine.detect_trigger(message);
        assert!(trigger.is_none());
    }

    #[tokio::test]
    async fn test_send_alert() {
        let engine = AlertEngine::new(vec!["+33123456789".to_string()]);
        let result = engine.send_alert(AlertTrigger::Threat, "conv-001").await;
        assert!(result.is_ok());
    }
}
