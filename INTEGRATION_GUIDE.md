# Intégration WhatsApp API ↔ Sellify Core

Ce guide explique comment intégrer l'API WhatsApp (Node.js) avec Sellify Core (Rust).

## Architecture

```
┌─────────────────┐         HTTP REST        ┌──────────────────┐
│  WhatsApp API   │ ───────────────────────→ │  Sellify Core    │
│   (Node.js)     │ ←─────────────────────── │     (Rust)       │
│                 │                           │                  │
│ - Baileys       │   POST /api/v1/decision  │ - Decision       │
│ - Message RX/TX │   POST /api/v1/validate  │ - Validation     │
│ - Session Mgmt  │   POST /api/v1/quota     │ - Quota          │
└─────────────────┘                           └──────────────────┘
```

## 1. Démarrer Sellify Core API

```bash
cd sellify-core

# Set API key
export SELLIFY_API_KEY="your-secure-api-key-here"

# Start server
cargo run --bin sellify-server --features http-server --release
```

Serveur sur `http://localhost:3000`

## 2. Configurer WhatsApp API

Dans votre projet `whatsapp-api`, créez un client Sellify :

### `whatsapp-api/src/sellify-client.js`

```javascript
const axios = require('axios');

class SellifyClient {
  constructor(baseUrl = 'http://localhost:3000', apiKey = process.env.SELLIFY_API_KEY) {
    this.client = axios.create({
      baseURL: baseUrl,
      headers: {
        'Content-Type': 'application/json',
        'X-API-Key': apiKey
      },
      timeout: 5000
    });
  }

  /**
   * Make a decision based on incoming message
   */
  async makeDecision(context) {
    try {
      const response = await this.client.post('/api/v1/decision', {
        incoming_message: context.message,
        conversation_state: context.state || 'Discovery',
        quotas_available: await this.checkQuota(),
        is_active_hours: this.isActiveHours(),
        sentiment_detected: context.sentiment || null
      });
      
      return response.data;
    } catch (error) {
      console.error('Sellify decision error:', error.message);
      throw error;
    }
  }

  /**
   * Validate AI-generated text
   */
  async validateText(text) {
    try {
      const response = await this.client.post('/api/v1/validate', {
        text
      });
      
      return response.data;
    } catch (error) {
      console.error('Sellify validation error:', error.message);
      return { valid: false, error: error.message };
    }
  }

  /**
   * Check if message can be sent (quota)
   */
  async checkQuota(messageType = 'text') {
    try {
      const response = await this.client.post('/api/v1/quota/check', {
        message_type: messageType
      });
      
      return response.data.can_send;
    } catch (error) {
      console.error('Sellify quota check error:', error.message);
      return false;
    }
  }

  /**
   * Record that a message was sent
   */
  async recordMessage() {
    try {
      await this.client.post('/api/v1/quota/record');
    } catch (error) {
      console.error('Sellify record error:', error.message);
    }
  }

  /**
   * Transition conversation state
   */
  async transitionState(currentState, event) {
    try {
      const response = await this.client.post('/api/v1/conversation/transition', {
        current_state: currentState,
        event: event
      });
      
      return response.data.new_state;
    } catch (error) {
      console.error('Sellify transition error:', error.message);
      return currentState; // Stay in current state on error
    }
  }

  /**
   * Log audit entry
   */
  async logAudit(auditLog) {
    try {
      await this.client.post('/api/v1/audit/log', auditLog);
    } catch (error) {
      console.error('Sellify audit error:', error.message);
    }
  }

  /**
   * Check if current time is within active hours
   */
  isActiveHours() {
    const hour = new Date().getHours();
    return hour >= 9 && hour < 18; // 9AM - 6PM
  }
}

module.exports = SellifyClient;
```

## 3. Intégrer dans WhatsApp Message Handler

### `whatsapp-api/src/handlers/message.handler.js`

