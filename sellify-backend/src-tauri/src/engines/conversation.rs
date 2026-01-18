use serde::{Deserialize, Serialize};

/// Conversation states - deterministic, rule-based transitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationState {
    Discovery,
    Interest,
    Intent,
    Objection,
    Negative,
    Escalated,
    Frozen,
}

/// Conversation Engine - Maintains objective state of discussions
pub struct ConversationEngine;

impl ConversationEngine {
    pub fn new() -> Self {
        Self
    }

    /// Transitions are deterministic and rule-based, NEVER AI-decided
    pub fn transition(&self, current: &ConversationState, _event: &str) -> ConversationState {
        // TODO: Implement deterministic state machine
        current.clone()
    }

    pub fn get_initial_state() -> ConversationState {
        ConversationState::Discovery
    }
}

impl Default for ConversationEngine {
    fn default() -> Self {
        Self::new()
    }
}
