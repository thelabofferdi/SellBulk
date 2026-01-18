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
    pub fn load_products(&mut self, products: Vec<Product>) -> Result<()> {
        self.products = products;
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
    
    /// Get all products
    pub fn get_all_products(&self) -> &[Product] {
        &self.products
    }
    
    /// Search products by keyword
    pub fn search_by_keyword(&self, keyword: &str) -> Vec<&Product> {
        let keyword_lower = keyword.to_lowercase();
        self.products.iter()
            .filter(|p| p.keywords.iter().any(|k| k.to_lowercase().contains(&keyword_lower)))
            .collect()
    }
}

impl Default for KnowledgeBaseEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_product() -> Product {
        Product {
            id: "prod-001".to_string(),
            name: "Test Product".to_string(),
            short_description: "A test product".to_string(),
            long_description: "A longer description".to_string(),
            price: 99.99,
            keywords: vec!["test".to_string(), "product".to_string()],
            objections: vec![],
            media: vec![],
        }
    }

    #[test]
    fn test_load_products() {
        let mut kb = KnowledgeBaseEngine::new();
        let products = vec![create_test_product()];
        assert!(kb.load_products(products).is_ok());
        assert_eq!(kb.get_all_products().len(), 1);
    }

    #[test]
    fn test_get_product_by_id() {
        let mut kb = KnowledgeBaseEngine::new();
        kb.load_products(vec![create_test_product()]).unwrap();
        
        assert!(kb.get_product("prod-001").is_some());
        assert!(kb.get_product("prod-999").is_none());
    }

    #[test]
    fn test_is_valid_product() {
        let mut kb = KnowledgeBaseEngine::new();
        kb.load_products(vec![create_test_product()]).unwrap();
        
        assert!(kb.is_valid_product("prod-001"));
        assert!(!kb.is_valid_product("invalid"));
    }

    #[test]
    fn test_search_by_keyword() {
        let mut kb = KnowledgeBaseEngine::new();
        kb.load_products(vec![create_test_product()]).unwrap();
        
        let results = kb.search_by_keyword("test");
        assert_eq!(results.len(), 1);
        
        let results = kb.search_by_keyword("nonexistent");
        assert_eq!(results.len(), 0);
    }
}
