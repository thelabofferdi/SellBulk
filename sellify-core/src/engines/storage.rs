use anyhow::{Result, anyhow};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    Aes256Gcm, Nonce, Key
};
use sha2::{Sha256, Digest};

/// Storage Engine - Local encrypted and transactional storage
pub struct StorageEngine {
    db_path: PathBuf,
    conn: Option<Connection>,
    encryption_key: Option<Key<Aes256Gcm>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub db_path: PathBuf,
    pub encryption_key: Vec<u8>,
}

impl StorageEngine {
    /// Creates a new Storage Engine
    pub fn new(db_path: PathBuf) -> Result<Self> {
        Ok(Self {
            db_path,
            conn: None,
            encryption_key: None,
        })
    }
    
    /// Creates Storage Engine with encryption key
    pub fn new_with_key(db_path: PathBuf, key: &[u8]) -> Result<Self> {
        let encryption_key = Self::derive_key_from_bytes(key)?;
        Ok(Self {
            db_path,
            conn: None,
            encryption_key: Some(encryption_key),
        })
    }
    
    /// Derives AES-256 key from arbitrary bytes using SHA-256
    fn derive_key_from_bytes(input: &[u8]) -> Result<Key<Aes256Gcm>> {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let hash = hasher.finalize();
        Ok(*Key::<Aes256Gcm>::from_slice(&hash))
    }

    /// Initializes the database and creates tables
    pub fn initialize(&mut self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        // Create tables for encrypted data storage
        conn.execute(
            "CREATE TABLE IF NOT EXISTS encrypted_data (
                key TEXT PRIMARY KEY,
                nonce BLOB NOT NULL,
                ciphertext BLOB NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        // Create tables for conversations
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                phone_number TEXT NOT NULL,
                state TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                content TEXT NOT NULL,
                direction TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                FOREIGN KEY(conversation_id) REFERENCES conversations(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                id TEXT PRIMARY KEY,
                conversation_id TEXT,
                event_type TEXT NOT NULL,
                data TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        self.conn = Some(conn);
        Ok(())
    }

    /// Stores data with encryption (atomic operation)
    pub fn store(&self, key: &str, value: &[u8]) -> Result<()> {
        let encryption_key = self.encryption_key.as_ref()
            .ok_or_else(|| anyhow!("Encryption key not set"))?;
        
        let cipher = Aes256Gcm::new(encryption_key);
        
        // Generate random nonce (12 bytes for AES-GCM)
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        // Encrypt data
        let ciphertext = cipher.encrypt(&nonce, value)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        // Store in database
        let conn = self.conn.as_ref()
            .ok_or_else(|| anyhow!("Database not initialized"))?;
        
        let timestamp = chrono::Utc::now().timestamp();
        
        conn.execute(
            "INSERT OR REPLACE INTO encrypted_data (key, nonce, ciphertext, created_at) 
             VALUES (?1, ?2, ?3, ?4)",
            (key, nonce.as_slice(), &ciphertext, timestamp),
        )?;
        
        Ok(())
    }

    /// Retrieves and decrypts data
    pub fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let encryption_key = self.encryption_key.as_ref()
            .ok_or_else(|| anyhow!("Encryption key not set"))?;
        
        let conn = self.conn.as_ref()
            .ok_or_else(|| anyhow!("Database not initialized"))?;
        
        let mut stmt = conn.prepare(
            "SELECT nonce, ciphertext FROM encrypted_data WHERE key = ?1"
        )?;
        
        let result = stmt.query_row([key], |row| {
            let nonce_bytes: Vec<u8> = row.get(0)?;
            let ciphertext: Vec<u8> = row.get(1)?;
            Ok((nonce_bytes, ciphertext))
        });
        
        match result {
            Ok((nonce_bytes, ciphertext)) => {
                let cipher = Aes256Gcm::new(encryption_key);
                let nonce = Nonce::from_slice(&nonce_bytes);
                
                let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
                    .map_err(|e| anyhow!("Decryption failed: {}", e))?;
                
                Ok(Some(plaintext))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!("Database error: {}", e)),
        }
    }

    /// Checks database integrity
    pub fn check_integrity(&self) -> Result<bool> {
        let conn = self.conn.as_ref()
            .ok_or_else(|| anyhow!("Database not initialized"))?;
        
        let result: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
        Ok(result == "ok")
    }
    
    /// Deletes encrypted data by key
    pub fn delete(&self, key: &str) -> Result<bool> {
        let conn = self.conn.as_ref()
            .ok_or_else(|| anyhow!("Database not initialized"))?;
        
        let rows_affected = conn.execute(
            "DELETE FROM encrypted_data WHERE key = ?1",
            [key],
        )?;
        
        Ok(rows_affected > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_storage_engine_creation() {
        let db_path = temp_dir().join("test_sellify.db");
        let mut engine = StorageEngine::new(db_path).unwrap();
        assert!(engine.initialize().is_ok());
    }
    
    #[test]
    fn test_encryption_and_decryption() {
        let db_path = temp_dir().join("test_encrypted_sellify.db");
        let key = b"test-master-key-32-bytes-long!!";
        let mut engine = StorageEngine::new_with_key(db_path, key).unwrap();
        engine.initialize().unwrap();
        
        // Store encrypted data
        let data = b"Secret sensitive data";
        engine.store("test_key", data).unwrap();
        
        // Retrieve and decrypt
        let retrieved = engine.retrieve("test_key").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), data);
    }
    
    #[test]
    fn test_retrieve_nonexistent_key() {
        let db_path = temp_dir().join("test_nonexistent_sellify.db");
        let key = b"test-master-key-32-bytes-long!!";
        let mut engine = StorageEngine::new_with_key(db_path, key).unwrap();
        engine.initialize().unwrap();
        
        let result = engine.retrieve("nonexistent").unwrap();
        assert!(result.is_none());
    }
    
    #[test]
    fn test_delete_data() {
        let db_path = temp_dir().join("test_delete_sellify.db");
        let key = b"test-master-key-32-bytes-long!!";
        let mut engine = StorageEngine::new_with_key(db_path, key).unwrap();
        engine.initialize().unwrap();
        
        // Store data
        engine.store("to_delete", b"temporary data").unwrap();
        
        // Delete it
        let deleted = engine.delete("to_delete").unwrap();
        assert!(deleted);
        
        // Verify it's gone
        let retrieved = engine.retrieve("to_delete").unwrap();
        assert!(retrieved.is_none());
    }
    
    #[test]
    fn test_check_integrity() {
        let db_path = temp_dir().join("test_integrity_sellify.db");
        let mut engine = StorageEngine::new(db_path).unwrap();
        engine.initialize().unwrap();
        
        assert!(engine.check_integrity().unwrap());
    }
}
