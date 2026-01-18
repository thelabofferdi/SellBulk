# Sellify Core API Documentation

## Base URL

```
http://localhost:3000
```

## Authentication

All API endpoints (except `/health`) require API key authentication.

**Header**: `X-API-Key`

**Default API Key**: `dev-api-key-change-in-production`

Set custom API key via environment variable:
```bash
export SELLIFY_API_KEY="your-secure-api-key"
```

**Example**:
```bash
curl -H "X-API-Key: your-api-key" http://localhost:3000/api/v1/decision
```

---

## Endpoints

### Health Check

**GET** `/health`

Check if API is running.

**Response** (200 OK):
```json
{
  "status": "healthy",
  "service": "sellify-core",
  "version": "0.1.0"
}
```

---

### Prometheus Metrics

**GET** `/metrics`

Get Prometheus-format metrics for monitoring.

**Public endpoint** (no authentication required)

**Response** (200 OK - Text):
```
# HELP sellify_decisions_total Total decisions made by action type
# TYPE sellify_decisions_total counter
sellify_decisions_total{action="RespondText"} 42
sellify_decisions_total{action="Ignore"} 8

# HELP sellify_quota_messages_today Current number of messages sent today
# TYPE sellify_quota_messages_today gauge
sellify_quota_messages_today 45

# HELP sellify_quota_resets_total Total quota resets by type
# TYPE sellify_quota_resets_total counter
sellify_quota_resets_total{reset_type="daily"} 5
sellify_quota_resets_total{reset_type="weekly"} 1

... (more metrics)
```

**Available Metrics**:

| Metric | Type | Description |
|--------|------|-------------|
| `sellify_decisions_total{action}` | Counter | Total decisions by action type |
| `sellify_conversation_state{state}` | Gauge | Current conversation state distribution |
| `sellify_quota_messages_today` | Gauge | Messages sent today |
| `sellify_quota_messages_week` | Gauge | Messages sent this week |
| `sellify_quota_images_today` | Gauge | Images sent today |
| `sellify_quota_videos_week` | Gauge | Videos sent this week |
| `sellify_quota_resets_total{reset_type}` | Counter | Total quota resets (daily/weekly) |
| `sellify_quota_limit_reached{quota_type}` | Counter | Times quota limit was reached |
| `sellify_ai_generation_duration_seconds` | Histogram | AI text generation duration |
| `sellify_ai_requests_total{status}` | Counter | AI requests by status |
| `sellify_validation_results{result}` | Counter | Text validation results (valid/invalid) |
| `sellify_forbidden_words_detected` | Counter | Forbidden words detected count |
| `sellify_http_requests_total{endpoint,status}` | Counter | HTTP requests by endpoint/status |
| `sellify_http_request_duration_seconds{endpoint}` | Histogram | HTTP request duration |
| `sellify_audit_logs_total` | Counter | Total audit logs written |
| `sellify_alerts_sent_total{severity}` | Counter | Alerts sent by severity |

---

### Decision Engine

**POST** `/api/v1/decision`

Make a decision based on context.

**Request Body**:
```json
{
  "incoming_message": "Je voudrais des informations",
  "conversation_state": "Discovery",
  "quotas_available": true,
  "is_active_hours": true,
  "sentiment_detected": null
}
```

**Response** (200 OK):
```json
{
  "action": "RespondText",
  "details": null
}
```

**Possible Actions**:
- `RespondText`
- `RespondWithMedia`
- `Ignore`
- `Delay`
- `AlertHuman`
- `StopAutomation`

---

### Text Validation

**POST** `/api/v1/validate`

Validate AI-generated text (anti-hallucination).

**Request Body**:
```json
{
  "text": "Bonjour, comment puis-je vous aider ?"
}
```

**Response** (200 OK - Valid):
```json
{
  "valid": true,
  "validated_text": "Bonjour, comment puis-je vous aider ?",
  "error": null
}
```

**Response** (200 OK - Invalid):
```json
{
  "valid": false,
  "validated_text": null,
  "error": "Generated text contains forbidden word: ai"
}
```

---

### Quota Management

