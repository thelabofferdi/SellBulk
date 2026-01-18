use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Product structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub price: f64,
    pub keywords: Vec<String>,
    pub objections: Vec<Objection>,
    pub media: Vec<Media>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objection {
    pub trigger: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    pub media_type: MediaType,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
}

/// Knowledge Base Engine - Defines the "authorized universe" for AI
pub struct KnowledgeBaseEngine {
    products: Vec<Product>,
}

impl KnowledgeBaseEngine {
    pub fn new() -> Self {
        Self {
            products: vec![],
        }
    }

    /// Load products from storage
    pub fn load_products(&mut self, _products: Vec<Product>) -> Result<()> {
        // TODO: Load from encrypted storage
        Ok(())
    }

    /// Get product by ID
    pub fn get_product(&self, id: &str) -> Option<&Product> {
        self.products.iter().find(|p| p.id == id)
    }

    /// Validate that product exists (no hallucination allowed)
    pub fn is_valid_product(&self, id: &str) -> bool {
        self.get_product(id).is_some()
    }
}

impl Default for KnowledgeBaseEngine {
    fn default() -> Self {
        Self::new()
    }
}
