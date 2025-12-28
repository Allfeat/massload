//! Internationalization (i18n) module for Mass Load
//! Supports: English, French, Spanish, German, Japanese, Korean, Greek

use leptos::*;

/// Supported languages
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    French,
    English,
    Spanish,
    German,
    Japanese,
    Korean,
    Greek,
}

impl Language {
    /// Get the flag emoji for this language
    pub fn flag(&self) -> &'static str {
        match self {
            Language::English => "🇬🇧",
            Language::French => "🇫🇷",
            Language::Spanish => "🇪🇸",
            Language::German => "🇩🇪",
            Language::Japanese => "🇯🇵",
            Language::Korean => "🇰🇷",
            Language::Greek => "🇬🇷",
        }
    }

    /// Get the native name of this language
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::German => "Deutsch",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::Greek => "Ελληνικά",
        }
    }

    /// Get all available languages
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::French,
            Language::Spanish,
            Language::German,
            Language::Japanese,
            Language::Korean,
            Language::Greek,
        ]
    }
}

/// Translation keys
pub struct Translations;

impl Translations {
    // Header
    pub fn connect_wallet(lang: Language) -> &'static str {
        match lang {
            Language::English => "Connect wallet",
            Language::French => "Connecter wallet",
            Language::Spanish => "Conectar wallet",
            Language::German => "Wallet verbinden",
            Language::Japanese => "ウォレットを接続",
            Language::Korean => "지갑 연결",
            Language::Greek => "Σύνδεση πορτοφολιού",
        }
    }

    pub fn toggle_theme(lang: Language) -> &'static str {
        match lang {
            Language::English => "Toggle theme",
            Language::French => "Basculer le thème",
            Language::Spanish => "Cambiar tema",
            Language::German => "Thema wechseln",
            Language::Japanese => "テーマを切り替え",
            Language::Korean => "테마 전환",
            Language::Greek => "Εναλλαγή θέματος",
        }
    }

    // Hero section
    pub fn title(lang: Language) -> &'static str {
        match lang {
            Language::English => "Mass Load",
            Language::French => "Mass Load",
            Language::Spanish => "Mass Load",
            Language::German => "Mass Load",
            Language::Japanese => "Mass Load",
            Language::Korean => "Mass Load",
            Language::Greek => "Mass Load",
        }
    }

    pub fn subtitle(lang: Language) -> &'static str {
        match lang {
            Language::English => "Bulk registration of musical works on the Allfeat blockchain. Import your CSV file to certify your metadata in a decentralized manner.",
            Language::French => "Enregistrement en masse d'œuvres musicales sur la blockchain Allfeat. Importez votre fichier CSV pour certifier vos métadonnées de manière décentralisée.",
            Language::Spanish => "Registro masivo de obras musicales en la blockchain Allfeat. Importe su archivo CSV para certificar sus metadatos de forma descentralizada.",
            Language::German => "Massenregistrierung von Musikwerken auf der Allfeat-Blockchain. Importieren Sie Ihre CSV-Datei, um Ihre Metadaten dezentral zu zertifizieren.",
            Language::Japanese => "Allfeatブロックチェーン上での音楽作品の一括登録。CSVファイルをインポートして、メタデータを分散型で認証します。",
            Language::Korean => "Allfeat 블록체인에 음악 작품 대량 등록. CSV 파일을 가져와 메타데이터를 분산 방식으로 인증하세요.",
            Language::Greek => "Μαζική εγγραφή μουσικών έργων στο blockchain Allfeat. Εισάγετε το αρχείο CSV σας για να πιστοποιήσετε τα μεταδεδομένα σας με αποκεντρωμένο τρόπο.",
        }
    }

    // Upload section
    pub fn drag_csv_here(lang: Language) -> &'static str {
        match lang {
            Language::English => "Drag a CSV file here",
            Language::French => "Glissez un fichier CSV ici",
            Language::Spanish => "Arrastre un archivo CSV aquí",
            Language::German => "CSV-Datei hierher ziehen",
            Language::Japanese => "CSVファイルをここにドラッグ",
            Language::Korean => "CSV 파일을 여기에 드래그",
            Language::Greek => "Σύρετε ένα αρχείο CSV εδώ",
        }
    }

    pub fn or_click_to_select(lang: Language) -> &'static str {
        match lang {
            Language::English => "or click to select",
            Language::French => "ou cliquez pour sélectionner",
            Language::Spanish => "o haga clic para seleccionar",
            Language::German => "oder klicken zum Auswählen",
            Language::Japanese => "またはクリックして選択",
            Language::Korean => "또는 클릭하여 선택",
            Language::Greek => "ή κάντε κλικ για επιλογή",
        }
    }

    pub fn supported_formats(lang: Language) -> &'static str {
        match lang {
            Language::English => "Supported formats: SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE",
            Language::French => "Formats supportés : SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE",
            Language::Spanish => "Formatos soportados: SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE",
            Language::German => "Unterstützte Formate: SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE",
            Language::Japanese => "対応フォーマット: SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE",
            Language::Korean => "지원 형식: SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE",
            Language::Greek => "Υποστηριζόμενες μορφές: SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE",
        }
    }

    pub fn auto_transform_ai(lang: Language) -> &'static str {
        match lang {
            Language::English => "Automatic AI transformation",
            Language::French => "Transformation automatique par IA",
            Language::Spanish => "Transformación automática por IA",
            Language::German => "Automatische KI-Transformation",
            Language::Japanese => "AIによる自動変換",
            Language::Korean => "AI 자동 변환",
            Language::Greek => "Αυτόματος μετασχηματισμός AI",
        }
    }

    pub fn choose_csv_file(lang: Language) -> &'static str {
        match lang {
            Language::English => "Choose a CSV file",
            Language::French => "Choisir un fichier CSV",
            Language::Spanish => "Elegir un archivo CSV",
            Language::German => "CSV-Datei auswählen",
            Language::Japanese => "CSVファイルを選択",
            Language::Korean => "CSV 파일 선택",
            Language::Greek => "Επιλέξτε αρχείο CSV",
        }
    }

    // Logs panel
    pub fn logs(lang: Language) -> &'static str {
        match lang {
            Language::English => "Logs",
            Language::French => "Logs",
            Language::Spanish => "Registros",
            Language::German => "Protokolle",
            Language::Japanese => "ログ",
            Language::Korean => "로그",
            Language::Greek => "Αρχεία καταγραφής",
        }
    }

    pub fn clear(lang: Language) -> &'static str {
        match lang {
            Language::English => "Clear",
            Language::French => "Effacer",
            Language::Spanish => "Limpiar",
            Language::German => "Löschen",
            Language::Japanese => "クリア",
            Language::Korean => "지우기",
            Language::Greek => "Εκκαθάριση",
        }
    }
}

