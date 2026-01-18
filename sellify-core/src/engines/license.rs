use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use machine_uid;

/// License states according to PRD
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseState {
    Valid,
    Invalid,
    Expired,
    HwidMismatch,
    Tampered,
}

/// License structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub hwid: String,
    pub activation_key: String,
    pub ai_api_key: Option<String>,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub license_type: String,
    pub state: LicenseState,
}

/// License Engine - Manages authorization and sensitive keys
pub struct LicenseEngine {
    license: Option<License>,
    current_hwid: String,
}

impl LicenseEngine {
    /// Creates a new License Engine
    pub fn new() -> Result<Self> {
        let current_hwid = Self::get_machine_hwid()?;
        Ok(Self {
            license: None,
            current_hwid,
        })
    }

    /// Gets machine HWID (Hardware ID)
    fn get_machine_hwid() -> Result<String> {
        machine_uid::get()
            .map_err(|e| anyhow!("Failed to get machine HWID: {}", e))
    }

    /// Validates and loads license from encrypted storage
    pub fn load_license(&mut self, encrypted_data: &[u8]) -> Result<LicenseState> {
        // TODO: Decrypt license data
        // TODO: Verify signature/integrity
        // TODO: Check expiration
        // TODO: Verify HWID match
        
        Ok(LicenseState::Invalid)
    }

    /// Checks if execution is authorized
    pub fn is_authorized(&self) -> bool {
        matches!(
            self.license.as_ref().map(|l| &l.state),
            Some(LicenseState::Valid)
        )
    }

    /// Gets AI API key if license is valid
    pub fn get_ai_api_key(&self) -> Option<&str> {
        self.license
            .as_ref()
            .filter(|l| l.state == LicenseState::Valid)
            .and_then(|l| l.ai_api_key.as_deref())
    }

    /// Returns current license state
    pub fn get_state(&self) -> LicenseState {
        self.license
            .as_ref()
            .map(|l| l.state.clone())
            .unwrap_or(LicenseState::Invalid)
    }

    /// Gets current machine HWID
    pub fn get_hwid(&self) -> &str {
        &self.current_hwid
    }
}

impl Default for LicenseEngine {
    fn default() -> Self {
        Self::new().expect("Failed to initialize License Engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_engine_creation() {
        let engine = LicenseEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_unauthorized_by_default() {
        let engine = LicenseEngine::new().unwrap();
        assert!(!engine.is_authorized());
    }
}
