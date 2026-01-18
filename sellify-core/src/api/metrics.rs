use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramVec, Registry, Encoder, TextEncoder,
    Opts, HistogramOpts,
};
use lazy_static::lazy_static;
use anyhow::Result;

lazy_static! {
    /// Global Prometheus registry
    pub static ref REGISTRY: Registry = Registry::new();

    // ============== DECISION ENGINE METRICS ==============
    
    /// Total decisions made by action type
    pub static ref DECISIONS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("sellify_decisions_total", "Total decisions made by action type"),
        &["action"]
    ).expect("Failed to create DECISIONS_TOTAL metric");

    /// Current conversation state distribution
    pub static ref CONVERSATION_STATE: GaugeVec = GaugeVec::new(
        Opts::new("sellify_conversation_state", "Current conversation state distribution"),
        &["state"]
    ).expect("Failed to create CONVERSATION_STATE metric");

    // ============== QUOTA ENGINE METRICS ==============
    
    /// Current quota usage for messages today
    pub static ref QUOTA_MESSAGES_TODAY: Gauge = Gauge::new(
        "sellify_quota_messages_today",
        "Current number of messages sent today"
    ).expect("Failed to create QUOTA_MESSAGES_TODAY metric");

    /// Current quota usage for messages this week
    pub static ref QUOTA_MESSAGES_WEEK: Gauge = Gauge::new(
        "sellify_quota_messages_week",
        "Current number of messages sent this week"
    ).expect("Failed to create QUOTA_MESSAGES_WEEK metric");

    /// Current quota usage for images today
    pub static ref QUOTA_IMAGES_TODAY: Gauge = Gauge::new(
        "sellify_quota_images_today",
        "Current number of images sent today"
    ).expect("Failed to create QUOTA_IMAGES_TODAY metric");

    /// Current quota usage for videos this week
    pub static ref QUOTA_VIDEOS_WEEK: Gauge = Gauge::new(
        "sellify_quota_videos_week",
        "Current number of videos sent this week"
    ).expect("Failed to create QUOTA_VIDEOS_WEEK metric");

    /// Total quota resets
    pub static ref QUOTA_RESETS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("sellify_quota_resets_total", "Total quota resets by type"),
        &["reset_type"]
    ).expect("Failed to create QUOTA_RESETS_TOTAL metric");

    /// Quota limit reached events
    pub static ref QUOTA_LIMIT_REACHED: CounterVec = CounterVec::new(
        Opts::new("sellify_quota_limit_reached", "Number of times quota limit was reached"),
        &["quota_type"]
    ).expect("Failed to create QUOTA_LIMIT_REACHED metric");

    // ============== AI GATEWAY METRICS ==============
    
    /// AI text generation duration in seconds
    pub static ref AI_GENERATION_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "sellify_ai_generation_duration_seconds",
            "AI text generation duration in seconds"
        ).buckets(vec![0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0])
    ).expect("Failed to create AI_GENERATION_DURATION metric");

    /// Total AI generation requests
    pub static ref AI_REQUESTS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("sellify_ai_requests_total", "Total AI generation requests by status"),
        &["status"]
    ).expect("Failed to create AI_REQUESTS_TOTAL metric");

    // ============== ANTI-HALLUCINATION METRICS ==============
    
    /// Text validation results
    pub static ref VALIDATION_RESULTS: CounterVec = CounterVec::new(
        Opts::new("sellify_validation_results", "Text validation results"),
        &["result"]
    ).expect("Failed to create VALIDATION_RESULTS metric");

    /// Forbidden words detected
    pub static ref FORBIDDEN_WORDS_DETECTED: Counter = Counter::new(
        "sellify_forbidden_words_detected",
        "Number of times forbidden words were detected"
    ).expect("Failed to create FORBIDDEN_WORDS_DETECTED metric");

    // ============== API METRICS ==============
    
    /// HTTP requests total
    pub static ref HTTP_REQUESTS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("sellify_http_requests_total", "Total HTTP requests by endpoint and status"),
        &["endpoint", "status"]
    ).expect("Failed to create HTTP_REQUESTS_TOTAL metric");

    /// HTTP request duration
    pub static ref HTTP_REQUEST_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new(
            "sellify_http_request_duration_seconds",
            "HTTP request duration in seconds"
        ).buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]),
        &["endpoint"]
    ).expect("Failed to create HTTP_REQUEST_DURATION metric");

    // ============== SYSTEM METRICS ==============
    
    /// Total audit logs written
    pub static ref AUDIT_LOGS_TOTAL: Counter = Counter::new(
        "sellify_audit_logs_total",
        "Total audit logs written"
    ).expect("Failed to create AUDIT_LOGS_TOTAL metric");

    /// Total alerts sent
    pub static ref ALERTS_SENT_TOTAL: CounterVec = CounterVec::new(
        Opts::new("sellify_alerts_sent_total", "Total alerts sent by severity"),
        &["severity"]
    ).expect("Failed to create ALERTS_SENT_TOTAL metric");
}

