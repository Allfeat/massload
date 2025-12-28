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

