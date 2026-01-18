use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

/// Rate limiter state
#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    /// Create new rate limiter
    /// 
    /// # Arguments
    /// * `max_requests` - Maximum requests per window
    /// * `window_secs` - Time window in seconds
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }
    
    /// Check if request is allowed
    pub async fn check_rate_limit(&self, client_id: &str) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();
        
        // Get or create request history for this client
        let history = requests.entry(client_id.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests outside the time window
        history.retain(|&time| now.duration_since(time) < self.window);
        
        // Check if under limit
        if history.len() < self.max_requests {
            history.push(now);
            true
        } else {
            false
        }
    }
    
    /// Get remaining requests for client
    pub async fn get_remaining(&self, client_id: &str) -> usize {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();
        
        if let Some(history) = requests.get_mut(client_id) {
            history.retain(|&time| now.duration_since(time) < self.window);
            self.max_requests.saturating_sub(history.len())
        } else {
            self.max_requests
        }
    }
    
    /// Clean up old entries periodically
    pub async fn cleanup(&self) {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();
        
        requests.retain(|_, history| {
            history.retain(|&time| now.duration_since(time) < self.window);
            !history.is_empty()
        });
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        // Default: 100 requests per minute
        Self::new(100, 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_under_limit() {
        let limiter = RateLimiter::new(5, 60);
        
        for _ in 0..5 {
            assert!(limiter.check_rate_limit("client1").await);
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_over_limit() {
        let limiter = RateLimiter::new(3, 60);
        
        // First 3 should pass
        for _ in 0..3 {
            assert!(limiter.check_rate_limit("client1").await);
        }
        
        // 4th should fail
        assert!(!limiter.check_rate_limit("client1").await);
    }

    #[tokio::test]
    async fn test_rate_limiter_per_client() {
        let limiter = RateLimiter::new(2, 60);
        
        assert!(limiter.check_rate_limit("client1").await);
        assert!(limiter.check_rate_limit("client1").await);
        assert!(!limiter.check_rate_limit("client1").await); // client1 blocked
        
        assert!(limiter.check_rate_limit("client2").await); // client2 still ok
    }

    #[tokio::test]
    async fn test_get_remaining() {
        let limiter = RateLimiter::new(5, 60);
        
        assert_eq!(limiter.get_remaining("client1").await, 5);
        
        limiter.check_rate_limit("client1").await;
        assert_eq!(limiter.get_remaining("client1").await, 4);
        
        limiter.check_rate_limit("client1").await;
        assert_eq!(limiter.get_remaining("client1").await, 3);
    }
}
