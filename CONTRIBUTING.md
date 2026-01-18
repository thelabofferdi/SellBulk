# Contributing to Sellify Core

Merci de ton intérêt pour contribuer à Sellify ! 🚀

## 🔄 CI/CD Automatique

Ce projet utilise **GitHub Actions** pour automatiser les tests et les releases.

### Tests Automatiques

À **chaque push** ou **pull request**, les actions suivantes sont exécutées automatiquement :

1. ✅ **Vérification du formatage** (`cargo fmt --check`)
2. ✅ **Analyse du code** (`cargo clippy`)
3. ✅ **Tests unitaires** (`cargo test --lib`)
4. ✅ **Tests avec features** (`cargo test --features http-server`)
5. ✅ **Compilation** (`cargo build`)
6. ✅ **Couverture de code** (Codecov)

**Tu n'as rien à faire** - GitHub exécute automatiquement tous ces checks !

### Voir les Résultats

1. Va sur l'onglet **Actions** de ce repo
2. Tu verras tous les workflows en cours/terminés
3. Badge vert ✅ = Tous les tests passent
4. Badge rouge ❌ = Un test a échoué

### Avant de Commit

Pour éviter les erreurs dans le CI, lance localement :

```bash
cd sellify-core

# Formatter le code
cargo fmt

# Vérifier avec Clippy
cargo clippy --all-features

# Lancer les tests
cargo test --lib --features http-server
```

---

## 🚀 Créer une Release

### Automatiquement avec GitHub Actions

1. **Choisis un numéro de version** (ex: `1.0.0`)

2. **Crée un tag Git** :
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin v1.0.0
   ```

3. **GitHub Actions se déclenche automatiquement** et :
   - ✅ Compile le binaire optimisé (`--release`)
   - ✅ Crée une release GitHub
   - ✅ Attache les binaires (Linux x86_64 et ARM64)
   - ✅ Build l'image Docker (optionnel)

4. **Les binaires sont disponibles** sur la page Releases !

### Format des Tags

- **Version majeure** : `v1.0.0` (breaking changes)
- **Version mineure** : `v0.2.0` (nouvelles features)
- **Version patch** : `v0.1.1` (bug fixes)

Voir [Semantic Versioning](https://semver.org/) pour plus de détails.

---

## 📝 Workflow de Développement

### 1. Fork & Clone

```bash
git clone https://github.com/TON-USERNAME/SellBulk.git
cd SellBulk
```

### 2. Créer une Branche

```bash
git checkout -b feature/ma-nouvelle-feature
```

### 3. Développer

```bash
cd sellify-core

# Modifier le code...

# Tester localement
cargo test --features http-server

# Formatter
cargo fmt
```

### 4. Commit

```bash
git add .
git commit -m "feat: ajout de ma nouvelle feature"
```

**Format des messages de commit** :
- `feat:` - Nouvelle fonctionnalité
- `fix:` - Correction de bug
- `docs:` - Documentation
- `test:` - Tests
- `refactor:` - Refactoring
- `chore:` - Maintenance

### 5. Push

```bash
git push origin feature/ma-nouvelle-feature
```

### 6. Créer une Pull Request

- Va sur GitHub
- Clique sur "New Pull Request"
- GitHub Actions lance automatiquement les tests
- Attends le ✅ vert avant de merge

---

## 🧪 Tests

### Lancer Tous les Tests

```bash
cd sellify-core
cargo test --all-features
```

### Lancer les Tests d'un Module

```bash
# Tests du quota engine
cargo test --lib engines::quota

# Tests de l'API
cargo test --features http-server api::
```

### Couverture de Code

```bash
# Installer tarpaulin
cargo install cargo-tarpaulin

# Générer le rapport
cargo tarpaulin --workspace --features http-server --out Html
```

---

## 📊 Badges

Le README contient des badges qui se mettent à jour automatiquement :

- **CI Badge** : Statut des tests (✅ passing ou ❌ failing)
- **Tests Badge** : Nombre de tests qui passent
- **Rust Badge** : Version minimale de Rust requise

---

## 🐛 Signaler un Bug

1. Va sur l'onglet **Issues**
2. Clique sur "New Issue"
3. Décris le problème avec :
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Version de Sellify

---

## 💡 Proposer une Feature

1. Ouvre une **Issue** avec le label `enhancement`
2. Décris la feature et pourquoi elle serait utile
3. Discutons ensemble avant de commencer le code

---

## ❓ Questions

Si tu as des questions :
- Ouvre une **Issue** avec le label `question`
- Ou contacte l'équipe directement

---

## 📜 License

En contribuant, tu acceptes que ton code soit sous licence MIT.

Merci de contribuer à Sellify ! 🙏
