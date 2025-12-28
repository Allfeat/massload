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
        
        // Fallback
        _ => key,
    }.to_string()
}

