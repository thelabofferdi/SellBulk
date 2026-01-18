# Architecture Sellify Core

## Vue d'ensemble

Sellify Core est construit sur un principe fondamental : **l'IA ne décide jamais**.

```
┌─────────────────────────────────────────────────────────────┐
│                    SELLIFY CORE                              │
│                   (Rust Backend)                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   License    │  │   Storage    │  │    Config    │     │
│  │   Engine     │  │   Engine     │  │    Engine    │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
│         │                  │                  │              │
│         └──────────────────┼──────────────────┘              │
│                            │                                 │
│                   ┌────────┴────────┐                       │
│                   │                 │                        │
│         ┌─────────▼──────┐   ┌─────▼──────────┐           │
│         │ Knowledge Base │   │  Conversation  │            │
│         │    Engine      │   │     Engine     │            │
│         └────────┬───────┘   └────────┬───────┘            │
│                  │                     │                     │
│                  └──────┬──────────────┘                     │
│                         │                                    │
│                  ┌──────▼──────────┐                        │
│                  │   DECISION      │ ⭐ CŒUR               │
│                  │    ENGINE       │                         │
│                  └──────┬──────────┘                        │
│                         │                                    │
│      ┌──────────────────┼──────────────────┐               │
│      │                  │                  │                │
│  ┌───▼───┐      ┌───────▼────────┐   ┌────▼─────┐         │
│  │ Quota │      │  IA Gateway    │   │  Alert   │         │
│  │Engine │      │  (Texte only)  │   │  Engine  │         │
│  └───┬───┘      └───────┬────────┘   └────┬─────┘         │
│      │                  │                  │                │
│      │          ┌───────▼─────────┐        │                │
│      │          │ Anti-Hallucin.  │        │                │
│      │          │     Engine      │        │                │
│      │          └───────┬─────────┘        │                │
│      │                  │                  │                │
│      └──────────────────┼──────────────────┘                │
│                         │                                    │
│                  ┌──────▼──────┐                            │
│                  │    Audit    │                             │
│                  │   Engine    │                             │
│                  └─────────────┘                             │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Flux de Traitement d'un Message

### 1. Réception Message WhatsApp

```
WhatsApp → WhatsApp API (Node.js) → Sellify Core
```

### 2. Contexte de Décision

```rust
let context = DecisionContext {
    incoming_message: "Je voudrais le prix du produit X",
    conversation_state: "Interest",      // ← Conversation Engine
    quotas_available: true,              // ← Quota Engine
    is_active_hours: true,               // ← Config Engine
    sentiment_detected: None,            // ← Analyse basique
};
```

### 3. Decision Engine (Règles Déterministes)

```rust
fn decide(context: DecisionContext) -> Action {
    // Règle 1: Horaires
    if !context.is_active_hours {
        return Action::Ignore;
    }
    
    // Règle 2: Quotas
    if !context.quotas_available {
        return Action::Delay { seconds: 3600 };
    }
    
    // Règle 3: Sentiment négatif
    if context.sentiment_detected == Some("threat") {
        return Action::AlertHuman { 
            reason: "Menace détectée".to_string()
        };
    }
    
    // Règle 4: Flux normal
    return Action::RespondText { text: "" };
}
```

**Résultat** : `Action::RespondText`

### 4. Génération Texte (IA Gateway)

**Seulement si** `Action::RespondText` ou `Action::RespondWithMedia`

```rust
// Prompt généré PAR LE BACKEND (pas par l'IA)
let constraints = AIConstraints {
    objective: "Répondre question prix",
    tone: "professionnel, amical",
    max_length: 200,
    prohibitions: vec!["ai", "robot", "humain", "transférer"],
};

let generated_text = ia_gateway.generate_text(constraints).await?;
// → "Bonjour ! Le produit X est à 49.99€. Voulez-vous plus d'infos ?"
```

### 5. Anti-Hallucination (Double Validation)

#### Avant IA :
```rust
✓ Produit "X" existe dans Knowledge Base
✓ Action "RespondText" autorisée
✓ Quota disponible
```

#### Après IA :
```rust
let validated = anti_hal.validate_after_ai(generated_text)?;

// Vérifications :
✓ Longueur < 200 caractères
✓ Pas de mots interdits (ai, robot, humain...)
✓ Pas de promesses commerciales non autorisées
```

**En cas d'échec** → Message fallback neutre

### 6. Quota Check & Delay

```rust
if !quota.can_send_message() {
    return Action::Delay { seconds: 3600 };
}

let delay = quota.calculate_delay();
// → 2-8s (base) × progressive_factor (1-3x)

tokio::time::sleep(Duration::from_secs(delay)).await;
```

### 7. Envoi WhatsApp

```rust
quota.record_message()?;
// messages_today: 42 → 43

