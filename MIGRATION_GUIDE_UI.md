# Guide de Migration vers `crates/ui`

## 🎯 Objectif

Ce guide explique comment migrer vos composants existants pour utiliser les composants partagés de `crates/ui`.

---

## 📦 1. Ajouter la dépendance

Dans le `Cargo.toml` de votre app frontend :

```toml
[dependencies]
allfeat-ui = { path = "../../../crates/ui" }
```

---

## 🔄 2. Migration des Icons

### Avant
```rust
// apps/hub/frontend/src/components/icons.rs
use leptos::*;

#[component]
pub fn IconHome() -> impl IntoView {
    view! {
        <svg>...</svg>
    }
}
```

### Après
```rust
// apps/hub/frontend/src/components/icons.rs
pub use allfeat_ui::components::icons::*;
```

**Tous les icônes sont maintenant centralisés dans `crates/ui` !**

---

## 📝 3. Migration des Forms

### Avant (composant custom)
```rust
view! {
    <div class="form-field">
        <label>"ISWC Code *"</label>
        <input
            type="text"
            value=move || iswc.get()
            on:input=move |ev| set_iswc.set(event_target_value(&ev))
            placeholder="T-123.456.789-0"
        />
    </div>
}
```

### Après (composant `allfeat-ui`)
```rust
use allfeat_ui::components::forms::*;

view! {
    <TextInput
        label="ISWC Code".to_string()
        name="iswc".to_string()
        value=Signal::derive(move || iswc.get())
        on_change=Callback::new(move |val| set_iswc.set(val))
        placeholder=Some("T-123.456.789-0".to_string())
        required=true
    />
}
```

### Avantages
- ✅ Styling cohérent
- ✅ Gestion d'erreurs intégrée
- ✅ Support des états (required, disabled, loading)
- ✅ Help text automatique

---

## 🧭 4. Migration du Sidebar

### Avant (hardcodé)
```rust
view! {
    <aside class="sidebar">
        <nav>
            <A href="/">"Home"</A>
            <A href="/explore">"Explore"</A>
        </nav>
    </aside>
}
```

### Après (configurable)
```rust
use allfeat_ui::components::sidebar::*;
use crate::i18n::t;

let sections = vec![
    NavSection::new(vec![
        NavItem::new("/", t("nav.home"), view! { <IconHome/> }),
        NavItem::new("/explore", t("nav.explore"), view! { <IconSearch/> }),
    ]),
];

view! {
    <Sidebar 
        sections=sections
        footer=Some(view! {
            <NetworkStatus 
                network_name="Melodie Testnet".to_string()
                online=true
            />
        })
    />
}
```

---

## 👣 5. Migration du Footer

### Avant
```rust
view! {
    <footer>
        <div>"Copyright © 2025 Allfeat"</div>
        <div class="footer-links">
            <a href="https://t.me/Allfeat_fndn">
                <span inner_html=telegram_icon()></span>
            </a>
        </div>
    </footer>
}
```

### Après
```rust
use allfeat_ui::components::footer::*;

view! {
    <Footer
        copyright="Copyright © 2025 Allfeat".to_string()
        social_links=vec![
            SocialLink::telegram("https://t.me/Allfeat_fndn"),
            SocialLink::instagram("https://www.instagram.com/allfeat/"),
            SocialLink::github("https://github.com/allfeat"),
        ]
    />
}
```

---

## 💰 6. Migration du Wallet State

### Avant (props drilling)
```rust
// Dans App.rs
let (wallet_connected, set_wallet_connected) = create_signal(false);
let (wallet_address, set_wallet_address) = create_signal(None::<String>);

view! {
    <Header
        wallet_connected=wallet_connected
        wallet_address=wallet_address
        set_wallet_connected=set_wallet_connected
        set_wallet_address=set_wallet_address
    />
    <SomeComponent
        wallet_connected=wallet_connected
        wallet_address=wallet_address
    />
}
```

### Après (Context API)
```rust
use allfeat_ui::state::wallet::*;

// Dans App.rs (composant racine)
#[component]
pub fn App() -> impl IntoView {
    // Fournir le wallet state à tout l'arbre de composants
    let wallet_state = provide_wallet_state();
    
    view! {
        <Header/>
        <SomeComponent/>
    }
}

// Dans Header.rs
#[component]
pub fn Header() -> impl IntoView {
    let wallet_connected = use_wallet_connected();
    let wallet_address = use_wallet_address();
    let wallet_actions = use_wallet_actions();
    
    let on_connect = move |_| {
        spawn_local(async move {
            // ... logique de connexion
            wallet_actions.connect(address, "SubWallet".to_string());
        });
    };
    
    view! {
        <header>
            <Show when=move || wallet_connected.get()>
                <span>{move || wallet_address.get().unwrap_or_default()}</span>
            </Show>
        </header>
    }
}

// Dans SomeComponent.rs
#[component]
pub fn SomeComponent() -> impl IntoView {
    // Pas besoin de props ! Utiliser le context
    let wallet_connected = use_wallet_connected();
    
    view! {
        <Show when=move || wallet_connected.get()>
            <p>"Wallet connected!"</p>
        </Show>
    }
}
```

