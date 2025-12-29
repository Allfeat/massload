# MIDDS Display Components

Composants Leptos réutilisables pour afficher tous les types MIDDS (Music Industry Decentralized Data Structures) depuis la blockchain Allfeat.

## 📂 Structure

```
midds_display/
├── mod.rs           # Module principal avec dispatcher générique
├── common.rs        # Utilitaires partagés
├── work.rs          # Affichage des œuvres musicales
├── recording.rs     # Affichage des enregistrements
└── release.rs       # Affichage des sorties
```

## 🎯 Composants

### 1. `MiddsDisplay` (générique)

Dispatcher qui affiche automatiquement le bon type MIDDS :

```rust
use crate::components::MiddsDisplay;
use crate::services::explorer::MiddsItem;

// Avec un enum MiddsItem
let item = MiddsItem::Work(work_data);
view! { <MiddsDisplay item=item/> }

// Ou directement avec le type spécifique
let item = MiddsItem::Recording(recording_data);
view! { <MiddsDisplay item=item/> }
```

### 2. `WorkDisplay` (spécifique)

Affichage d'une œuvre musicale :

```rust
use crate::components::WorkDisplay;
use crate::MusicalWorkData;

view! { <WorkDisplay work=work_data/> }
```

**Champs affichés** :
- `title` - Titre de l'œuvre
- `iswc` - Code ISWC (International Standard Musical Work Code)
- `creators` - Créateurs avec IPI/ISNI et rôles (Composer, Lyricist, etc.)
- `creationYear` - Année de création
- `workType` - Type (Original, Adaptation, etc.)
- `instrumental` - Boolean
- `language` - Langue (ISO 639-3)
- `key` - Tonalité musicale

### 3. `RecordingDisplay` (spécifique)

Affichage d'un enregistrement :

```rust
use crate::components::RecordingDisplay;
use crate::RecordingData;

view! { <RecordingDisplay recording=recording_data/> }
```

**Champs affichés** :
- `title` - Titre de l'enregistrement
- `isrc` - Code ISRC (International Standard Recording Code)
- `musicalWorkId` - Référence vers l'œuvre musicale
- `performers` - Interprètes avec IPI/ISNI
- `duration` - Durée en ms (formaté en MM:SS)
- `recordingDate` - Date d'enregistrement (ISO)
- `recordingLocation` - Lieu d'enregistrement

### 4. `ReleaseDisplay` (spécifique)

Affichage d'une sortie :

```rust
use crate::components::ReleaseDisplay;
use crate::ReleaseData;

view! { <ReleaseDisplay release=release_data/> }
```

**Champs affichés** :
- `title` - Titre de la sortie
- `upc` - Code UPC/EAN (Universal Product Code)
- `releaseType` - Type (Album, Single, EP, etc.)
- `releaseDate` - Date de sortie (ISO)
- `label` - Label
- `catalogNumber` - Numéro de catalogue
- `recordings` - Liste des enregistrements (IDs)
- `totalTracks` - Nombre total de pistes

## 🛠️ Utilitaires partagés

### `format_party_id(id: &Value) -> String`

Formate un Party ID (IPI/ISNI/Both) depuis JSON :

```rust
use crate::components::midds_display::format_party_id;

let id_str = format_party_id(&creator.id);
// => "IPI: 101010107" ou "ISNI: 0000000123456789" ou "IPI: 101010107 / ISNI: 0000000123456789"
```

### `<MiddsHeader midds_type="..." />`

Badge + type MIDDS :

```rust
<MiddsHeader midds_type="MusicalWork"/>
// => [MIDDS] MusicalWork (badge vert + texte cyan)
```

### `<MiddsField label="..." class="...">`

Champ MIDDS formaté :

```rust
<MiddsField label="title">
    {work.title}
</MiddsField>

<MiddsField label="iswc" class="iswc-value">
    {work.iswc}
</MiddsField>
```

## 🎨 Classes CSS

Les composants utilisent les classes suivantes (définies dans `main.css` et `accordion.css`) :

- `.midds-work`, `.midds-recording`, `.midds-release` - Conteneurs principaux
- `.midds-header` - En-tête avec badge
- `.midds-badge` - Badge "MIDDS" (vert)
- `.midds-type` - Type MIDDS (cyan)
- `.midds-field` - Champ MIDDS
- `.midds-label` - Label du champ
- `.midds-value` - Valeur du champ
- `.midds-array` - Tableau (creators, performers, recordings)
- `.midds-array-item` - Item d'un tableau
- `.midds-object` - Objet imbriqué
- `.midds-prop` - Propriété d'un objet
- `.prop-key`, `.prop-value` - Clé/valeur d'une propriété
- `.role-badge` - Badge de rôle (Composer, Lyricist, etc.)
- `.iswc-value`, `.isrc-value`, `.upc-value` - Codes standards
- `.reference-id` - ID de référence (vers autre MIDDS)

## 🔄 Compatibilité

### Composant legacy

Pour la compatibilité avec le code existant :

```rust
use crate::components::MiddsWorkDisplay;

// Équivalent à WorkDisplay
view! { <MiddsWorkDisplay work=work_data/> }
```

## 📦 Utilisation dans l'explorer

```rust
// Exemple : Explorer page
use crate::components::MiddsWorkDisplay;
use crate::MusicalWorkData;

#[component]
pub fn ExplorePage() -> impl IntoView {
    let works = fetch_all_musical_works().await;
    
    view! {
        <For
            each=move || works.clone().into_iter()
            key=|work| work.id.clone()
            children=move |work| {
                view! {
                    <div class="preview-item">
                        <div class="preview-item-header">
                            {work.title}
                        </div>
                        <div class="preview-item-expanded">
                            <MiddsWorkDisplay work=work/>
                        </div>
                    </div>
                }
            }
        />
    }
}
```

## 📦 Utilisation dans massload

```rust
// Exemple : Massload preview
use crate::components::WorkDisplay;
use serde_json::Value;

// Convertir Value -> MusicalWorkData puis afficher
let work_data: MusicalWorkData = serde_json::from_value(work_json)?;
view! { <WorkDisplay work=work_data/> }
```

## 🚀 Prochaines étapes

1. **Ajouter les fetch JS** pour recordings et releases dans `blockchain.js`
2. **Implémenter les tabs** dans l'explorer (Œuvres / Enregistrements / Sorties)
3. **Migrer massload** pour utiliser `WorkDisplay` au lieu de `WorkDetail`
4. **Tester avec des vraies données** de recordings/releases depuis la blockchain

## 🧪 Tests

Pour tester les composants :

```bash
# Build frontend
cd apps/hub/frontend && trunk build --release

# Run server
cd ../../.. && PORT=3000 ./target/release/allfeat-hub serve

# Navigate to http://localhost:3000/explore
# Click "Œuvres" → Expand an item → MIDDS details should appear
```

## 📝 Contributions

Pour ajouter un nouveau type MIDDS :

1. Créer le struct dans `services/explorer.rs` (e.g., `ArtistData`)
2. Ajouter la variante dans `MiddsItem` enum
3. Créer `artist.rs` avec `ArtistDisplay` component
4. Ajouter le dispatch dans `mod.rs` `MiddsDisplay`
5. Implémenter les fonctions JS dans `blockchain.js`

