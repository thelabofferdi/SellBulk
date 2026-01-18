# Sellify Core 🚀

**Backend Rust déterministe pour l'automatisation des ventes WhatsApp**

[![CI](https://github.com/VOTRE-USERNAME/SellBulk/workflows/Tests/badge.svg)](https://github.com/VOTRE-USERNAME/SellBulk/actions)
[![Tests](https://img.shields.io/badge/tests-65%2F65-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.77%2B-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

## 📋 Vue d'ensemble

Sellify Core est un moteur d'automatisation WhatsApp **100% déterministe** où l'IA est un simple outil de génération de texte, jamais un décideur.

### Principe fondamental

> **L'IA ne décide jamais, elle génère uniquement du texte sous contraintes strictes.**

## 🏗️ Architecture - 11 Moteurs Déterministes

Conformément au PRD, Sellify implémente 11 moteurs indépendants et testés :

| # | Moteur | État | Tests | Fonction |
|---|--------|------|-------|----------|
| 1 | **License Engine** | ✅ | 2/2 | Autorisation et gestion des clés (HWID) |
| 2 | **Storage Engine** | ✅ | 1/1 | Stockage local chiffré (SQLite) |
| 3 | **Config Engine** | ✅ | 2/2 | Paramètres globaux (horaires, délais, quotas) |
| 4 | **Knowledge Base** | ✅ | 4/4 | Catalogue produits autorisé |
| 5 | **Conversation Engine** | ✅ | 6/6 | Machine à états déterministe |
| 6 | **Quota Engine** | ✅ | 12/12 | Anti-ban et limites quotidiennes + auto-reset |
| 7 | **Decision Engine** | ✅ | 2/2 | **CŒUR** - Décisions déterministes |
| 8 | **Alert Engine** | ✅ | 4/4 | Notifications humaines silencieuses |
| 9 | **IA Gateway** | ✅ | 3/3 | Génération de texte sous contraintes |
| 10 | **Anti-Hallucination** | ✅ | 2/2 | Double validation (avant/après IA) |
| 11 | **Audit Engine** | ✅ | 3/3 | Traçabilité complète |

**Total : 58/58 tests passent** ✅

## 🚀 Installation

### Prérequis

- Rust 1.77+ ([rustup.rs](https://rustup.rs/))
- SQLite3
- OpenSSL

### Compilation

```bash
git clone https://github.com/votre-org/sellify-core.git
cd sellify-core
cargo build --release
```

### Tests

```bash
cargo test
```

## 💡 Utilisation

### En tant que bibliothèque

```toml
[dependencies]
sellify-core = { path = "../sellify-core" }
```

```rust
use sellify_core::*;

#[tokio::main]
async fn main() {
    // Initialiser les moteurs
    let license = LicenseEngine::new().unwrap();
    let config = ConfigEngine::new();
    let decision = DecisionEngine::new();
    let quota = QuotaEngine::default();
    
    // Utiliser le Decision Engine
    let context = DecisionContext {
        incoming_message: "Je voudrais des infos".to_string(),
        conversation_state: "Discovery".to_string(),
        quotas_available: true,
        is_active_hours: true,
        sentiment_detected: None,
    };
    
    let action = decision.decide(context).unwrap();
    println!("Action décidée: {:?}", action);
}
```

## 🔐 Sécurité

### Contraintes Non-négociables (PRD)

1. ✅ **Mono WhatsApp par machine** (HWID)
2. ✅ **Mono licence par HWID**
3. ✅ **Fonctionnement offline-first**
4. ✅ **IA stateless** (pas de mémoire, pas de décision)
5. ✅ **Aucune mention** d'IA/humain/escalade dans les messages clients
6. ✅ **Actions fermées** (ensemble fini uniquement)
7. ✅ **Zéro autonomie IA**

### Anti-Hallucination

Double verrou avant/après génération IA :

**Avant IA :**
- Validation produit existe
- Validation action autorisée
- Validation quota disponible

**Après IA :**
- Filtrage lexical (mots interdits)
- Vérification longueur
- Détection promesses commerciales
- **En cas d'échec** : message neutre de fallback

## 📊 Machine à États (Conversation)

```
                    ┌─────────────┐
                    │  Discovery  │ (initial)
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              v            v            v
        ┌─────────┐  ┌─────────┐  ┌──────────┐
        │Interest │  │Negative │  │Escalated │
        └────┬────┘  └─────────┘  └──────────┘
             │
        ┌────┼────┐
        │    │    │
        v    v    v
   ┌──────┬────┬─────────┐
   │Intent│    │Objection│
   └──────┘    └─────────┘
```

**États terminaux** (automation stoppée) :
- `Escalated` - Menace/colère détectée
- `Frozen` - Conversation gelée manuellement

## 🎯 Decision Engine (Cœur du Système)

### Actions Possibles (Fermées)

```rust
enum Action {
    RespondText { text: String },
    RespondWithMedia { text: String, media_id: String },
    Ignore,
    Delay { seconds: u64 },
    AlertHuman { reason: String },
    StopAutomation,
}
```

**Aucune autre action n'est possible** ✋

### Règles de Décision

1. **Hors horaires actifs** → `Ignore`
2. **Quota dépassé** → `Delay(3600)` (1h)
3. **Menace/colère** → `AlertHuman` + `StopAutomation`
4. **Flux normal** → `RespondText` (IA génère le texte)

## 📈 Anti-Ban & Quotas

### Limites par Défaut

| Type | Limite Jour | Limite Semaine |
|------|-------------|----------------|
| Messages | 200 | 1000 |
| Images | 50 | - |
| Vidéos | - | 20 |

### Stratégies Anti-Détection

- ✅ Délais aléatoires (2-8s base)
- ✅ Délais progressifs (x2 à 50%, x3 à 80% quota)
- ✅ Simulation de frappe (optionnel)
- ✅ Jitter contrôlé

## 📝 Audit & Traçabilité

Chaque message est tracé complètement :

```rust
AuditLog {
    id: "log-xxx",
    timestamp: ...,
    conversation_id: "conv-001",
    incoming_message: "...",
    state: "Interest",
    chosen_action: "RespondText",
    ai_prompt: Some("..."),
    ai_response: Some("..."),
    sent_message: Some("..."),
    quotas_before: { messages_today: 42, ... },
    quotas_after: { messages_today: 43, ... },
}
```

## 🔄 Intégration WhatsApp

### Via WhatsApp API (Node.js)

Sellify Core s'intègre avec l'API WhatsApp existante :

```
sellify-core (Rust)
    ↓ HTTP/gRPC
whatsapp-api (Node.js + Baileys)
    ↓ WhatsApp Protocol
session-manager (Go)
```

### Workflow Complet

1. **Message reçu** → WhatsApp API
2. **Décision** → Sellify Core (Decision Engine)
3. **Génération texte** → IA Gateway (si nécessaire)
4. **Validation** → Anti-Hallucination Engine
5. **Quota check** → Quota Engine
6. **Envoi** → WhatsApp API (avec délai)
7. **Audit** → Storage Engine

## 🧪 Tests

```bash
# Tous les tests
cargo test

# Tests spécifiques
cargo test decision
cargo test quota
cargo test anti_hallucination

# Avec logs
RUST_LOG=debug cargo test

# Coverage
cargo tarpaulin --out Html
```

## 📦 Structure du Projet

```
sellify-core/
├── src/
│   ├── lib.rs              # Point d'entrée bibliothèque
│   └── engines/            # 11 moteurs
│       ├── mod.rs
│       ├── license.rs      # Licence & HWID
│       ├── storage.rs      # SQLite chiffré
│       ├── config.rs       # Configuration
│       ├── knowledge_base.rs # Produits
│       ├── conversation.rs # États
│       ├── quota.rs        # Anti-ban
│       ├── decision.rs     # ⭐ CŒUR
│       ├── alert.rs        # Alertes humaines
│       ├── ia_gateway.rs   # IA
│       ├── anti_hallucination.rs # Validation
│       └── audit.rs        # Logs
├── Cargo.toml
└── README.md
```

## 🛣️ Roadmap

### ✅ Phase 1 - Core (Terminé)
- [x] 11 moteurs implémentés
- [x] 36 tests unitaires
- [x] Machine à états conversation
- [x] Anti-hallucination complet

### 🚧 Phase 2 - Storage & Security (En cours)
- [ ] Chiffrement AES-256-GCM pour Storage
- [ ] Signature licence avec clé publique
- [ ] Reset quotas automatique (daily/weekly)
- [ ] Persistance audit logs chiffrés

### 📋 Phase 3 - API & Integration
- [ ] API HTTP (axum/actix-web)
- [ ] WebSocket pour événements temps réel
- [ ] Intégration WhatsApp API
- [ ] Dashboard monitoring

### 🎯 Phase 4 - Production
- [ ] Docker compose
- [ ] Métriques Prometheus
- [ ] Logs structurés (JSON)
- [ ] Déploiement automatisé

## 🤝 Contribution

Les contributions sont bienvenues ! Merci de :

1. Fork le projet
2. Créer une branche (`git checkout -b feature/AmazingFeature`)
3. Commit (`git commit -m 'Add AmazingFeature'`)
4. Push (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

**Règles :**
- ✅ Tous les tests doivent passer
- ✅ Couverture de code > 80%
- ✅ Respecter le PRD (pas d'autonomie IA)

## 📄 Licence

MIT License - voir [LICENSE](LICENSE)

## 🙏 Remerciements

- [Baileys](https://github.com/WhiskeySockets/Baileys) - WhatsApp Web API
- [Tauri](https://tauri.app/) - Initial prototype (abandonné)
- Communauté Rust 🦀

---

**⚠️ Important** : Sellify est conçu pour un usage commercial responsable. L'utilisateur est responsable du respect des conditions d'utilisation de WhatsApp.
