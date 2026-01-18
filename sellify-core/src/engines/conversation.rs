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

/// Event signals that trigger state transitions
#[derive(Debug, Clone, PartialEq)]
pub enum ConversationEvent {
    ProductQuestion,
    PriceInterest,
    PurchaseIntent,
    ObjectionRaised,
    NegativeResponse,
    ThreatDetected,
    Freeze,
}

/// Conversation Engine - Maintains objective state of discussions
pub struct ConversationEngine;

impl ConversationEngine {
    pub fn new() -> Self {
        Self
    }

    /// Transitions are deterministic and rule-based, NEVER AI-decided
    pub fn transition(&self, current: &ConversationState, event: ConversationEvent) -> ConversationState {
        use ConversationState::*;
        use ConversationEvent::*;
        
        match (current, event) {
            // From Discovery
            (Discovery, ProductQuestion) => Interest,
            (Discovery, NegativeResponse) => Negative,
            (Discovery, ThreatDetected) => Escalated,
            
            // From Interest
            (Interest, PriceInterest) => Intent,
            (Interest, ObjectionRaised) => Objection,
            (Interest, NegativeResponse) => Negative,
            (Interest, ThreatDetected) => Escalated,
            
            // From Intent
            (Intent, PurchaseIntent) => Intent, // Stay in Intent
            (Intent, ObjectionRaised) => Objection,
            (Intent, NegativeResponse) => Negative,
            (Intent, ThreatDetected) => Escalated,
            
            // From Objection
            (Objection, ProductQuestion) => Interest,
            (Objection, NegativeResponse) => Negative,
            (Objection, ThreatDetected) => Escalated,
            
            // From Negative/Escalated - frozen states
            (Negative, Freeze) => Frozen,
            (Escalated, Freeze) => Frozen,
            (Escalated, _) => Escalated, // Stay escalated
            (Negative, _) => Negative,   // Stay negative
            (Frozen, _) => Frozen,       // Stay frozen
            
            // Default: stay in current state
            _ => current.clone(),
        }
    }

    pub fn get_initial_state() -> ConversationState {
        ConversationState::Discovery
    }
    
    /// Check if state is terminal (no automation)
    pub fn is_terminal_state(state: &ConversationState) -> bool {
        matches!(state, ConversationState::Escalated | ConversationState::Frozen)
    }
}

impl Default for ConversationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let state = ConversationEngine::get_initial_state();
        assert_eq!(state, ConversationState::Discovery);
    }

    #[test]
    fn test_discovery_to_interest() {
        let engine = ConversationEngine::new();
        let current = ConversationState::Discovery;
        let next = engine.transition(&current, ConversationEvent::ProductQuestion);
        assert_eq!(next, ConversationState::Interest);
    }

    #[test]
    fn test_interest_to_intent() {
        let engine = ConversationEngine::new();
        let current = ConversationState::Interest;
        let next = engine.transition(&current, ConversationEvent::PriceInterest);
        assert_eq!(next, ConversationState::Intent);
    }

    #[test]
    fn test_threat_escalates_immediately() {
        let engine = ConversationEngine::new();
        let current = ConversationState::Discovery;
        let next = engine.transition(&current, ConversationEvent::ThreatDetected);
        assert_eq!(next, ConversationState::Escalated);
    }

    #[test]
    fn test_escalated_is_terminal() {
        assert!(ConversationEngine::is_terminal_state(&ConversationState::Escalated));
        assert!(ConversationEngine::is_terminal_state(&ConversationState::Frozen));
        assert!(!ConversationEngine::is_terminal_state(&ConversationState::Discovery));
    }

    #[test]
    fn test_objection_can_return_to_interest() {
        let engine = ConversationEngine::new();
        let current = ConversationState::Objection;
        let next = engine.transition(&current, ConversationEvent::ProductQuestion);
        assert_eq!(next, ConversationState::Interest);
    }
}