```javascript
const SellifyClient = require('../sellify-client');

const sellify = new SellifyClient();

async function handleIncomingMessage(message, sock) {
  const { from, body } = message;
  
  console.log(`📩 Message from ${from}: ${body}`);
  
  try {
    // 1. Make decision
    const decision = await sellify.makeDecision({
      message: body,
      state: 'Discovery', // TODO: Get from conversation DB
      sentiment: null // TODO: Add sentiment analysis
    });
    
    console.log(`🎯 Decision: ${decision.action}`);
    
    // 2. Handle decision
    switch (decision.action) {
      case 'RespondText':
        await handleRespondText(from, body, sock);
        break;
        
      case 'Ignore':
        console.log('⏭️  Ignoring message (outside hours or other reason)');
        break;
        
      case 'Delay':
        console.log(`⏳ Delaying response: ${decision.details}`);
        // TODO: Schedule delayed response
        break;
        
      case 'AlertHuman':
        console.log(`🚨 Alert human: ${decision.details}`);
        await notifyHuman(from, body, decision.details);
        break;
        
      case 'StopAutomation':
        console.log('🛑 Stopping automation for this conversation');
        // TODO: Mark conversation as manual
        break;
        
      default:
        console.warn(`⚠️  Unknown action: ${decision.action}`);
    }
    
  } catch (error) {
    console.error('❌ Error handling message:', error);
  }
}

async function handleRespondText(to, incomingMessage, sock) {
  try {
    // 1. Generate AI response (your existing AI logic)
    const aiResponse = await generateAIResponse(incomingMessage);
    
    // 2. Validate with Sellify
    const validation = await sellify.validateText(aiResponse);
    
    if (!validation.valid) {
      console.error(`❌ Validation failed: ${validation.error}`);
      // Use fallback message
      const fallbackMessage = "Désolé, je n'ai pas bien compris. Pouvez-vous préciser ?";
      await sendMessage(to, fallbackMessage, sock);
      return;
    }
    
    // 3. Check delay
    const quotaCheck = await sellify.client.post('/api/v1/quota/check', {
      message_type: 'text'
    });
    
    const delaySeconds = quotaCheck.data.delay_seconds || 0;
    
    // 4. Wait for anti-ban delay
    if (delaySeconds > 0) {
      console.log(`⏰ Waiting ${delaySeconds}s before sending...`);
      await new Promise(resolve => setTimeout(resolve, delaySeconds * 1000));
    }
    
    // 5. Send message
    await sendMessage(to, validation.validated_text, sock);
    
    // 6. Record message sent
    await sellify.recordMessage();
    
    // 7. Log audit
    await sellify.logAudit({
      id: generateUuid(),
      timestamp: new Date().toISOString(),
      conversation_id: to,
      incoming_message: incomingMessage,
      state: 'Discovery',
      chosen_action: 'RespondText',
      ai_prompt: 'Generated by OpenAI/Anthropic',
      ai_response: aiResponse,
      sent_message: validation.validated_text,
      quotas_before: { messages_today: 10, messages_this_week: 50 },
      quotas_after: { messages_today: 11, messages_this_week: 51 }
    });
    
    console.log('✅ Message sent successfully');
    
  } catch (error) {
    console.error('❌ Error in handleRespondText:', error);
  }
}

async function sendMessage(to, text, sock) {
  await sock.sendMessage(to, { text });
}

async function generateAIResponse(message) {
  // Your existing AI generation logic
  // Could be OpenAI, Anthropic, etc.
  return `Réponse IA pour: ${message}`;
}

async function notifyHuman(phone, message, reason) {
  // Send WhatsApp notification to alert number
  const alertNumber = process.env.ALERT_WHATSAPP_NUMBER;
  if (alertNumber) {
    const alertMessage = `🚨 ALERTE\n\nClient: ${phone}\nMessage: ${message}\nRaison: ${reason}`;
    // Send via WhatsApp to alert number
  }
}

function generateUuid() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, c => {
    const r = Math.random() * 16 | 0;
    const v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

module.exports = { handleIncomingMessage };
```

## 4. Configuration Environnement

