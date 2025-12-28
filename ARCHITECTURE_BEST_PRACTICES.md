# 🏗️ Architecture Best Practices - Allfeat Apps Hub

## 📋 Table des Matières

1. [Problèmes Identifiés](#problèmes-identifiés)
2. [Bonnes Pratiques Générales](#bonnes-pratiques-générales)
3. [Architecture Recommandée](#architecture-recommandée)
4. [Plan d'Action](#plan-daction)

---

## 🔴 Problèmes Identifiés dans l'Architecture Actuelle

### 1. **Duplication des Types MIDDS**

**❌ Problème :**
```rust
// apps/hub/frontend/src/midds.rs
pub struct MusicalWork { ... }

// crates/core/src/models/musical_work.rs
pub struct MusicalWork { ... }
```

Les types MIDDS sont définis 2 fois : une fois dans le frontend, une fois dans `core`. Cela crée :
- Risque de désynchronisation
- Double maintenance
- Bugs difficiles à tracer

**✅ Solution :**
```rust
// crates/core/src/models/musical_work.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(PartialEq, Eq))]
pub struct MusicalWork { ... }
```

Définir les types **une seule fois** dans `crates/core` avec des feature flags pour WASM.

---

### 2. **Logique Métier dans les Composants UI**

**❌ Problème :**
```rust
// apps/hub/frontend/src/components/musical_work_form_v2.rs (ligne 280+)
let handle_submit = move |ev: web_sys::SubmitEvent| {
    // 70 lignes de logique métier ici !
    // Validation, transformation, appel SDK, etc.
};
```

La validation, transformation et soumission sont directement dans le composant UI.

**✅ Solution :**
```rust
// crates/ui/src/forms/musical_work.rs
pub struct MusicalWorkForm {
    state: FormState,
    validator: FormValidator,
    submitter: FormSubmitter,
}

impl MusicalWorkForm {
    pub fn validate(&self) -> Result<MusicalWork, Vec<String>> { ... }
    pub async fn submit(&self) -> Result<TxHash, Error> { ... }
}
```

Séparer la logique métier dans des modules dédiés.

---

### 3. **Pas de Gestion d'État Centralisée**

**❌ Problème :**
```rust
let (iswc, set_iswc) = create_signal(String::new());
let (title, set_title) = create_signal(String::new());
let (creation_year, set_creation_year) = create_signal(String::new());
// ... 20 signals dispersés
```

État éparpillé dans chaque composant sans structure claire.

**✅ Solution :**
```rust
// crates/ui/src/state/mod.rs
#[derive(Clone, Debug)]
pub struct AppState {
    pub wallet: WalletState,
    pub forms: FormState,
    pub notifications: NotificationState,
}

provide_context(create_rw_signal(AppState::default()));
```

Utiliser un état global typé avec contexts Leptos.

---

### 4. **Services Blockchain dans le Frontend**

**❌ Problème :**
Le `BlockchainService` est défini dans le frontend, mais devrait être partagé.

**✅ Solution :**
```rust
// crates/services/src/blockchain/
├── mod.rs          // Public interface
├── client.rs       // MelodieClient wrapper
├── transactions.rs // Transaction builders
└── signer.rs       // Wallet integration (WASM-only)
```

Services blockchain dans `crates/services` avec feature flags.

---

### 5. **Validation Incohérente**

**❌ Problème :**
- Validation basique dans `MusicalWork::validate()` (frontend)
- Validation JSON schema dans `crates/core/src/validation.rs` (backend)
- Validation SDK dans JavaScript

3 couches de validation différentes !

**✅ Solution :**
```rust
// crates/core/src/validation/
├── mod.rs
├── schema.rs      // JSON schema validation
├── business.rs    // Business rules
└── formats.rs     // ISWC, IPI, ISNI formats
```

Une seule source de vérité pour la validation, partagée via features.

---

## ✅ Bonnes Pratiques Générales

### 1. **Separation of Concerns (SoC)**

```
┌─────────────────────────────────────────────────────────┐
│ Presentation Layer (UI)                                 │
│ • Composants Leptos                                     │
│ • Gestion des événements utilisateur                    │
│ • Rendu visuel                                          │
├─────────────────────────────────────────────────────────┤
│ Application Layer (Logic)                               │
│ • State management                                      │
│ • Form handlers                                         │
│ • Navigation                                            │
├─────────────────────────────────────────────────────────┤
│ Domain Layer (Business)                                 │
│ • Types MIDDS                                           │
│ • Validation rules                                      │
│ • Business logic                                        │
├─────────────────────────────────────────────────────────┤
│ Infrastructure Layer (Services)                         │
│ • Blockchain client                                     │
│ • API calls                                             │
│ • File I/O                                              │
└─────────────────────────────────────────────────────────┘
```

---

### 2. **Feature Flags pour Code Partagé**

```toml
[features]
default = []
wasm = ["wasm-bindgen", "web-sys", "js-sys"]
backend = ["tokio", "axum"]
validation = ["jsonschema"]
```

```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use tokio::fs;
```

---

### 3. **Type-Driven Design**

```rust
// ❌ Mauvais : types faibles
pub fn register(work: serde_json::Value) -> Result<String, String>

// ✅ Bon : types forts
pub fn register(work: MusicalWork) -> Result<TxHash, ValidationError>

// Types newtype pour la sécurité
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iswc(String);

impl Iswc {
    pub fn new(s: impl Into<String>) -> Result<Self, ValidationError> {
        let s = s.into();
        if !s.starts_with('T') || s.len() != 11 {
            return Err(ValidationError::InvalidIswc);
        }
        Ok(Self(s))
    }
}
```

---

### 4. **Error Handling Structuré**

```rust
// crates/core/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum AllfeatError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Blockchain error: {0}")]
    Blockchain(String),
    
    #[error("Wallet not connected")]
    NoWallet,
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}
```

---

### 5. **Testing Strategy**

```rust
// crates/core/tests/
├── integration/
│   ├── musical_work.rs
│   └── validation.rs
├── fixtures/
│   └── sample_works.json
└── common/
    └── mod.rs

// tests/integration/musical_work.rs
#[test]
fn test_musical_work_validation() {
    let work = MusicalWork {
        iswc: Iswc::new("T1234567890").unwrap(),
        title: "Test Song".into(),
        creators: vec![...],
    };
    
    assert!(work.validate().is_ok());
}
```

---

## 🎯 Architecture Recommandée

### Structure du Workspace

```
allfeat-apps/
├── Cargo.toml                    # Workspace root
├── rust-toolchain.toml
│
├── apps/                         # 🚀 Applications déployables
│   ├── hub/
│   │   ├── backend/             # Axum server
│   │   │   ├── src/
│   │   │   │   ├── main.rs     # CLI + Server
│   │   │   │   ├── api/        # Routes HTTP
│   │   │   │   └── config.rs   # Configuration
│   │   │   └── tests/
│   │   │
│   │   └── frontend/            # Leptos CSR + WASM
│   │       ├── src/
│   │       │   ├── main.rs     # WASM entry
│   │       │   ├── app.rs      # Root component
│   │       │   └── pages/      # Routes
│   │       └── index.html
│   │
│   ├── register/                # Future: Register standalone
│   └── protect/                 # Future: Protect standalone
│
├── crates/                       # 📦 Bibliothèques réutilisables
│   │
│   ├── core/                    # 🧱 Domain Layer
│   │   ├── Cargo.toml          # features = ["wasm", "validation"]
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── models/         # MIDDS types
│   │   │   │   ├── mod.rs
│   │   │   │   ├── musical_work.rs
│   │   │   │   ├── recording.rs
│   │   │   │   ├── release.rs
│   │   │   │   └── party.rs
│   │   │   ├── validation/     # Validation logic
│   │   │   │   ├── mod.rs
│   │   │   │   ├── schema.rs
│   │   │   │   ├── formats.rs
│   │   │   │   └── business.rs
│   │   │   └── error.rs        # Error types
│   │   └── tests/
│   │
│   ├── services/                # 🔧 Infrastructure Layer
│   │   ├── Cargo.toml          # features = ["blockchain", "ai", "storage"]
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── blockchain/     # Blockchain integration
│   │   │   │   ├── mod.rs
│   │   │   │   ├── client.rs
│   │   │   │   ├── transactions.rs
│   │   │   │   └── signer.rs   # WASM-only
│   │   │   ├── ai/             # AI transformation
│   │   │   ├── parser/         # CSV parsing
│   │   │   └── cache/          # LRU cache
│   │   └── tests/
│   │
│   └── ui/                      # 🎨 Presentation Layer
│       ├── Cargo.toml          # WASM-only
│       ├── src/
│       │   ├── lib.rs
│       │   ├── components/     # Shared UI components
│       │   │   ├── mod.rs
│       │   │   ├── header.rs
│       │   │   ├── footer.rs
│       │   │   ├── sidebar.rs
│       │   │   └── wallet_modal.rs
│       │   ├── forms/          # Form components
│       │   │   ├── mod.rs
│       │   │   ├── musical_work_form.rs
│       │   │   ├── form_field.rs
│       │   │   └── form_validator.rs
│       │   ├── state/          # State management
│       │   │   ├── mod.rs
│       │   │   ├── app_state.rs
│       │   │   ├── wallet_state.rs
│       │   │   └── form_state.rs
│       │   └── utils/          # UI utilities
│       │       ├── mod.rs
│       │       └── i18n.rs
│       └── style/              # Global CSS
│
├── schemas/                     # 📄 JSON Schemas
│   ├── midds-musical-work.json
│   ├── midds-recording.json
│   └── midds-release.json
│
└── docs/                        # 📚 Documentation
    ├── architecture/
    ├── api/
    └── deployment/
```

---

### Flux de Données Recommandé

```
┌─────────────────────────────────────────────────────────────┐
│ 1. USER ACTION                                              │
│    User fills form & clicks "Submit"                        │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. UI COMPONENT (crates/ui/forms)                          │
│    • Collect form data                                      │
│    • Basic UI validation (required fields, formats)        │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. DOMAIN MODEL (crates/core/models)                       │
│    • Build typed MusicalWork                                │
│    • Business validation (creators, ISWC, etc.)            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. SERVICE LAYER (crates/services/blockchain)              │
│    • Get wallet from context                                │
│    • Serialize to JSON                                      │
│    • Call SDK via wasm_bindgen                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. BLOCKCHAIN SDK (JavaScript)                             │
│    • Convert IPI to BigInt                                  │
│    • Build extrinsic                                        │
│    • Sign with wallet                                       │
│    • Submit transaction                                     │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 6. BLOCKCHAIN (Allfeat Melodie)                           │
│    • Runtime validation (JSON schema)                       │
│    • Storage in pallet                                      │
│    • Emit events                                            │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Plan d'Action

### Phase 1: Refactoring Core ✅ (Priorité Haute)

**Objectif:** Éliminer la duplication des types

```bash
# 1. Déplacer midds.rs vers crates/core
mv apps/hub/frontend/src/midds.rs crates/core/src/models/musical_work.rs

# 2. Ajouter features WASM
# crates/core/Cargo.toml
[features]
default = []
wasm = ["serde-wasm-bindgen"]
validation = ["jsonschema"]

# 3. Adapter pour WASM et backend
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct MusicalWork { ... }

# 4. Utiliser dans frontend
# apps/hub/frontend/Cargo.toml
allfeat-core = { workspace = true, features = ["wasm"] }

# apps/hub/frontend/src/lib.rs
use allfeat_core::models::MusicalWork;
```

**Bénéfices:**
- ✅ Une seule définition des types
- ✅ Validation partagée
- ✅ Tests unifiés

---

### Phase 2: Extract UI Components ⏳ (Priorité Haute)

**Objectif:** Composants réutilisables dans `crates/ui`

```bash
# 1. Créer structure crates/ui
mkdir -p crates/ui/src/{components,forms,state,utils}

# 2. Extraire Header
mv apps/hub/frontend/src/components/header.rs crates/ui/src/components/

# 3. Extraire Footer
mv apps/hub/frontend/src/components/footer.rs crates/ui/src/components/

# 4. Extraire Forms
mv apps/hub/frontend/src/components/musical_work_form_v2.rs \
   crates/ui/src/forms/musical_work_form.rs

# 5. Extraire State Management
cat > crates/ui/src/state/app_state.rs <<'EOF'
#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub wallet: WalletState,
    pub notifications: NotificationState,
}
EOF
```

**Bénéfices:**
- ✅ Réutilisabilité entre apps
- ✅ Tests isolés
- ✅ Maintenance simplifiée

---

### Phase 3: Service Layer 📦 (Priorité Moyenne)

**Objectif:** Services blockchain structurés

```rust
// crates/services/src/blockchain/mod.rs
pub struct BlockchainClient {
    rpc_url: String,
}

impl BlockchainClient {
    pub fn new() -> Self { ... }
    
    pub async fn submit_work(
        &self,
        work: &MusicalWork,
        signer: &WalletSigner,
    ) -> Result<TxHash, BlockchainError> {
        // Validation
        work.validate()?;
        
        // Serialize
        let json = serde_json::to_value(work)?;
        
        // Submit via SDK
        self.call_sdk("submitMusicalWork", json, signer).await
    }
}
```

---

### Phase 4: Testing Infrastructure 🧪 (Priorité Moyenne)

```rust
// crates/core/tests/fixtures/mod.rs
pub fn sample_musical_work() -> MusicalWork {
    MusicalWork {
        iswc: Iswc::new("T1234567890").unwrap(),
        title: "Test Song".into(),
        creators: vec![
            Creator {
                id: PartyId::Ipi(123456789),
                role: Role::Composer,
            }
        ],
        ...
    }
}

// crates/core/tests/integration/musical_work.rs
#[test]
fn test_validation() {
    let work = sample_musical_work();
    assert!(work.validate().is_ok());
}

#[test]
fn test_serialization() {
    let work = sample_musical_work();
    let json = serde_json::to_value(&work).unwrap();
    assert_eq!(json["iswc"], "T1234567890");
}
```

---

### Phase 5: Documentation 📚 (Priorité Basse)

```bash
# 1. API documentation
cargo doc --workspace --no-deps --open

# 2. Architecture diagrams
docs/architecture/overview.md
docs/architecture/data-flow.md

# 3. Deployment guide
docs/deployment/docker.md
docs/deployment/kubernetes.md
```

---

## 🎯 Checklist de Bonnes Pratiques

### Code Quality
- [ ] **DRY (Don't Repeat Yourself)**: Pas de duplication de types
- [ ] **SOLID Principles**: Séparation des responsabilités
- [ ] **Type Safety**: Types forts (newtype pattern)
- [ ] **Error Handling**: thiserror + Result<T, E>
- [ ] **Logging**: log! avec niveaux (info, warn, error)

### Architecture
- [ ] **Layered Architecture**: Presentation / Application / Domain / Infrastructure
- [ ] **Feature Flags**: Code partagé entre WASM et backend
- [ ] **State Management**: Context API Leptos centralisé
- [ ] **Service Layer**: Logique métier hors UI
- [ ] **Validation Layer**: Unique source de vérité

### Testing
- [ ] **Unit Tests**: Chaque fonction critique
- [ ] **Integration Tests**: Flux complets
- [ ] **Fixtures**: Données de test réutilisables
- [ ] **Property Testing**: proptest pour validations
- [ ] **WASM Tests**: wasm-bindgen-test

### Performance
- [ ] **Lazy Loading**: Components chargés à la demande
- [ ] **Memoization**: create_memo pour calculs coûteux
- [ ] **Code Splitting**: Trunk avec multiple entrypoints
- [ ] **Bundle Size**: wasm-opt en production
- [ ] **Caching**: LRU cache pour données fréquentes

### Security
- [ ] **Input Validation**: Toujours valider côté serveur
- [ ] **XSS Protection**: Leptos échappe automatiquement
- [ ] **CORS**: Configuration stricte
- [ ] **Rate Limiting**: Tower middleware
- [ ] **Secrets**: .env + dotenvy, jamais commit

### DevOps
- [ ] **CI/CD**: GitHub Actions
- [ ] **Docker**: Multi-stage builds
- [ ] **Kubernetes**: Helm charts
- [ ] **Monitoring**: Prometheus + Grafana
- [ ] **Logging**: Structured logs (JSON)

---

## 📖 Références

### Rust Best Practices
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

### Leptos Best Practices
- [Leptos Book](https://leptos-rs.github.io/leptos/)
- [Leptos Examples](https://github.com/leptos-rs/leptos/tree/main/examples)
- [CSR vs SSR Trade-offs](https://leptos-rs.github.io/leptos/ssr/index.html)

### Architecture Patterns
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Domain-Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)

---

## 🎬 Conclusion

L'architecture actuelle est **fonctionnelle** mais souffre de :
1. **Duplication des types** entre frontend et backend
2. **Logique métier** mélangée avec l'UI
3. **Manque de tests** structurés

En appliquant ces bonnes pratiques, vous obtiendrez :
- ✅ **Maintenabilité**: Code DRY et bien structuré
- ✅ **Évolutivité**: Ajout facile de nouvelles apps
- ✅ **Fiabilité**: Tests automatisés + types forts
- ✅ **Performance**: WASM optimisé + caching

**Next Steps:**
1. Commencer par **Phase 1** (déplacer types dans core)
2. Continuer avec **Phase 2** (extraire composants UI)
3. Ajouter **tests** au fur et à mesure

Good luck! 🚀