/// Create a language context provider
pub fn provide_language_context() {
    let (lang, set_lang) = create_signal(Language::default());
    provide_context(lang);
    provide_context(set_lang);
}

/// Get the current language signal
pub fn use_language() -> ReadSignal<Language> {
    use_context::<ReadSignal<Language>>().expect("Language context not found")
}

/// Get the language setter signal  
pub fn use_set_language() -> WriteSignal<Language> {
    use_context::<WriteSignal<Language>>().expect("Language context not found")
}

/// Translate a key using the current language context
pub fn t(key: &str) -> String {
    let lang = use_language().get();
    translate(key, lang)
}

/// Translate a key for a specific language
pub fn translate(key: &str, lang: Language) -> String {
    match key {
        // Navigation
        "nav.home" => match lang {
            Language::English => "Home",
            Language::French => "Accueil",
            Language::Spanish => "Inicio",
            Language::German => "Startseite",
            Language::Japanese => "ホーム",
            Language::Korean => "홈",
            Language::Greek => "Αρχική",
        },
        "nav.apps" => match lang {
            Language::English => "Apps",
            Language::French => "Applications",
            Language::Spanish => "Aplicaciones",
            Language::German => "Apps",
            Language::Japanese => "アプリ",
            Language::Korean => "앱",
            Language::Greek => "Εφαρμογές",
        },
        "nav.massload" => match lang {
            Language::English => "Mass Load",
            Language::French => "Mass Load",
            Language::Spanish => "Carga Masiva",
            Language::German => "Massenladen",
            Language::Japanese => "一括登録",
            Language::Korean => "대량 등록",
            Language::Greek => "Μαζική Φόρτωση",
        },
        "nav.register" => match lang {
            Language::English => "Register",
            Language::French => "Enregistrer",
            Language::Spanish => "Registrar",
            Language::German => "Registrieren",
            Language::Japanese => "登録",
            Language::Korean => "등록",
            Language::Greek => "Εγγραφή",
        },
        "nav.protect" => match lang {
            Language::English => "Protect",
            Language::French => "Protéger",
            Language::Spanish => "Proteger",
            Language::German => "Schützen",
            Language::Japanese => "保護",
            Language::Korean => "보호",
            Language::Greek => "Προστασία",
        },
        "nav.explore" => match lang {
            Language::English => "Explore",
            Language::French => "Explorer",
            Language::Spanish => "Explorar",
            Language::German => "Erkunden",
            Language::Japanese => "探索",
            Language::Korean => "탐색",
            Language::Greek => "Εξερεύνηση",
        },
        "nav.docs" => match lang {
            Language::English => "Documentation",
            Language::French => "Documentation",
            Language::Spanish => "Documentación",
            Language::German => "Dokumentation",
            Language::Japanese => "ドキュメント",
            Language::Korean => "문서",
            Language::Greek => "Τεκμηρίωση",
        },
        "nav.website" => match lang {
            Language::English => "Website",
            Language::French => "Site web",
            Language::Spanish => "Sitio web",
            Language::German => "Webseite",
            Language::Japanese => "ウェブサイト",
            Language::Korean => "웹사이트",
            Language::Greek => "Ιστοσελίδα",
        },
        "nav.how_it_works" => match lang {
            Language::English => "How it works",
            Language::French => "Comment ça marche",
            Language::Spanish => "Cómo funciona",
            Language::German => "Wie es funktioniert",
            Language::Japanese => "仕組み",
            Language::Korean => "작동 방식",
            Language::Greek => "Πώς λειτουργεί",
        },
        
        // Home page
        "home.welcome" => match lang {
            Language::English => "Welcome to Allfeat Apps",
            Language::French => "Bienvenue sur Allfeat Apps",
            Language::Spanish => "Bienvenido a Allfeat Apps",
            Language::German => "Willkommen bei Allfeat Apps",
            Language::Japanese => "Allfeat Appsへようこそ",
            Language::Korean => "Allfeat Apps에 오신 것을 환영합니다",
            Language::Greek => "Καλώς ήρθατε στο Allfeat Apps",
        },
        "home.subtitle" => match lang {
            Language::English => "Register and protect your musical works on the blockchain.",
            Language::French => "Enregistrez et protégez vos œuvres musicales sur la blockchain.",
            Language::Spanish => "Registre y proteja sus obras musicales en blockchain.",
            Language::German => "Registrieren und schützen Sie Ihre Musikwerke auf der Blockchain.",
            Language::Japanese => "ブロックチェーン上で音楽作品を登録・保護します。",
            Language::Korean => "블록체인에서 음악 작품을 등록하고 보호하세요.",
            Language::Greek => "Καταχωρίστε και προστατέψτε τα μουσικά σας έργα στο blockchain.",
        },
        
        // Feature cards
        "home.massload.title" => match lang {
            Language::English => "Mass Load",
            Language::French => "Mass Load",
            _ => "Mass Load",
        },
        "home.massload.desc" => match lang {
            Language::English => "Bulk register hundreds of works from a CSV file with AI-powered transformation.",
            Language::French => "Enregistrez des centaines d'œuvres depuis un fichier CSV avec transformation IA.",
            Language::Spanish => "Registre cientos de obras desde un archivo CSV con transformación IA.",
            Language::German => "Registrieren Sie Hunderte von Werken aus einer CSV-Datei mit KI-Transformation.",
            Language::Japanese => "AIによる変換でCSVファイルから数百の作品を一括登録。",
            Language::Korean => "AI 변환으로 CSV 파일에서 수백 개의 작품을 대량 등록.",
            Language::Greek => "Μαζική εγγραφή εκατοντάδων έργων από αρχείο CSV με μετασχηματισμό AI.",
        },
        "home.register.title" => match lang {
            Language::English => "Register",
            Language::French => "Enregistrer",
            _ => "Register",
        },
        "home.register.desc" => match lang {
            Language::English => "Register a single musical work with full metadata control.",
            Language::French => "Enregistrez une œuvre musicale avec contrôle complet des métadonnées.",
            Language::Spanish => "Registre una obra musical con control total de metadatos.",
            Language::German => "Registrieren Sie ein einzelnes Werk mit voller Metadaten-Kontrolle.",
            Language::Japanese => "完全なメタデータ制御で単一の作品を登録。",
            Language::Korean => "완전한 메타데이터 제어로 단일 작품 등록.",
            Language::Greek => "Καταχωρίστε ένα μουσικό έργο με πλήρη έλεγχο μεταδεδομένων.",
        },
        "home.protect.title" => match lang {
            Language::English => "Protect",
            Language::French => "Protéger",
            _ => "Protect",
        },
        "home.protect.desc" => match lang {
            Language::English => "Protect your intellectual property with blockchain-certified timestamps.",
            Language::French => "Protégez votre propriété intellectuelle avec des horodatages certifiés blockchain.",
            Language::Spanish => "Proteja su propiedad intelectual con marcas de tiempo certificadas por blockchain.",
            Language::German => "Schützen Sie Ihr geistiges Eigentum mit Blockchain-zertifizierten Zeitstempeln.",
            Language::Japanese => "ブロックチェーン認証タイムスタンプで知的財産を保護。",
            Language::Korean => "블록체인 인증 타임스탬프로 지적 재산 보호.",
            Language::Greek => "Προστατέψτε την πνευματική σας ιδιοκτησία με blockchain-πιστοποιημένες χρονοσφραγίδες.",
        },
        
        // Resources
        "home.resources.title" => match lang {
            Language::English => "Resources",
            Language::French => "Ressources",
            Language::Spanish => "Recursos",
            Language::German => "Ressourcen",
            Language::Japanese => "リソース",
            Language::Korean => "리소스",
            Language::Greek => "Πόροι",
        },
        "home.resources.getting_started" => match lang {
            Language::English => "Getting Started",
            Language::French => "Premiers pas",
            Language::Spanish => "Comenzar",
            Language::German => "Erste Schritte",
            Language::Japanese => "はじめに",
            Language::Korean => "시작하기",
            Language::Greek => "Ξεκινώντας",
        },
        "home.resources.documentation" => match lang {
            Language::English => "Documentation",
            Language::French => "Documentation",
            _ => "Documentation",
        },
        "home.resources.community" => match lang {
            Language::English => "Community",
            Language::French => "Communauté",
            Language::Spanish => "Comunidad",
            Language::German => "Community",
            Language::Japanese => "コミュニティ",
            Language::Korean => "커뮤니티",
            Language::Greek => "Κοινότητα",
        },
        "home.resources.github" => match lang {
            Language::English => "GitHub",
            _ => "GitHub",
        },
        
        // Stats
        "home.stats.works" => match lang {
            Language::English => "Works Registered",
            Language::French => "Œuvres enregistrées",
            Language::Spanish => "Obras registradas",
            Language::German => "Registrierte Werke",
            Language::Japanese => "登録作品数",
            Language::Korean => "등록된 작품",
            Language::Greek => "Εγγεγραμμένα έργα",
        },
        "home.stats.creators" => match lang {
            Language::English => "Creators",
            Language::French => "Créateurs",
            Language::Spanish => "Creadores",
            Language::German => "Ersteller",
            Language::Japanese => "クリエイター",
            Language::Korean => "창작자",
            Language::Greek => "Δημιουργοί",
        },
        "home.stats.network" => match lang {
            Language::English => "Network",
            Language::French => "Réseau",
            Language::Spanish => "Red",
            Language::German => "Netzwerk",
            Language::Japanese => "ネットワーク",
            Language::Korean => "네트워크",
            Language::Greek => "Δίκτυο",
        },
        
        // Register page (coming soon)
        "register.title" => match lang {
            Language::English => "Register",
            Language::French => "Enregistrer",
            _ => "Register",
        },
        "register.coming_soon" => match lang {
            Language::English => "Single work registration is coming soon to the unified hub.",
            Language::French => "L'enregistrement d'œuvres individuelles arrive bientôt sur le hub unifié.",
            Language::Spanish => "El registro de obras individuales llegará pronto al hub unificado.",
            Language::German => "Die Einzelwerk-Registrierung kommt bald zum vereinheitlichten Hub.",
            Language::Japanese => "単一作品の登録は統合ハブに近日公開予定。",
            Language::Korean => "단일 작품 등록이 곧 통합 허브에 출시됩니다.",
            Language::Greek => "Η εγγραφή μεμονωμένων έργων έρχεται σύντομα στο ενοποιημένο hub.",
        },
        "register.features_title" => match lang {
            Language::English => "Upcoming Features",
            Language::French => "Fonctionnalités à venir",
            _ => "Coming Features",
        },
        "register.feature1" => match lang {
            Language::English => "Step-by-step work registration wizard",
            Language::French => "Assistant d'enregistrement étape par étape",
            _ => "Step-by-step registration wizard",
        },
        "register.feature2" => match lang {
            Language::English => "Full MIDDS metadata support",
            Language::French => "Support complet des métadonnées MIDDS",
            _ => "Full MIDDS metadata support",
        },
        "register.feature3" => match lang {
            Language::English => "Creator and participant management",
            Language::French => "Gestion des créateurs et participants",
            _ => "Creator and participant management",
        },
        "register.current_version" => match lang {
            Language::English => "Use current version at register.allfeat.org",
            Language::French => "Utiliser la version actuelle sur register.allfeat.org",
            _ => "Use current version",
        },
        
        // Protect page (coming soon)
        "protect.title" => match lang {
            Language::English => "Protect",
            Language::French => "Protéger",
            _ => "Protect",
        },
        "protect.coming_soon" => match lang {
            Language::English => "IP protection features are coming soon to the unified hub.",
            Language::French => "Les fonctionnalités de protection IP arrivent bientôt sur le hub unifié.",
            Language::Spanish => "Las funciones de protección IP llegarán pronto al hub unificado.",
            Language::German => "IP-Schutzfunktionen kommen bald zum vereinheitlichten Hub.",
            Language::Japanese => "IP保護機能は統合ハブに近日公開予定。",
            Language::Korean => "IP 보호 기능이 곧 통합 허브에 출시됩니다.",
            Language::Greek => "Οι λειτουργίες προστασίας IP έρχονται σύντομα στο ενοποιημένο hub.",
        },
        "protect.features_title" => match lang {
            Language::English => "Upcoming Features",
            Language::French => "Fonctionnalités à venir",
            _ => "Coming Features",
        },
        "protect.feature1" => match lang {
            Language::English => "Blockchain-certified timestamps",
            Language::French => "Horodatages certifiés blockchain",
            _ => "Blockchain-certified timestamps",
        },
        "protect.feature2" => match lang {
            Language::English => "Proof of existence certificates",
            Language::French => "Certificats de preuve d'existence",
            _ => "Proof of existence certificates",
        },
        "protect.feature3" => match lang {
            Language::English => "Dispute resolution support",
            Language::French => "Support de résolution des litiges",
            _ => "Dispute resolution support",
        },
        "protect.current_version" => match lang {
            Language::English => "Use current version at protect.allfeat.org",
            Language::French => "Utiliser la version actuelle sur protect.allfeat.org",
            _ => "Use current version",
        },
        
        // Explore page
        "explore.title" => match lang {
            Language::English => "Explore",
            Language::French => "Explorer",
            Language::Spanish => "Explorar",
            Language::German => "Erkunden",
            _ => "Explore",
        },
        "explore.subtitle" => match lang {
            Language::English => "Browse registered works, recordings, and releases on the Allfeat blockchain.",
            Language::French => "Parcourez les œuvres, enregistrements et sorties inscrites sur la blockchain Allfeat.",
            _ => "Browse registered works on the blockchain.",
        },
        "explore.search_placeholder" => match lang {
            Language::English => "Search by ISWC, ISRC, title, or artist...",
            Language::French => "Rechercher par ISWC, ISRC, titre ou artiste...",
            _ => "Search...",
        },
        "explore.filter_all" => match lang {
            Language::English => "All",
            Language::French => "Tout",
            _ => "All",
        },
        "explore.filter_works" => match lang {
            Language::English => "Works",
            Language::French => "Œuvres",
            _ => "Works",
        },
        "explore.filter_recordings" => match lang {
            Language::English => "Recordings",
            Language::French => "Enregistrements",
            _ => "Recordings",
        },
        "explore.filter_releases" => match lang {
            Language::English => "Releases",
            Language::French => "Sorties",
            _ => "Releases",
        },
        "explore.total_works" => match lang {
            Language::English => "Musical Works",
            Language::French => "Œuvres musicales",
            _ => "Works",
        },
        "explore.total_recordings" => match lang {
            Language::English => "Recordings",
            Language::French => "Enregistrements",
            _ => "Recordings",
        },
        "explore.total_releases" => match lang {
            Language::English => "Releases",
            Language::French => "Sorties",
            _ => "Releases",
        },
        "explore.total_artists" => match lang {
            Language::English => "Artists",
            Language::French => "Artistes",
            _ => "Artists",
        },
        "explore.no_results" => match lang {
            Language::English => "No results found",
            Language::French => "Aucun résultat trouvé",
            _ => "No results",
        },
        "explore.search_hint" => match lang {
            Language::English => "Try searching for an ISWC, ISRC, or artist name",
            Language::French => "Essayez de rechercher un ISWC, ISRC ou nom d'artiste",
            _ => "Try searching",
        },
        
        // How it works page
        "how.title" => match lang {
            Language::English => "How it works",
            Language::French => "Comment ça marche",
            _ => "How it works",
        },
        "how.subtitle" => match lang {
            Language::English => "Understand the Allfeat ecosystem and how to register your musical works.",
            Language::French => "Comprenez l'écosystème Allfeat et comment enregistrer vos œuvres musicales.",
            _ => "Understand the ecosystem.",
        },
        "how.step1_title" => match lang {
            Language::English => "Register",
            Language::French => "Enregistrer",
            _ => "Register",
        },
        "how.step1_desc" => match lang {
            Language::English => "Submit your musical work metadata in MIDDS format.",
            Language::French => "Soumettez les métadonnées de votre œuvre au format MIDDS.",
            _ => "Submit your metadata.",
        },
        "how.step2_title" => match lang {
            Language::English => "Validate",
            Language::French => "Valider",
            _ => "Validate",
        },
        "how.step2_desc" => match lang {
            Language::English => "Community trusters verify and certify the metadata.",
            Language::French => "Les certificateurs de la communauté vérifient les métadonnées.",
            _ => "Community validates.",
        },
        "how.step3_title" => match lang {
            Language::English => "Certify",
            Language::French => "Certifier",
            _ => "Certify",
        },
        "how.step3_desc" => match lang {
            Language::English => "Your work is permanently recorded on the blockchain.",
            Language::French => "Votre œuvre est enregistrée de façon permanente sur la blockchain.",
            _ => "Recorded on blockchain.",
        },
        "how.step4_title" => match lang {
            Language::English => "Protect",
            Language::French => "Protéger",
            _ => "Protect",
        },
        "how.step4_desc" => match lang {
            Language::English => "Your intellectual property is protected with immutable proof.",
            Language::French => "Votre propriété intellectuelle est protégée avec une preuve immuable.",
            _ => "IP protected.",
        },
        "how.concepts_title" => match lang {
            Language::English => "Key Concepts",
            Language::French => "Concepts clés",
            _ => "Key Concepts",
        },
        "how.midds_desc" => match lang {
            Language::English => "Music Industry Decentralized Data Structures - standardized metadata format.",
            Language::French => "Music Industry Decentralized Data Structures - format de métadonnées standardisé.",
            _ => "Standardized metadata format.",
        },
        "how.iswc_desc" => match lang {
            Language::English => "International Standard Musical Work Code - unique identifier for compositions.",
            Language::French => "International Standard Musical Work Code - identifiant unique pour les compositions.",
            _ => "Unique work identifier.",
        },
        "how.isrc_desc" => match lang {
            Language::English => "International Standard Recording Code - unique identifier for sound recordings.",
            Language::French => "International Standard Recording Code - identifiant unique pour les enregistrements sonores.",
            _ => "Unique recording identifier.",
        },
        "how.ipi_desc" => match lang {
            Language::English => "Interested Parties Information - unique identifier for rights holders.",
            Language::French => "Interested Parties Information - identifiant unique pour les ayants droit.",
            _ => "Rights holder identifier.",
        },
        "how.cta_title" => match lang {
            Language::English => "Ready to get started?",
            Language::French => "Prêt à commencer ?",
            _ => "Ready?",
        },
        "how.start_registering" => match lang {
            Language::English => "Start Registering",
            Language::French => "Commencer à enregistrer",
            _ => "Start",
        },
        "how.read_docs" => match lang {
            Language::English => "Read Documentation",
            Language::French => "Lire la documentation",
            _ => "Read Docs",
        },
        
        // Register page - hub (like register.allfeat.org)
        "register.hero_title" => match lang {
            Language::English => "Music Industry Decentralized Data Structures",
            Language::French => "Music Industry Decentralized Data Structures",
            _ => "Music Industry Decentralized Data Structures",
        },
        "register.hero_subtitle" => match lang {
            Language::English => "Allfeat is the decentralized source of truth for the music industry, leveraging blockchain technology to secure and certify metadata.",
            Language::French => "Allfeat est la source de vérité décentralisée pour l'industrie musicale, exploitant la technologie blockchain pour sécuriser et certifier les métadonnées.",
            Language::Spanish => "Allfeat es la fuente de verdad descentralizada para la industria musical, aprovechando la tecnología blockchain para asegurar y certificar metadatos.",
            Language::German => "Allfeat ist die dezentralisierte Wahrheitsquelle für die Musikindustrie und nutzt Blockchain-Technologie zur Sicherung und Zertifizierung von Metadaten.",
            _ => "Decentralized source of truth for the music industry.",
        },
        "register.group_parties" => match lang {
            Language::English => "Party Identification",
            Language::French => "Identification des parties",
            Language::Spanish => "Identificación de partes",
            Language::German => "Parteienidentifikation",
            _ => "Party Identification",
        },
        "register.group_products" => match lang {
            Language::English => "Product Identification",
            Language::French => "Identification des produits",
            Language::Spanish => "Identificación de productos",
            Language::German => "Produktidentifikation",
            _ => "Product Identification",
        },
        "register.parties_note" => match lang {
            Language::English => "Data privacy in compliance with applicable global regulations.",
            Language::French => "Confidentialité des données conformément aux réglementations mondiales applicables.",
            Language::Spanish => "Privacidad de datos conforme a las regulaciones globales aplicables.",
            Language::German => "Datenschutz gemäß geltenden globalen Vorschriften.",
            _ => "Privacy compliant.",
        },
        "register.massload_title" => match lang {
            Language::English => "Mass Load",
            Language::French => "Mass Load",
            _ => "Mass Load",
        },
        "register.massload_desc" => match lang {
            Language::English => "Bulk register hundreds of works from a CSV file with AI-powered transformation.",
            Language::French => "Enregistrez des centaines d'œuvres depuis un fichier CSV avec transformation IA.",
            _ => "Bulk registration from CSV.",
        },
        "register.subtitle" => match lang {
            Language::English => "Register your musical works, recordings, and releases on the blockchain.",
            Language::French => "Enregistrez vos œuvres musicales, enregistrements et sorties sur la blockchain.",
            _ => "Register on blockchain.",
        },
        "register.musical_work.title" => match lang {
            Language::English => "Musical Work",
            Language::French => "Œuvre musicale",
            _ => "Musical Work",
        },
        "register.musical_work.desc" => match lang {
            Language::English => "Register a composition with ISWC identifier and creator information.",
            Language::French => "Enregistrez une composition avec identifiant ISWC et informations sur les créateurs.",
            _ => "Register a composition.",
        },
        "register.musical_work.form_desc" => match lang {
            Language::English => "Fill in the details of your musical work to register it on the blockchain.",
            Language::French => "Remplissez les détails de votre œuvre musicale pour l'enregistrer sur la blockchain.",
            _ => "Fill in work details.",
        },
        "register.recording.title" => match lang {
            Language::English => "Recording",
            Language::French => "Enregistrement",
            _ => "Recording",
        },
        "register.recording.desc" => match lang {
            Language::English => "Register a sound recording with ISRC identifier linked to a work.",
            Language::French => "Enregistrez un enregistrement sonore avec identifiant ISRC lié à une œuvre.",
            _ => "Register a recording.",
        },
        "register.recording.form_desc" => match lang {
            Language::English => "Fill in the recording details to register it on the blockchain.",
            Language::French => "Remplissez les détails de l'enregistrement pour l'inscrire sur la blockchain.",
            _ => "Fill in recording details.",
        },
        "register.release.title" => match lang {
            Language::English => "Release",
            Language::French => "Sortie",
            _ => "Release",
        },
        "register.release.desc" => match lang {
            Language::English => "Register an album, EP, or single with UPC identifier.",
            Language::French => "Enregistrez un album, EP ou single avec identifiant UPC.",
            _ => "Register a release.",
        },
        "register.release.form_desc" => match lang {
            Language::English => "Fill in the release details to register it on the blockchain.",
            Language::French => "Remplissez les détails de la sortie pour l'inscrire sur la blockchain.",
            _ => "Fill in release details.",
        },
        "register.artist.title" => match lang {
            Language::English => "Artist",
            Language::French => "Artiste",
            _ => "Artist",
        },
        "register.artist.desc" => match lang {
            Language::English => "Register an artist profile with IPI/ISNI identifiers.",
            Language::French => "Enregistrez un profil d'artiste avec identifiants IPI/ISNI.",
            _ => "Register an artist.",
        },
        "register.artist.coming_soon" => match lang {
            Language::English => "Artist registration is coming soon.",
            Language::French => "L'enregistrement d'artistes arrive bientôt.",
            _ => "Coming soon.",
        },
        "register.legal_entity.title" => match lang {
            Language::English => "Legal Entity",
            Language::French => "Entité légale",
            _ => "Legal Entity",
        },
        "register.legal_entity.desc" => match lang {
            Language::English => "Register a label, publisher, or collection society.",
            Language::French => "Enregistrez un label, éditeur ou société de gestion.",
            _ => "Register an entity.",
        },
        "register.legal_entity.coming_soon" => match lang {
            Language::English => "Legal entity registration is coming soon.",
            Language::French => "L'enregistrement d'entités légales arrive bientôt.",
            _ => "Coming soon.",
        },
        "register.mass_cta_title" => match lang {
            Language::English => "Have many works to register?",
            Language::French => "Beaucoup d'œuvres à enregistrer ?",
            _ => "Many works?",
        },
        "register.mass_cta_desc" => match lang {
            Language::English => "Use Mass Load for bulk registration from CSV files.",
            Language::French => "Utilisez Mass Load pour l'enregistrement en masse depuis des fichiers CSV.",
            _ => "Use bulk upload.",
        },
        "register.go_to_massload" => match lang {
            Language::English => "Go to Mass Load",
            Language::French => "Aller à Mass Load",
            _ => "Mass Load",
        },
        "register.back" => match lang {
            Language::English => "Back",
            Language::French => "Retour",
            _ => "Back",
        },
        
        // Register form fields
        "register.form.basic_info" => match lang {
            Language::English => "Basic Information",
            Language::French => "Informations de base",
            _ => "Basic Info",
        },
        "register.form.title" => match lang {
            Language::English => "Title",
            Language::French => "Titre",
            _ => "Title",
        },
        "register.form.title_placeholder" => match lang {
            Language::English => "Enter work title",
            Language::French => "Entrez le titre de l'œuvre",
            _ => "Enter title",
        },
        "register.form.iswc" => match lang {
            Language::English => "ISWC",
            Language::French => "ISWC",
            _ => "ISWC",
        },
        "register.form.iswc_hint" => match lang {
            Language::English => "Optional - International Standard Musical Work Code",
            Language::French => "Optionnel - International Standard Musical Work Code",
            _ => "Optional",
        },
        "register.form.isrc" => match lang {
            Language::English => "ISRC",
            Language::French => "ISRC",
            _ => "ISRC",
        },
        "register.form.linked_work" => match lang {
            Language::English => "Linked Work (ISWC)",
            Language::French => "Œuvre liée (ISWC)",
            _ => "Linked Work",
        },
        "register.form.linked_work_hint" => match lang {
            Language::English => "Optional - Link to a registered musical work",
            Language::French => "Optionnel - Lien vers une œuvre musicale enregistrée",
            _ => "Optional",
        },
        "register.form.duration" => match lang {
            Language::English => "Duration",
            Language::French => "Durée",
            _ => "Duration",
        },
        "register.form.release_title" => match lang {
            Language::English => "Release Title",
            Language::French => "Titre de la sortie",
            _ => "Release Title",
        },
        "register.form.release_date" => match lang {
            Language::English => "Release Date",
            Language::French => "Date de sortie",
            _ => "Release Date",
        },
        "register.form.upc" => match lang {
            Language::English => "UPC",
            Language::French => "UPC",
            _ => "UPC",
        },
        "register.form.creators" => match lang {
            Language::English => "Creators",
            Language::French => "Créateurs",
            _ => "Creators",
        },
        "register.form.creators_desc" => match lang {
            Language::English => "Add the creators and their roles in this work.",
            Language::French => "Ajoutez les créateurs et leurs rôles dans cette œuvre.",
            _ => "Add creators.",
        },
        "register.form.creator_name" => match lang {
            Language::English => "Creator name",
            Language::French => "Nom du créateur",
            _ => "Name",
        },
        "register.form.role_composer" => match lang {
            Language::English => "Composer",
            Language::French => "Compositeur",
            _ => "Composer",
        },
        "register.form.role_author" => match lang {
            Language::English => "Author",
            Language::French => "Auteur",
            _ => "Author",
        },
        "register.form.role_arranger" => match lang {
            Language::English => "Arranger",
            Language::French => "Arrangeur",
            _ => "Arranger",
        },
        "register.form.add_creator" => match lang {
            Language::English => "Add creator",
            Language::French => "Ajouter un créateur",
            _ => "Add",
        },
        "register.form.submit" => match lang {
            Language::English => "Register on Blockchain",
            Language::French => "Enregistrer sur la Blockchain",
            _ => "Register",
        },
        "register.form.wallet_required" => match lang {
            Language::English => "Connect your wallet to submit",
            Language::French => "Connectez votre wallet pour soumettre",
            _ => "Connect wallet",
        },
        "register.form.iswc" => match lang {
            Language::English => "ISWC",
            Language::French => "ISWC",
            _ => "ISWC",
        },
        "register.form.title" => match lang {
            Language::English => "Title",
            Language::French => "Titre",
            _ => "Title",
        },
        "register.form.creation_year" => match lang {
            Language::English => "Creation Year",
            Language::French => "Année de création",
            _ => "Year",
        },
        "register.form.work_type" => match lang {
            Language::English => "Work Type",
            Language::French => "Type d'œuvre",
            _ => "Type",
        },
        "register.form.work_type_original" => match lang {
            Language::English => "Original",
            Language::French => "Original",
            _ => "Original",
        },
        "register.form.work_type_medley" => match lang {
            Language::English => "Medley",
            Language::French => "Medley",
            _ => "Medley",
        },
        "register.form.work_type_mashup" => match lang {
            Language::English => "Mashup",
            Language::French => "Mashup",
            _ => "Mashup",
        },
        "register.form.work_type_adaptation" => match lang {
            Language::English => "Adaptation",
            Language::French => "Adaptation",
            _ => "Adaptation",
        },
        "register.form.instrumental" => match lang {
            Language::English => "Instrumental",
            Language::French => "Instrumental",
            _ => "Instrumental",
        },
        "register.form.basic_info" => match lang {
            Language::English => "Basic Information",
            Language::French => "Informations de base",
            _ => "Basic Info",
        },
        "register.form.creators" => match lang {
            Language::English => "Creators",
            Language::French => "Créateurs",
            _ => "Creators",
        },
        "register.form.role" => match lang {
            Language::English => "Role",
            Language::French => "Rôle",
            _ => "Role",
        },
        "register.form.musical_attributes" => match lang {
            Language::English => "Musical Attributes",
            Language::French => "Attributs musicaux",
            _ => "Attributes",
        },
        "register.form.language" => match lang {
            Language::English => "Language",
            Language::French => "Langue",
            _ => "Language",
        },
        "register.form.key" => match lang {
            Language::English => "Musical Key",
            Language::French => "Tonalité",
            _ => "Key",
        },
        "register.form.classical_info" => match lang {
            Language::English => "Classical Information (Optional)",
            Language::French => "Informations classiques (Optionnel)",
            _ => "Classical Info",
        },
        "register.form.publishers" => match lang {
            Language::English => "Publishers",
            Language::French => "Éditeurs",
            _ => "Publishers",
        },
        
        // Fallback
        _ => key,
    }.to_string()
}