whatsapp_api.send_text(phone, validated_text).await?;
```

### 8. Audit Log

```rust
audit.log_message_flow(AuditLog {
    id: uuid::Uuid::new_v4(),
    timestamp: Utc::now(),
    conversation_id: "conv-001",
    incoming_message: "Je voudrais le prix du produit X",
    state: "Interest",
    chosen_action: "RespondText",
    ai_prompt: Some("Objectif: Répondre question prix..."),
    ai_response: Some("Bonjour ! Le produit X est à 49.99€..."),
    sent_message: Some("Bonjour ! Le produit X est à 49.99€..."),
    quotas_before: { messages_today: 42, ... },
    quotas_after: { messages_today: 43, ... },
})?;
```

## Gestion des Erreurs

### Erreurs Critiques (Arrêt Automation)

1. **IA indisponible** → Action::Ignore
2. **Quota atteint** → Action::Delay(3600) ou StopAutomation
3. **Licence expirée** → StopAutomation
4. **WhatsApp déconnecté** → StopAutomation
5. **Validation IA échoue** → Fallback message neutre

### Principe de Sécurité

> **Le système se tait plutôt que de risquer une erreur.**

## Machine à États (Conversation Engine)

### États Possibles

```rust
enum ConversationState {
    Discovery,   // État initial
    Interest,    // Client intéressé
    Intent,      // Intention d'achat
    Objection,   // Objection levée
    Negative,    // Réponse négative
    Escalated,   // ⚠️ Terminal - menace/colère
    Frozen,      // ⚠️ Terminal - gelé manuellement
}
```

### Événements de Transition

```rust
enum ConversationEvent {
    ProductQuestion,
    PriceInterest,
    PurchaseIntent,
    ObjectionRaised,
    NegativeResponse,
    ThreatDetected,  // → Escalated (terminal)
    Freeze,          // → Frozen (terminal)
}
```

### Matrice de Transition

| État Actuel | Événement | État Suivant |
|-------------|-----------|--------------|
| Discovery | ProductQuestion | Interest |
| Discovery | ThreatDetected | Escalated ⚠️ |
| Interest | PriceInterest | Intent |
| Interest | ObjectionRaised | Objection |
| Objection | ProductQuestion | Interest |
| Escalated | * | Escalated (bloqué) |
| Frozen | * | Frozen (bloqué) |

**États terminaux** : Escalated, Frozen
→ Automation arrêtée, intervention humaine requise

## Sécurité & Cryptographie

### Storage Engine (Chiffrement)

```rust
// TODO: Implémentation complète
// Utilise AES-256-GCM

let key = derive_key_from_hwid(hwid)?;
let cipher = Aes256Gcm::new(&key);

// Chiffrement
let nonce = Aes256Gcm::generate_nonce();
let encrypted = cipher.encrypt(&nonce, data.as_ref())?;

// Stockage
db.execute("INSERT INTO encrypted_data VALUES (?, ?)", 
    [nonce, encrypted])?;
```

### License Engine (HWID)

```rust
let hwid = machine_uid::get()?;
// → "unique-hardware-id-xxx"

// Vérification à chaque démarrage
if license.hwid != current_hwid {
    return LicenseState::HwidMismatch;
}
```

## Performance

### Benchmarks Cibles

- **Décision** : < 1ms
- **Validation IA** : < 5ms
- **Génération texte IA** : < 2s
- **Stockage audit** : < 10ms

### Optimisations

1. **Decision Engine** : Zero-allocation
2. **Quota Engine** : In-memory cache
3. **Storage** : Connection pooling
4. **Anti-Hallucination** : Regex pré-compilées

## Tests

### Couverture par Moteur

```
License Engine:        2 tests ✅
Storage Engine:        1 test  ✅
Config Engine:         2 tests ✅
Knowledge Base:        4 tests ✅
Conversation Engine:   6 tests ✅
Quota Engine:          6 tests ✅
Decision Engine:       2 tests ✅
Alert Engine:          4 tests ✅
IA Gateway:            3 tests ✅
Anti-Hallucination:    2 tests ✅
Audit Engine:          3 tests ✅
Integration:           1 test  ✅
──────────────────────────────
TOTAL:                36 tests ✅
```

### Tests d'Intégration

```rust
#[tokio::test]
async fn test_full_message_flow() {
    // 1. Init engines
    let mut storage = StorageEngine::new(db_path)?;
    storage.initialize()?;
    
    let decision = DecisionEngine::new();
    let quota = QuotaEngine::default();
    let anti_hal = AntiHallucinationEngine::new();
    
    // 2. Simulate incoming message
    let context = DecisionContext { ... };
    
    // 3. Decide action
    let action = decision.decide(context)?;
    assert!(matches!(action, Action::RespondText { .. }));
    
    // 4. Generate text (mock)
    let generated = "Test response";
    
    // 5. Validate
    let validated = anti_hal.validate_after_ai(generated)?;
    
    // 6. Check quota
    assert!(quota.can_send_message());
    
    // ✅ Complete flow validated
}
```

## Déploiement

### Docker (Recommandé)

```dockerfile
FROM rust:1.77-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libssl3 \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*
    
COPY --from=builder /app/target/release/sellify-core /usr/local/bin/

EXPOSE 3000
CMD ["sellify-core"]
```

### Variables d'Environnement

```bash
# License
SELLIFY_LICENSE_KEY=xxx

# AI
SELLIFY_AI_API_KEY=xxx
SELLIFY_AI_PROVIDER=openai  # or anthropic, etc.

# Storage
SELLIFY_DB_PATH=/data/sellify.db

# Logs
RUST_LOG=info
RUST_BACKTRACE=1
```

## Monitoring

### Métriques Exposées

```
sellify_decisions_total{action="respond_text"}
sellify_decisions_total{action="ignore"}
sellify_decisions_total{action="alert_human"}

sellify_quota_usage{type="messages_today"}
sellify_quota_usage{type="images_today"}

sellify_ai_generation_duration_seconds
sellify_ai_validation_failures_total

sellify_conversation_state{state="interest"}
sellify_conversation_state{state="escalated"}
```

### Alertes Recommandées

- ⚠️ Quota > 80%
- 🚨 État Escalated count > 5/jour
- ⚠️ Validation IA échec > 10%
- 🚨 License expiration < 7 jours

---

Pour plus d'informations, voir [README.md](README.md)