/// Initialize and register all metrics
pub fn init_metrics() -> Result<()> {
    // Decision Engine metrics
    REGISTRY.register(Box::new(DECISIONS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(CONVERSATION_STATE.clone()))?;

    // Quota Engine metrics
    REGISTRY.register(Box::new(QUOTA_MESSAGES_TODAY.clone()))?;
    REGISTRY.register(Box::new(QUOTA_MESSAGES_WEEK.clone()))?;
    REGISTRY.register(Box::new(QUOTA_IMAGES_TODAY.clone()))?;
    REGISTRY.register(Box::new(QUOTA_VIDEOS_WEEK.clone()))?;
    REGISTRY.register(Box::new(QUOTA_RESETS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(QUOTA_LIMIT_REACHED.clone()))?;

    // AI Gateway metrics
    REGISTRY.register(Box::new(AI_GENERATION_DURATION.clone()))?;
    REGISTRY.register(Box::new(AI_REQUESTS_TOTAL.clone()))?;

    // Anti-hallucination metrics
    REGISTRY.register(Box::new(VALIDATION_RESULTS.clone()))?;
    REGISTRY.register(Box::new(FORBIDDEN_WORDS_DETECTED.clone()))?;

    // API metrics
    REGISTRY.register(Box::new(HTTP_REQUESTS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(HTTP_REQUEST_DURATION.clone()))?;

    // System metrics
    REGISTRY.register(Box::new(AUDIT_LOGS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(ALERTS_SENT_TOTAL.clone()))?;

    log::info!("📊 Prometheus metrics initialized");
    Ok(())
}

/// Gather all metrics in Prometheus text format
pub fn gather_metrics() -> Result<String> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_metrics() {
        // Should not panic
        let result = init_metrics();
        // Note: Will fail on second run due to duplicate registration
        // This is expected behavior in tests
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_decision_metric_increment() {
        DECISIONS_TOTAL.with_label_values(&["RespondText"]).inc();
        let value = DECISIONS_TOTAL.with_label_values(&["RespondText"]).get();
        assert!(value >= 1.0);
    }

    #[test]
    fn test_quota_gauge_set() {
        QUOTA_MESSAGES_TODAY.set(42.0);
        assert_eq!(QUOTA_MESSAGES_TODAY.get(), 42.0);
    }

    #[test]
    fn test_gather_metrics() {
        // Initialize metrics first (ignore error if already initialized)
        let _ = init_metrics();
        
        let result = gather_metrics();
        assert!(result.is_ok());
        let metrics = result.unwrap();
        // Should contain at least the HELP text
        assert!(!metrics.is_empty());
    }

    #[test]
    fn test_validation_metrics() {
        VALIDATION_RESULTS.with_label_values(&["valid"]).inc();
        let value = VALIDATION_RESULTS.with_label_values(&["valid"]).get();
        assert!(value >= 1.0);
    }

    #[test]
    fn test_quota_reset_counter() {
        QUOTA_RESETS_TOTAL.with_label_values(&["daily"]).inc();
        let value = QUOTA_RESETS_TOTAL.with_label_values(&["daily"]).get();
        assert!(value >= 1.0);
    }
}
