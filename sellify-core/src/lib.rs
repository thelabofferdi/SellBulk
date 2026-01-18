//! Sellify Core - Deterministic WhatsApp Sales Automation Engine
//! 
//! This library implements 11 deterministic engines as per PRD:
//! 1. License Engine - Authorization and key management
//! 2. Storage Engine - Encrypted local storage
//! 3. Config Engine - Global parameters
//! 4. Knowledge Base Engine - Product catalog
//! 5. Conversation Engine - State machine
//! 6. Quota Engine - Anti-ban quotas
//! 7. Decision Engine - Core decision logic
//! 8. Alert Engine - Human notifications
//! 9. IA Gateway - Text generation
//! 10. Anti-Hallucination Engine - Validation
//! 11. Audit Engine - Complete traceability

pub mod engines;

// API HTTP (optional)
#[cfg(feature = "http-server")]
pub mod api;

// Re-export all engines for convenience
pub use engines::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engines_compile() {
        // Basic smoke test to ensure all engines compile
        let _license = LicenseEngine::new();
        let _config = ConfigEngine::new();
        let _decision = DecisionEngine::new();
        let _quota = QuotaEngine::default();
        let _kb = KnowledgeBaseEngine::new();
        let _conv = ConversationEngine::new();
        let _alert = AlertEngine::default();
        let _ia = IAGateway::default();
        let _anti_hal = AntiHallucinationEngine::new();
        let _audit = AuditEngine::new();
    }
}