### `.env` dans `whatsapp-api/`

```bash
# Sellify Core API
SELLIFY_API_URL=http://localhost:3000
SELLIFY_API_KEY=your-secure-api-key-here

# Alert numbers
ALERT_WHATSAPP_NUMBER=+33612345678
```

## 5. Exemple Complet d'Utilisation

### Démarrage

```bash
# Terminal 1 - Sellify Core
cd sellify-core
export SELLIFY_API_KEY="my-secret-key"
cargo run --bin sellify-server --features http-server --release

# Terminal 2 - WhatsApp API
cd whatsapp-api
export SELLIFY_API_KEY="my-secret-key"
npm run start:prod
```

### Flux Complet

1. **Message WhatsApp reçu** → WhatsApp API
2. **Decision** → Sellify Core `/api/v1/decision`
3. **Génération IA** → Votre provider (OpenAI, etc.)
4. **Validation** → Sellify Core `/api/v1/validate`
5. **Check quota + delay** → Sellify Core `/api/v1/quota/check`
6. **Wait delay** → Anti-ban
7. **Send message** → WhatsApp
8. **Record quota** → Sellify Core `/api/v1/quota/record`
9. **Audit log** → Sellify Core `/api/v1/audit/log`

## 6. Tests d'Intégration

### `whatsapp-api/tests/sellify.integration.test.js`

```javascript
const SellifyClient = require('../src/sellify-client');

describe('Sellify Integration', () => {
  let sellify;
  
  beforeAll(() => {
    sellify = new SellifyClient('http://localhost:3000', 'test-api-key');
  });
  
  it('should make decision', async () => {
    const decision = await sellify.makeDecision({
      message: 'Bonjour',
      state: 'Discovery'
    });
    
    expect(decision.action).toBeDefined();
  });
  
  it('should validate text', async () => {
    const validation = await sellify.validateText('Bonjour, comment puis-je vous aider ?');
    
    expect(validation.valid).toBe(true);
  });
  
  it('should reject forbidden words', async () => {
    const validation = await sellify.validateText('Je suis une AI');
    
    expect(validation.valid).toBe(false);
  });
});
```

## 7. Monitoring

Ajoutez des logs pour surveiller l'intégration :

```javascript
console.log('📊 Sellify Stats:');
console.log('  - Decisions made:', decisionsCount);
console.log('  - Messages validated:', validatedCount);
console.log('  - Messages rejected:', rejectedCount);
console.log('  - Current quota:', await sellify.checkQuota());
```

## 8. Gestion d'Erreurs

```javascript
try {
  const decision = await sellify.makeDecision(context);
} catch (error) {
  if (error.response?.status === 401) {
    console.error('❌ Invalid API key');
  } else if (error.response?.status === 429) {
    console.error('❌ Rate limit exceeded');
  } else {
    console.error('❌ Sellify error:', error.message);
    // Fallback: use default safe behavior
    decision = { action: 'Ignore' };
  }
}
```

## 9. Production Checklist

- [ ] API key sécurisée (env variable, jamais en dur)
- [ ] HTTPS entre WhatsApp API et Sellify Core
- [ ] Retry logic avec backoff exponentiel
- [ ] Timeout approprié (5s recommandé)
- [ ] Logs structurés (JSON)
- [ ] Monitoring erreurs (Sentry, etc.)
- [ ] Health check périodique de Sellify Core

---

## Troubleshooting

### Sellify Core ne répond pas

```bash
# Check if running
curl http://localhost:3000/health

# Check logs
RUST_LOG=debug cargo run --bin sellify-server --features http-server
```

### Authentication errors

```bash
# Verify API key
echo $SELLIFY_API_KEY

# Test with curl
curl -H "X-API-Key: your-key" http://localhost:3000/api/v1/decision
```

### Rate limiting issues

Augmentez les limites dans Sellify Core ou implémentez queue côté WhatsApp API.

---

Vous êtes maintenant prêt à intégrer Sellify Core avec votre API WhatsApp ! 🚀
