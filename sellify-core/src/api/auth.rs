use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// API Key authentication middleware
pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check if this is a public endpoint
    let path = request.uri().path();
    if is_public_endpoint(path) {
        return Ok(next.run(request).await);
    }
    
    // Get API key from environment or config
    let valid_api_key = std::env::var("SELLIFY_API_KEY")
        .unwrap_or_else(|_| "dev-api-key-change-in-production".to_string());
    
    // Extract API key from header
    let api_key = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok());
    
    match api_key {
        Some(key) if key == valid_api_key => {
            // Valid API key, proceed
            Ok(next.run(request).await)
        }
        _ => {
            // Invalid or missing API key
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Check if endpoint should skip authentication
pub fn is_public_endpoint(path: &str) -> bool {
    matches!(path, "/health" | "/api/v1/health" | "/metrics")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_endpoints() {
        assert!(is_public_endpoint("/health"));
        assert!(is_public_endpoint("/api/v1/health"));
        assert!(is_public_endpoint("/metrics"));
        assert!(!is_public_endpoint("/api/v1/decision"));
    }
}
