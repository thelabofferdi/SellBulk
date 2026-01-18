# TODO - Prochaines Étapes Sellify

## 🔴 Priorité CRITIQUE

### 1. Implémenter Chiffrement Storage Engine
**Pourquoi** : Sécurité des données sensibles (conversations, quotas, audit)

**Tâches :**
- [ ] Dériver clé de chiffrement depuis HWID
- [ ] Implémenter AES-256-GCM pour données
- [ ] Chiffrer conversations avant stockage
- [ ] Chiffrer audit logs
- [ ] Tests de chiffrement/déchiffrement
- [ ] Gestion rotation de clés

**Fichiers à modifier :**
- `src/engines/storage.rs`

**Estimation** : 2-3h

---

## 🟠 Priorité HAUTE

### 2. API HTTP avec Axum
**Pourquoi** : Exposer les moteurs à whatsapp-api

**Tâches :**
- [ ] Ajouter dépendance `axum` dans Cargo.toml
- [ ] Créer `src/api/mod.rs`
- [ ] Route POST `/decision` - Prendre décision
- [ ] Route POST `/validate` - Valider texte IA
- [ ] Route GET `/quota` - Consulter quotas
- [ ] Route POST `/audit` - Logger événement
- [ ] Middleware CORS
- [ ] Authentification API key
- [ ] Tests API (integration tests)

**Estimation** : 4-5h

### 3. Intégration whatsapp-api ↔ sellify-core
**Pourquoi** : Tester flux end-to-end

**Tâches :**
- [ ] Connecter whatsapp-api à sellify-core (HTTP)
- [ ] Webhook message reçu → Decision Engine
- [ ] Génération texte IA → Validation
- [ ] Envoi message WhatsApp
- [ ] Tests d'intégration complets

**Estimation** : 3-4h

---

## 🟡 Priorité MOYENNE

### 4. Reset Quotas Automatique
**Pourquoi** : Quotas doivent se réinitialiser chaque jour/semaine

**Tâches :**
- [ ] Cron job pour reset daily (00:00)
- [ ] Cron job pour reset weekly (lundi 00:00)
- [ ] Persistance quotas dans DB
- [ ] Tests de reset
- [ ] Logging des resets

**Estimation** : 2h

### 5. Monitoring & Métriques
**Pourquoi** : Observabilité production

**Tâches :**
- [ ] Ajouter `prometheus` crate
- [ ] Exposer métriques :
  - `sellify_decisions_total{action}`
  - `sellify_quota_usage{type}`
  - `sellify_ai_generation_duration_seconds`
  - `sellify_conversation_state{state}`
- [ ] Route `/metrics` (Prometheus format)
- [ ] Documentation Grafana dashboards

**Estimation** : 3h

### 6. Licence - Signature et Validation
**Pourquoi** : Empêcher licences piratées

**Tâches :**
- [ ] Génération paire clés (publique/privée)
- [ ] Signature licence côté serveur (privée)
- [ ] Validation signature côté client (publique)
- [ ] Vérification expiration
- [ ] Tests de validation

**Estimation** : 2-3h

---

## 🟢 Priorité BASSE

### 7. Docker & Déploiement
**Tâches :**
- [ ] Dockerfile optimisé (multi-stage)
- [ ] docker-compose.yml (sellify + whatsapp-api + session-manager)
- [ ] Variables d'environnement
- [ ] Health checks
- [ ] Logs vers stdout (12-factor)

**Estimation** : 2h

### 8. CI/CD GitHub Actions
**Tâches :**
- [ ] Workflow test (cargo test sur chaque PR)
- [ ] Workflow build (cargo build --release)
- [ ] Workflow release (tag → binary artifacts)
- [ ] Coverage report (tarpaulin)

**Estimation** : 1-2h

### 9. Améliorer IA Gateway
**Tâches :**
- [ ] Support multi-providers (OpenAI, Anthropic, local)
- [ ] Rate limiting API IA
- [ ] Retry avec backoff exponentiel
- [ ] Cache réponses IA (même prompt)
- [ ] Fallback provider si échec

**Estimation** : 3h

### 10. Dashboard Web (Optionnel)
**Tâches :**
- [ ] Frontend React/Vue
- [ ] Visualisation quotas temps réel
- [ ] Liste conversations actives
- [ ] Graphiques états conversation
- [ ] Logs audit explorables
- [ ] Configuration en ligne

**Estimation** : 8-10h

---

## 🔧 Améliorations Code

### Refactoring
- [ ] Réduire warnings Rust (unused imports)
- [ ] Ajouter documentation inline (`///`)
- [ ] Extraire constantes magiques
- [ ] Améliorer gestion erreurs (types custom)

### Tests
- [ ] Tests d'intégration complets
- [ ] Tests de performance (benchmarks)
- [ ] Tests de charge (stress testing)
- [ ] Property-based testing (proptest)

### Documentation
- [ ] API documentation (rustdoc)
- [ ] Guide déploiement production
- [ ] Troubleshooting guide
- [ ] FAQ

---

## 📅 Planning Suggéré

### Semaine 1 - Sécurité & API
- Jour 1-2 : Chiffrement Storage Engine
- Jour 3-4 : API HTTP avec Axum
- Jour 5 : Tests d'intégration

### Semaine 2 - Production Ready
- Jour 1-2 : Intégration whatsapp-api
- Jour 3 : Reset quotas + monitoring
- Jour 4 : Signature licences
- Jour 5 : Docker + CI/CD

### Semaine 3 - Améliorations
- Jour 1-2 : Dashboard (optionnel)
- Jour 3-4 : IA Gateway multi-providers
- Jour 5 : Documentation finale

---

## 🎯 Objectif Court Terme

**Livrable** : Sellify Core production-ready avec :
- ✅ Chiffrement complet
- ✅ API HTTP fonctionnelle
- ✅ Intégration WhatsApp testée
- ✅ Monitoring basique
- ✅ Déploiement Docker

**Délai** : 2 semaines

---

## 📞 Support & Ressources

- **Documentation Rust** : https://doc.rust-lang.org/
- **Axum Docs** : https://docs.rs/axum/
- **AES-GCM** : https://docs.rs/aes-gcm/
- **Prometheus** : https://prometheus.io/docs/
- **Docker** : https://docs.docker.com/

---

Pour toute question, voir :
- `README.md` - Vue d'ensemble
- `ARCHITECTURE.md` - Détails techniques
- `SESSION_SUMMARY.md` - Ce qui a été fait
