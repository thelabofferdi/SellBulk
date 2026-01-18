# SellBulk - Sellify Core

[![CI](https://github.com/thelabofferdi/SellBulk/workflows/Tests/badge.svg)](https://github.com/thelabofferdi/SellBulk/actions)
[![Tests](https://img.shields.io/badge/tests-65%2F65-brightgreen)]()
[![Coverage](https://img.shields.io/badge/coverage-codecov-blue)]()
[![Rust](https://img.shields.io/badge/rust-1.77%2B-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

> **Backend Rust déterministe pour l'automatisation WhatsApp avec IA contrainte**

## 🚀 Qu'est-ce que Sellify ?

Sellify est un **moteur d'automatisation WhatsApp 100% déterministe** où **l'IA ne décide jamais**, elle génère uniquement du texte sous contraintes strictes.

### Principe Fondamental

> **L'IA est un outil de génération de texte, jamais un décideur.**

Toute la logique métier est **déterministe et basée sur des règles** - zéro décision prise par l'IA.

---

## ✨ Features

### 🏗️ Architecture
- ✅ **11 Moteurs Déterministes** indépendants et testés
- ✅ **Machine à États** pour les conversations (7 états)
- ✅ **Double Validation** (avant et après IA)
- ✅ **Traçabilité Complète** avec audit logs

### 🔐 Sécurité
- ✅ **Chiffrement AES-256-GCM** pour les données sensibles
- ✅ **Authentification API Key** sur tous les endpoints
- ✅ **Rate Limiting** (100 req/min par client)
- ✅ **Anti-Hallucination** avec mots interdits

### 📊 Production
- ✅ **API REST** (13 endpoints avec Axum)
- ✅ **Métriques Prometheus** (16 métriques)
- ✅ **Reset Automatique** des quotas (daily/weekly)
- ✅ **Docker** avec docker-compose
- ✅ **CI/CD** GitHub Actions

### 🧪 Qualité
- ✅ **65 Tests Unitaires** (100% passing)
- ✅ **Code Coverage** via Codecov
- ✅ **Clippy + Rustfmt** dans le CI
- ✅ **Documentation Complète**

---

## 📦 Installation

### Prérequis

- **Rust** 1.77+ ([rustup.rs](https://rustup.rs/))
- **SQLite3**
- **OpenSSL 1.1+**

### Via Cargo

```bash
git clone https://github.com/thelabofferdi/SellBulk.git
cd SellBulk/sellify-core

# Lancer les tests
cargo test --features http-server

# Compiler en release
cargo build --release --features http-server --bin sellify-server
```

### Via Docker

```bash
cd SellBulk

# Configurer
cp .env.example .env
# Éditer SELLIFY_API_KEY dans .env

# Lancer
docker-compose up -d

# Vérifier
curl http://localhost:3000/health
```

### Via Release Binaries

Télécharge la dernière release depuis [Releases](https://github.com/thelabofferdi/SellBulk/releases):

```bash
# Linux x86_64
wget https://github.com/thelabofferdi/SellBulk/releases/download/v0.1.0/sellify-server-linux-x86_64-v0.1.0.tar.gz
tar -xzf sellify-server-linux-x86_64-v0.1.0.tar.gz
chmod +x sellify-server

# Lancer
SELLIFY_API_KEY="your-api-key" ./sellify-server
```

---

## 🎯 Quick Start

### 1. Démarrer le Serveur

```bash
cd sellify-core
SELLIFY_API_KEY="dev-api-key" cargo run --bin sellify-server --features http-server
```

Sortie :
```
🕐 Setting up automatic quota reset scheduler...
📅 Daily reset job scheduled (00:00 UTC)
📅 Weekly reset job scheduled (Monday 00:00 UTC)
📊 Prometheus metrics initialized
🚀 Sellify Core API Server starting on http://0.0.0.0:3000
```

### 2. Tester l'API

```bash
# Health check (public)
curl http://localhost:3000/health

# Faire une décision
curl -X POST http://localhost:3000/api/v1/decision \
  -H "Content-Type: application/json" \
  -H "X-API-Key: dev-api-key" \
  -d '{
    "incoming_message": "Bonjour",
    "conversation_state": "Discovery",
    "quotas_available": true,
    "is_active_hours": true,
    "sentiment_detected": null
  }'

# Voir les métriques Prometheus
curl http://localhost:3000/metrics
```

### 3. Intégrer avec WhatsApp

Voir [INTEGRATION_GUIDE.md](INTEGRATION_GUIDE.md) pour le guide complet.

---

## 🏗️ Architecture

### Les 11 Moteurs

| # | Moteur | Tests | Fonction |
|---|--------|-------|----------|
| 1 | **License Engine** | 2/2 ✅ | Validation HWID |
| 2 | **Storage Engine** | 4/4 ✅ | SQLite chiffré (AES-256-GCM) |
| 3 | **Config Engine** | 2/2 ✅ | Paramètres globaux |
| 4 | **Knowledge Base** | 4/4 ✅ | Catalogue produits |
| 5 | **Conversation Engine** | 6/6 ✅ | Machine à états (7 états) |
| 6 | **Quota Engine** | 12/12 ✅ | Anti-ban + auto-reset |
| 7 | **Decision Engine** | 2/2 ✅ | **CŒUR** - Décisions déterministes |
| 8 | **Alert Engine** | 4/4 ✅ | Notifications humaines |
| 9 | **IA Gateway** | 3/3 ✅ | Génération texte contrainte |
| 10 | **Anti-Hallucination** | 2/2 ✅ | Double validation |
| 11 | **Audit Engine** | 3/3 ✅ | Traçabilité complète |

**Total : 65/65 tests passent** ✅

### Flux de Décision

```
Message WhatsApp reçu
    ↓
[1] Decision Engine analyse le contexte
    ↓
[2] Choix d'une action déterministe
    ├─ RespondText
    ├─ RespondWithMedia
    ├─ Ignore
    ├─ Delay
    ├─ AlertHuman
    └─ StopAutomation
    ↓
[3] Si réponse → Anti-Hallucination AVANT
    ↓
[4] IA Gateway génère le texte
    ↓
[5] Anti-Hallucination APRÈS
    ↓
[6] Message validé envoyé
    ↓
[7] Audit log enregistré
```

---

## 📊 API Endpoints

| Méthode | Endpoint | Auth | Description |
|---------|----------|------|-------------|
| GET | `/health` | ❌ | Health check |
| GET | `/metrics` | ❌ | Métriques Prometheus |
| POST | `/api/v1/decision` | ✅ | Prendre une décision |
| POST | `/api/v1/validate` | ✅ | Valider texte IA |
| POST | `/api/v1/quota/check` | ✅ | Vérifier quotas |
| POST | `/api/v1/quota/record` | ✅ | Enregistrer message |
| GET | `/api/v1/quota/status` | ✅ | Statut quotas |
| POST | `/api/v1/quota/reset/daily` | ✅ | Reset quotas daily |
| POST | `/api/v1/quota/reset/weekly` | ✅ | Reset quotas weekly |
| POST | `/api/v1/conversation/transition` | ✅ | Changer état |
| GET | `/api/v1/products` | ✅ | Lister produits |
| GET | `/api/v1/products/:id` | ✅ | Détails produit |
| POST | `/api/v1/audit/log` | ✅ | Logger audit |

Voir [API.md](sellify-core/API.md) pour la documentation complète.

---

## 📈 Monitoring

### Prometheus Metrics

Le endpoint `/metrics` expose 16 métriques :

```bash
# Exemples de métriques disponibles
sellify_decisions_total{action="RespondText"} 42
sellify_quota_messages_today 45
sellify_validation_results{result="valid"} 38
sellify_ai_generation_duration_seconds_sum 12.5
```

### Configuration Prometheus

```yaml
scrape_configs:
  - job_name: 'sellify'
    static_configs:
      - targets: ['localhost:3000']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

### Dashboards Grafana

Voir les suggestions dans [API.md](sellify-core/API.md#monitoring-with-prometheus).

---

## 🧪 Tests

```bash
cd sellify-core

# Tous les tests
cargo test --features http-server

# Tests d'un module spécifique
cargo test --lib engines::quota

# Avec coverage
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --features http-server --out Html
```

**Résultat** : 65/65 tests ✅

---

## 🤝 Contribuer

Les contributions sont les bienvenues ! Voir [CONTRIBUTING.md](CONTRIBUTING.md).

### Quick Start

```bash
# Fork & Clone
git clone https://github.com/thelabofferdi/SellBulk.git
cd SellBulk

# Créer une branche
git checkout -b feature/ma-feature

# Développer
cd sellify-core
# ... code ...

# Tester
cargo fmt
cargo clippy --all-features
cargo test --features http-server

# Commit et Push
git commit -m "feat: ma nouvelle feature"
git push origin feature/ma-feature

# Créer une Pull Request
# Les tests CI se lancent automatiquement !
```

---

## 📚 Documentation

| Fichier | Description |
|---------|-------------|
| [README.md](sellify-core/README.md) | Documentation principale Sellify Core |
| [API.md](sellify-core/API.md) | Documentation API REST |
| [ARCHITECTURE.md](sellify-core/ARCHITECTURE.md) | Architecture technique |
| [INTEGRATION_GUIDE.md](INTEGRATION_GUIDE.md) | Guide intégration WhatsApp |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Guide de contribution |
| [CHANGELOG.md](CHANGELOG.md) | Historique des versions |

### Summaries de Sessions

- [Session 1](SESSION_SUMMARY.md) - Création du backend Rust
- [Session 2](SESSION2_SUMMARY.md) - Chiffrement + API HTTP
- [Session 3](SESSION3_SUMMARY.md) - Auth + Rate Limiting
- [Session 4](SESSION4_SUMMARY.md) - Reset Automatique Quotas
- [Session 5](SESSION5_SUMMARY.md) - Métriques Prometheus
- [Session 6](SESSION6_SUMMARY.md) - CI/CD GitHub Actions

---

## 🐳 Docker

### docker-compose

```bash
# Démarrer tous les services
docker-compose up -d

# Voir les logs
docker-compose logs -f sellify-core

# Arrêter
docker-compose down
```

### Dockerfile

Build optimisé multi-stage (~50 MB final).

---

## 📝 License

MIT License - Voir [LICENSE](LICENSE)

---

## 🙏 Crédits

Développé par **Sellify Team** avec ❤️ et Rust 🦀

---

## 📞 Support

- **Issues** : [GitHub Issues](https://github.com/thelabofferdi/SellBulk/issues)
- **Discussions** : [GitHub Discussions](https://github.com/thelabofferdi/SellBulk/discussions)

---

## 🎯 Roadmap

- [x] Backend Rust déterministe
- [x] API REST avec Axum
- [x] Chiffrement AES-256-GCM
- [x] Reset automatique quotas
- [x] Métriques Prometheus
- [x] CI/CD GitHub Actions
- [ ] Signature cryptographique licences
- [ ] Dashboard Web React
- [ ] Multi-provider IA (OpenAI, Anthropic, local)

---

**Made with Rust 🦀 | Deterministic by Design 🎯 | Production Ready ✅**