#### Check Quota

**POST** `/api/v1/quota/check`

Check if message can be sent.

**Request Body**:
```json
{
  "message_type": "text"
}
```

Types: `"text"`, `"image"`, `"video"`

**Response** (200 OK):
```json
{
  "can_send": true,
  "delay_seconds": 5
}
```

#### Record Message

**POST** `/api/v1/quota/record`

Record that a message was sent.

**Response**: 200 OK (empty body)

#### Get Quota Status

**GET** `/api/v1/quota/status`

Get current quota usage and reset status.

**Response** (200 OK):
```json
{
  "messages_today": 45,
  "messages_this_week": 234,
  "images_today": 12,
  "videos_this_week": 3,
  "last_reset": "2026-01-18T00:00:00Z",
  "needs_daily_reset": false,
  "needs_weekly_reset": false
}
```

#### Reset Daily Quotas

**POST** `/api/v1/quota/reset/daily`

Manually reset daily quotas (messages_today, images_today).

**Response** (200 OK):
```json
{
  "reset_type": "daily",
  "timestamp": "2026-01-18T14:30:00Z"
}
```

#### Reset Weekly Quotas

**POST** `/api/v1/quota/reset/weekly`

Manually reset weekly quotas (messages_this_week, videos_this_week).

**Response** (200 OK):
```json
{
  "reset_type": "weekly",
  "timestamp": "2026-01-18T14:30:00Z"
}
```

**Note**: Quotas are automatically reset:
- **Daily**: Every day at 00:00 UTC
- **Weekly**: Every Monday at 00:00 UTC

---

### Conversation State Machine

**POST** `/api/v1/conversation/transition`

Transition conversation state.

**Request Body**:
```json
{
  "current_state": "Discovery",
  "event": "ProductQuestion"
}
```

**States**:
- `Discovery`
- `Interest`
- `Intent`
- `Objection`
- `Negative`
- `Escalated`
- `Frozen`

**Events**:
- `ProductQuestion`
- `PriceInterest`
- `PurchaseIntent`
- `ObjectionRaised`
- `NegativeResponse`
- `ThreatDetected`
- `Freeze`

**Response** (200 OK):
```json
{
  "new_state": "Interest"
}
```

---

### Knowledge Base

#### List Products

**GET** `/api/v1/products`

Get all products.

**Response** (200 OK):
```json
[
  {
    "id": "prod-001",
    "name": "Product Name",
    "short_description": "...",
    "long_description": "...",
    "price": 99.99,
    "keywords": ["keyword1", "keyword2"],
    "objections": [],
    "media": []
  }
]
```

#### Get Product

**GET** `/api/v1/products/:id`

Get product by ID.

**Response** (200 OK):
```json
{
  "id": "prod-001",
  "name": "Product Name",
  ...
}
```

**Response** (404 Not Found):
```json
"Product not found"
```

---

### Audit Logging

**POST** `/api/v1/audit/log`

Log an audit entry.

**Request Body**:
```json
{
  "id": "log-001",
  "timestamp": "2026-01-18T14:00:00Z",
  "conversation_id": "conv-001",
  "incoming_message": "...",
  "state": "Interest",
  "chosen_action": "RespondText",
  "ai_prompt": "...",
  "ai_response": "...",
  "sent_message": "...",
  "quotas_before": {
    "messages_today": 10,
    "messages_this_week": 50
  },
  "quotas_after": {
    "messages_today": 11,
    "messages_this_week": 51
  }
}
```

**Response**: 201 Created

---

## Example Usage

### cURL

