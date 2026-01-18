// Sellify Backend - Engine Modules
// Architecture: 11 deterministic engines as per PRD

pub mod license;
pub mod storage;
pub mod config;
pub mod knowledge_base;
pub mod conversation;
pub mod quota;
pub mod decision;
pub mod alert;
pub mod ia_gateway;
pub mod anti_hallucination;
pub mod audit;

// Re-exports for convenience
pub use license::LicenseEngine;
pub use storage::StorageEngine;
pub use config::ConfigEngine;
pub use knowledge_base::KnowledgeBaseEngine;
pub use conversation::ConversationEngine;
pub use quota::QuotaEngine;
pub use decision::DecisionEngine;
pub use alert::AlertEngine;
pub use ia_gateway::IAGateway;
pub use anti_hallucination::AntiHallucinationEngine;
pub use audit::AuditEngine;
