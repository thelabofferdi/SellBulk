use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Closed set of actions - Decision Engine can ONLY choose from these
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    RespondText { text: String },
    RespondWithMedia { text: String, media_id: String },
    Ignore,
    Delay { seconds: u64 },
    AlertHuman { reason: String },
    StopAutomation,
}

/// Decision context - all inputs for decision making
#[derive(Debug, Clone)]
pub struct DecisionContext {
    pub incoming_message: String,
    pub conversation_state: String,
    pub quotas_available: bool,
    pub is_active_hours: bool,
    pub sentiment_detected: Option<String>,
}

/// Decision Engine - The CORE system that decides "what to do"
/// IMPORTANT: This engine decides actions, NEVER how AI speaks
pub struct DecisionEngine;

impl DecisionEngine {
    /// Creates a new Decision Engine
    pub fn new() -> Self {
        Self
    }

    /// Main decision function - returns ONE action from the closed set
    /// Inputs: message, state, quotas, rules, schedule, knowledge base
    /// Output: ONE deterministic action
    pub fn decide(&self, context: DecisionContext) -> Result<Action> {
        // Rule 1: If outside active hours -> Ignore
        if !context.is_active_hours {
            return Ok(Action::Ignore);
        }

        // Rule 2: If quotas exceeded -> Stop or Delay
        if !context.quotas_available {
            return Ok(Action::Delay { seconds: 3600 }); // Wait 1 hour
        }

        // Rule 3: If anger/threat detected -> Alert human + Stop
        if let Some(sentiment) = &context.sentiment_detected {
            if sentiment == "anger" || sentiment == "threat" {
                return Ok(Action::AlertHuman {
                    reason: format!("Sentiment detected: {}", sentiment),
                });
            }
        }

        // Rule 4: Normal flow - Respond with text
        // Note: The actual text generation is delegated to IA Gateway
        Ok(Action::RespondText {
            text: String::new(), // Will be filled by IA Gateway
        })
    }

    /// Validates that an action is allowed in current context
    pub fn validate_action(&self, action: &Action, context: &DecisionContext) -> bool {
        match action {
            Action::RespondText { .. } | Action::RespondWithMedia { .. } => {
                context.is_active_hours && context.quotas_available
            }
            Action::Ignore | Action::Delay { .. } => true,
            Action::AlertHuman { .. } => true,
            Action::StopAutomation => true,
        }
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_engine_ignore_outside_hours() {
        let engine = DecisionEngine::new();
        let context = DecisionContext {
            incoming_message: "Hello".to_string(),
            conversation_state: "Discovery".to_string(),
            quotas_available: true,
            is_active_hours: false,
            sentiment_detected: None,
        };

        let action = engine.decide(context).unwrap();
        assert_eq!(action, Action::Ignore);
    }

    #[test]
    fn test_decision_engine_alert_on_threat() {
        let engine = DecisionEngine::new();
        let context = DecisionContext {
            incoming_message: "I'll sue you!".to_string(),
            conversation_state: "Discovery".to_string(),
            quotas_available: true,
            is_active_hours: true,
            sentiment_detected: Some("threat".to_string()),
        };

        let action = engine.decide(context).unwrap();
        assert!(matches!(action, Action::AlertHuman { .. }));
    }
}