```bash
# Health check
curl http://localhost:3000/health

# Make decision (with API key)
curl -X POST http://localhost:3000/api/v1/decision \
  -H "Content-Type: application/json" \
  -H "X-API-Key: dev-api-key-change-in-production" \
  -d '{
    "incoming_message": "Bonjour",
    "conversation_state": "Discovery",
    "quotas_available": true,
    "is_active_hours": true,
    "sentiment_detected": null
  }'

# Validate text
curl -X POST http://localhost:3000/api/v1/validate \
  -H "Content-Type: application/json" \
  -H "X-API-Key: dev-api-key-change-in-production" \
  -d '{
    "text": "Bonjour, comment puis-je vous aider ?"
  }'

# Check quota
curl -X POST http://localhost:3000/api/v1/quota/check \
  -H "Content-Type: application/json" \
  -H "X-API-Key: dev-api-key-change-in-production" \
  -d '{
    "message_type": "text"
  }'

# Get quota status
curl http://localhost:3000/api/v1/quota/status \
  -H "X-API-Key: dev-api-key-change-in-production"

# Reset daily quotas
curl -X POST http://localhost:3000/api/v1/quota/reset/daily \
  -H "X-API-Key: dev-api-key-change-in-production"

# Reset weekly quotas
curl -X POST http://localhost:3000/api/v1/quota/reset/weekly \
  -H "X-API-Key: dev-api-key-change-in-production"
```

### JavaScript/Node.js

```javascript
// Make decision
const response = await fetch('http://localhost:3000/api/v1/decision', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'X-API-Key': 'dev-api-key-change-in-production'
  },
  body: JSON.stringify({
    incoming_message: "Bonjour",
    conversation_state: "Discovery",
    quotas_available: true,
    is_active_hours: true,
    sentiment_detected: null
  })
});

const data = await response.json();
console.log(data.action); // "RespondText"

// Get quota status
const quotaResponse = await fetch('http://localhost:3000/api/v1/quota/status', {
  headers: {
    'X-API-Key': 'dev-api-key-change-in-production'
  }
});

const quotaData = await quotaResponse.json();
console.log(quotaData.messages_today); // e.g., 45
```

### Python

```python
import requests

# Set API key
headers = {
    'X-API-Key': 'dev-api-key-change-in-production'
}

# Make decision
response = requests.post('http://localhost:3000/api/v1/decision', 
    headers=headers,
    json={
        "incoming_message": "Bonjour",
        "conversation_state": "Discovery",
        "quotas_available": True,
        "is_active_hours": True,
        "sentiment_detected": None
    }
)

print(response.json())

# Get quota status
quota_response = requests.get('http://localhost:3000/api/v1/quota/status', 
    headers=headers
)

print(quota_response.json())
```

---

## Running the Server

```bash
# With default features
cargo run --bin sellify-server --features http-server

# With custom API key
SELLIFY_API_KEY="my-secure-key" cargo run --bin sellify-server --features http-server

# With release optimization and logging
RUST_LOG=info SELLIFY_API_KEY="my-secure-key" \
  cargo run --bin sellify-server --features http-server --release
```

**Note**: The server automatically starts the quota reset scheduler:
- Daily reset at 00:00 UTC
- Weekly reset on Monday at 00:00 UTC

## CORS

CORS is enabled for all origins (development mode).

For production, configure specific origins in `src/api/server.rs`.

---

## Error Responses

All errors return appropriate HTTP status codes:

- `400 Bad Request` - Invalid input
- `401 Unauthorized` - Missing or invalid API key
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

Error body format:
```
"Error message string"
```

---

## Next Steps

- [x] Add API key authentication ✅
- [x] Add rate limiting ✅
- [x] Add automatic quota resets ✅
- [x] Add metrics endpoint (`/metrics` for Prometheus) ✅
- [ ] Add request/response validation middleware
- [ ] Add OpenAPI/Swagger documentation

## Monitoring with Prometheus

The `/metrics` endpoint is compatible with Prometheus. Example scrape config:

```yaml
scrape_configs:
  - job_name: 'sellify'
    static_configs:
      - targets: ['localhost:3000']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

**Grafana Dashboard Suggestions**:
- **Decision Actions**: Panel showing `sellify_decisions_total` by action type (pie chart)
- **Quota Usage**: Panel showing `sellify_quota_messages_today` vs limit (gauge)
- **Validation Rate**: Panel showing `sellify_validation_results{result="valid"}` rate
- **API Response Time**: Panel showing `sellify_http_request_duration_seconds` (histogram)
- **Reset Events**: Panel showing `sellify_quota_resets_total` over time (graph)