### Avantages
- ✅ Plus de props drilling
- ✅ État centralisé
- ✅ Accès depuis n'importe quel composant
- ✅ Hooks réactifs

---

## 🎨 7. Styling

Les composants de `crates/ui` utilisent les classes CSS suivantes :

### Forms
- `.form-field` - Container du champ
- `.form-label` - Label du champ
- `.form-input` - Input texte
- `.form-textarea` - Textarea
- `.form-select` - Select/dropdown
- `.form-checkbox` - Checkbox
- `.form-error` - Message d'erreur
- `.form-help` - Texte d'aide

### Buttons
- `.btn` - Bouton de base
- `.btn-primary` - Bouton primaire
- `.btn-secondary` - Bouton secondaire
- `.btn-danger` - Bouton danger
- `.btn.loading` - État loading

### Sidebar
- `.sidebar` - Container du sidebar
- `.sidebar-nav` - Navigation
- `.nav-section` - Section de navigation
- `.nav-item` - Item de navigation
- `.nav-item.active` - Item actif
- `.nav-icon` - Icône de navigation
- `.nav-label` - Label de navigation
- `.nav-divider` - Séparateur
- `.sidebar-footer` - Footer du sidebar

### Footer
- `footer` - Container du footer
- `.footer-links` - Container des liens sociaux
- `.footer-link` - Lien social

**Important** : Assurez-vous que votre CSS global définit ces classes !

---

## 🧪 8. Exemple Complet : Formulaire de Musical Work

### Avant (tout custom)
```rust
view! {
    <form on:submit=on_submit>
        <div class="form-field">
            <label>"ISWC *"</label>
            <input
                type="text"
                value=move || iswc.get()
                on:input=move |ev| set_iswc.set(event_target_value(&ev))
            />
        </div>
        
        <div class="form-field">
            <label>"Title *"</label>
            <input
                type="text"
                value=move || title.get()
                on:input=move |ev| set_title.set(event_target_value(&ev))
            />
        </div>
        
        <button type="submit" disabled=move || is_submitting.get()>
            {move || if is_submitting.get() { "Submitting..." } else { "Submit" }}
        </button>
    </form>
}
```

### Après (avec `allfeat-ui`)
```rust
use allfeat_ui::components::forms::*;

view! {
    <form on:submit=on_submit>
        <TextInput
            label="ISWC".to_string()
            name="iswc".to_string()
            value=Signal::derive(move || iswc.get())
            on_change=Callback::new(move |val| set_iswc.set(val))
            placeholder=Some("T-123.456.789-0".to_string())
            required=true
            error=Signal::derive(move || iswc_error.get())
        />
        
        <TextInput
            label="Title".to_string()
            name="title".to_string()
            value=Signal::derive(move || title.get())
            on_change=Callback::new(move |val| set_title.set(val))
            required=true
            error=Signal::derive(move || title_error.get())
        />
        
        <Button
            button_type="submit".to_string()
            variant="primary".to_string()
            loading=is_submitting.get()
        >
            "Submit Musical Work"
        </Button>
    </form>
}
```

---

## 📚 9. Ressources

- **Documentation complète** : `PHASE2_COMPLETE.md`
- **Code source** : `crates/ui/src/`
- **Exemples** : `apps/hub/frontend/src/`

---

## ❓ FAQ

### Q: Puis-je personnaliser le style des composants ?
**R:** Oui ! Les composants utilisent des classes CSS standard. Vous pouvez les surcharger dans votre CSS global.

### Q: Comment ajouter un nouveau composant à `crates/ui` ?
**R:** 
1. Créer le fichier dans `crates/ui/src/components/`
2. L'exporter dans `crates/ui/src/components/mod.rs`
3. Documenter son usage dans ce guide

### Q: Le `WalletState` fonctionne-t-il avec tous les wallets ?
**R:** Oui, c'est un état générique. Vous devez implémenter la logique de connexion spécifique dans votre app.

### Q: Puis-je utiliser `crates/ui` dans une app non-Leptos ?
**R:** Non, `crates/ui` est spécifique à Leptos (CSR). Pour d'autres frameworks, créez un crate séparé.

---

## 🎉 Conclusion

La migration vers `crates/ui` améliore la cohérence, la maintenabilité et la réutilisabilité de votre code. Bon refactoring ! 🚀

