use anyhow::{Result, anyhow};

/// Anti-Hallucination Engine - Double lock (before + after AI)
pub struct AntiHallucinationEngine {
    forbidden_words: Vec<String>,
    max_length: usize,
}

impl AntiHallucinationEngine {
    pub fn new() -> Self {
        Self {
            forbidden_words: vec![
                "AI".to_string(),
                "intelligence artificielle".to_string(),
                "robot".to_string(),
                "humain".to_string(),
                "transférer".to_string(),
                "escalade".to_string(),
            ],
            max_length: 500,
        }
    }

    /// Validates BEFORE AI generation
    pub fn validate_before_ai(&self, product_id: &str, action: &str) -> Result<()> {
        // TODO: Validate product exists
        // TODO: Validate action is in allowed set
        // TODO: Validate quotas
        log::info!("Pre-AI validation for product: {}, action: {}", product_id, action);
        Ok(())
    }

    /// Validates AFTER AI generation
    pub fn validate_after_ai(&self, generated_text: &str) -> Result<String> {
        // Length check
        if generated_text.len() > self.max_length {
            return Err(anyhow!("Generated text exceeds max length"));
        }

        // Forbidden words check
        let text_lower = generated_text.to_lowercase();
        for forbidden in &self.forbidden_words {
            if text_lower.contains(forbidden) {
                return Err(anyhow!("Generated text contains forbidden word: {}", forbidden));
            }
        }

        // TODO: Check for commercial promises
        // TODO: Lexical filtering

        Ok(generated_text.to_string())
    }

    /// Returns neutral fallback if validation fails
    pub fn get_fallback_message(&self) -> String {
        "Désolé, je n'ai pas bien compris votre demande. Pouvez-vous préciser ?".to_string()
    }
}

impl Default for AntiHallucinationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_forbidden_words() {
        let engine = AntiHallucinationEngine::new();
        let text = "Je suis une AI qui peut vous aider";
        assert!(engine.validate_after_ai(text).is_err());
    }

    #[test]
    fn test_accept_valid_text() {
        let engine = AntiHallucinationEngine::new();
        let text = "Bonjour, comment puis-je vous aider ?";
        assert!(engine.validate_after_ai(text).is_ok());
    }
}
