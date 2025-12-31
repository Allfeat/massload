//! Common translations (Hero, Upload, Logs)

use super::Language;

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

// System messages
pub fn translate(key: &str, lang: Language) -> Option<&'static str> {
    match key {
        // Navigation
        "nav.midds_registration" => Some(match lang {
            Language::English => "MIDDS Registration",
            Language::French => "Enregistrement MIDDS",
            Language::Spanish => "Registro MIDDS",
            Language::German => "MIDDS-Registrierung",
            Language::Japanese => "MIDDS登録",
            Language::Korean => "MIDDS 등록",
            Language::Greek => "Εγγραφή MIDDS",
        }),
        
        // Form system messages
        "form.validation_errors" => Some(match lang {
            Language::English => "Validation Errors:",
            Language::French => "Erreurs de validation :",
            Language::Spanish => "Errores de validación:",
            Language::German => "Validierungsfehler:",
            Language::Japanese => "検証エラー:",
            Language::Korean => "유효성 검사 오류:",
            Language::Greek => "Σφάλματα επικύρωσης:",
        }),
        
        "form.unknown_error" => Some(match lang {
            Language::English => "Unknown error",
            Language::French => "Erreur inconnue",
            Language::Spanish => "Error desconocido",
            Language::German => "Unbekannter Fehler",
            Language::Japanese => "不明なエラー",
            Language::Korean => "알 수 없는 오류",
            Language::Greek => "Άγνωστο σφάλμα",
        }),
        
        "form.submitting" => Some(match lang {
            Language::English => "Submitting...",
            Language::French => "Envoi en cours...",
            Language::Spanish => "Enviando...",
            Language::German => "Wird gesendet...",
            Language::Japanese => "送信中...",
            Language::Korean => "제출 중...",
            Language::Greek => "Υποβολή...",
        }),
        
        // Language names (for select options)
        "language.english" => Some(match lang {
            Language::English => "English",
            Language::French => "Anglais",
            Language::Spanish => "Inglés",
            Language::German => "Englisch",
            Language::Japanese => "英語",
            Language::Korean => "영어",
            Language::Greek => "Αγγλικά",
        }),
        
        "language.french" => Some(match lang {
            Language::English => "French",
            Language::French => "Français",
            Language::Spanish => "Francés",
            Language::German => "Französisch",
            Language::Japanese => "フランス語",
            Language::Korean => "프랑스어",
            Language::Greek => "Γαλλικά",
        }),
        
        "language.spanish" => Some(match lang {
            Language::English => "Spanish",
            Language::French => "Espagnol",
            Language::Spanish => "Español",
            Language::German => "Spanisch",
            Language::Japanese => "スペイン語",
            Language::Korean => "스페인어",
            Language::Greek => "Ισπανικά",
        }),
        
        "language.german" => Some(match lang {
            Language::English => "German",
            Language::French => "Allemand",
            Language::Spanish => "Alemán",
            Language::German => "Deutsch",
            Language::Japanese => "ドイツ語",
            Language::Korean => "독일어",
            Language::Greek => "Γερμανικά",
        }),
        
        "language.italian" => Some(match lang {
            Language::English => "Italian",
            Language::French => "Italien",
            Language::Spanish => "Italiano",
            Language::German => "Italienisch",
            Language::Japanese => "イタリア語",
            Language::Korean => "이탈리아어",
            Language::Greek => "Ιταλικά",
        }),
        
        "language.portuguese" => Some(match lang {
            Language::English => "Portuguese",
            Language::French => "Portugais",
            Language::Spanish => "Portugués",
            Language::German => "Portugiesisch",
            Language::Japanese => "ポルトガル語",
            Language::Korean => "포르투갈어",
            Language::Greek => "Πορτογαλικά",
        }),
        
        "language.japanese" => Some(match lang {
            Language::English => "Japanese",
            Language::French => "Japonais",
            Language::Spanish => "Japonés",
            Language::German => "Japanisch",
            Language::Japanese => "日本語",
            Language::Korean => "일본어",
            Language::Greek => "Ιαπωνικά",
        }),
        
        "language.korean" => Some(match lang {
            Language::English => "Korean",
            Language::French => "Coréen",
            Language::Spanish => "Coreano",
            Language::German => "Koreanisch",
            Language::Japanese => "韓国語",
            Language::Korean => "한국어",
            Language::Greek => "Κορεατικά",
        }),
        
        "language.chinese" => Some(match lang {
            Language::English => "Chinese",
            Language::French => "Chinois",
            Language::Spanish => "Chino",
            Language::German => "Chinesisch",
            Language::Japanese => "中国語",
            Language::Korean => "중국어",
            Language::Greek => "Κινεζικά",
        }),
        
        "language.arabic" => Some(match lang {
            Language::English => "Arabic",
            Language::French => "Arabe",
            Language::Spanish => "Árabe",
            Language::German => "Arabisch",
            Language::Japanese => "アラビア語",
            Language::Korean => "아랍어",
            Language::Greek => "Αραβικά",
        }),
        
        "language.other" => Some(match lang {
            Language::English => "Other",
            Language::French => "Autre",
            Language::Spanish => "Otro",
            Language::German => "Andere",
            Language::Japanese => "その他",
            Language::Korean => "기타",
            Language::Greek => "Άλλο",
        }),
        
        // Common UI elements
        "common.live" => Some(match lang {
            Language::English => "Live",
            Language::French => "En direct",
            Language::Spanish => "En vivo",
            Language::German => "Live",
            Language::Japanese => "ライブ",
            Language::Korean => "라이브",
            Language::Greek => "Ζωντανά",
        }),
        
        "common.coming_soon" => Some(match lang {
            Language::English => "Coming Soon",
            Language::French => "Bientôt disponible",
            Language::Spanish => "Próximamente",
            Language::German => "Demnächst",
            Language::Japanese => "近日公開",
            Language::Korean => "곧 출시",
            Language::Greek => "Σύντομα",
        }),
        
        "common.midds" => Some(match lang {
            Language::English => "MIDDS",
            Language::French => "MIDDS",
            Language::Spanish => "MIDDS",
            Language::German => "MIDDS",
            Language::Japanese => "MIDDS",
            Language::Korean => "MIDDS",
            Language::Greek => "MIDDS",
        }),
        
        "common.musical_work" => Some(match lang {
            Language::English => "Musical Work",
            Language::French => "Œuvre musicale",
            Language::Spanish => "Obra musical",
            Language::German => "Musikwerk",
            Language::Japanese => "音楽作品",
            Language::Korean => "음악 작품",
            Language::Greek => "Μουσικό έργο",
        }),
        
        "common.recording" => Some(match lang {
            Language::English => "Recording",
            Language::French => "Enregistrement",
            Language::Spanish => "Grabación",
            Language::German => "Aufnahme",
            Language::Japanese => "レコーディング",
            Language::Korean => "녹음",
            Language::Greek => "Ηχογράφηση",
        }),
        
        "common.release" => Some(match lang {
            Language::English => "Release",
            Language::French => "Sortie",
            Language::Spanish => "Lanzamiento",
            Language::German => "Veröffentlichung",
            Language::Japanese => "リリース",
            Language::Korean => "릴리스",
            Language::Greek => "Κυκλοφορία",
        }),
        
        // Network names
        "network.melodie" => Some(match lang {
            Language::English => "Melodie",
            Language::French => "Melodie",
            Language::Spanish => "Melodie",
            Language::German => "Melodie",
            Language::Japanese => "Melodie",
            Language::Korean => "Melodie",
            Language::Greek => "Melodie",
        }),
        
        "network.melodie_testnet" => Some(match lang {
            Language::English => "Melodie Testnet",
            Language::French => "Melodie Testnet",
            Language::Spanish => "Melodie Testnet",
            Language::German => "Melodie Testnet",
            Language::Japanese => "Melodie テストネット",
            Language::Korean => "Melodie 테스트넷",
            Language::Greek => "Melodie Testnet",
        }),
        
        // Footer
        "footer.copyright" => Some(match lang {
            Language::English => "Copyright © 2025 Allfeat",
            Language::French => "Copyright © 2025 Allfeat",
            Language::Spanish => "Copyright © 2025 Allfeat",
            Language::German => "Copyright © 2025 Allfeat",
            Language::Japanese => "Copyright © 2025 Allfeat",
            Language::Korean => "Copyright © 2025 Allfeat",
            Language::Greek => "Copyright © 2025 Allfeat",
        }),
        
        // Upload section
        "upload.uploading" => Some(match lang {
            Language::English => "⏳ Uploading and processing...",
            Language::French => "⏳ Envoi et traitement en cours...",
            Language::Spanish => "⏳ Subiendo y procesando...",
            Language::German => "⏳ Hochladen und Verarbeitung...",
            Language::Japanese => "⏳ アップロード中...",
            Language::Korean => "⏳ 업로드 및 처리 중...",
            Language::Greek => "⏳ Μεταφόρτωση και επεξεργασία...",
        }),
        
        "upload.error_no_wallet_title" => Some(match lang {
            Language::English => "Wallet Not Connected",
            Language::French => "Wallet non connecté",
            Language::Spanish => "Wallet no conectada",
            Language::German => "Wallet nicht verbunden",
            Language::Japanese => "ウォレット未接続",
            Language::Korean => "지갑이 연결되지 않음",
            Language::Greek => "Πορτοφόλι μη συνδεδεμένο",
        }),
        
        "upload.error_no_wallet_message" => Some(match lang {
            Language::English => "Please connect your wallet before uploading a CSV file. Click on \"Connect Wallet\" in the header.",
            Language::French => "Veuillez connecter votre wallet avant de télécharger un fichier CSV. Cliquez sur \"Connecter Wallet\" dans l'en-tête.",
            Language::Spanish => "Conecte su wallet antes de subir un archivo CSV. Haga clic en \"Conectar Wallet\" en el encabezado.",
            Language::German => "Bitte verbinden Sie Ihre Wallet, bevor Sie eine CSV-Datei hochladen. Klicken Sie auf \"Wallet verbinden\" in der Kopfzeile.",
            Language::Japanese => "CSVファイルをアップロードする前にウォレットを接続してください。ヘッダーの「ウォレット接続」をクリックしてください。",
            Language::Korean => "CSV 파일을 업로드하기 전에 지갑을 연결하세요. 헤더의 \"지갑 연결\"을 클릭하세요.",
            Language::Greek => "Συνδέστε το πορτοφόλι σας πριν ανεβάσετε ένα αρχείο CSV. Κάντε κλικ στο \"Σύνδεση Πορτοφολιού\" στην κεφαλίδα.",
        }),
        
        "upload.error_file_too_large_title" => Some(match lang {
            Language::English => "File Too Large",
            Language::French => "Fichier trop volumineux",
            Language::Spanish => "Archivo demasiado grande",
            Language::German => "Datei zu groß",
            Language::Japanese => "ファイルが大きすぎます",
            Language::Korean => "파일이 너무 큼",
            Language::Greek => "Αρχείο πολύ μεγάλο",
        }),
        
        "upload.error_file_too_large_message" => Some(match lang {
            Language::English => "The file is too large",
            Language::French => "Le fichier est trop volumineux",
            Language::Spanish => "El archivo es demasiado grande",
            Language::German => "Die Datei ist zu groß",
            Language::Japanese => "ファイルが大きすぎます",
            Language::Korean => "파일이 너무 큽니다",
            Language::Greek => "Το αρχείο είναι πολύ μεγάλο",
        }),
        
        "upload.error_max_size" => Some(match lang {
            Language::English => "Maximum size:",
            Language::French => "Taille maximale :",
            Language::Spanish => "Tamaño máximo:",
            Language::German => "Maximale Größe:",
            Language::Japanese => "最大サイズ:",
            Language::Korean => "최대 크기:",
            Language::Greek => "Μέγιστο μέγεθος:",
        }),
        
        // Common buttons
        "common.ok" => Some(match lang {
            Language::English => "OK",
            Language::French => "OK",
            Language::Spanish => "OK",
            Language::German => "OK",
            Language::Japanese => "OK",
            Language::Korean => "확인",
            Language::Greek => "OK",
        }),
        
        // Hero section
        "common.title" => Some(title(lang)),
        "common.subtitle" => Some(subtitle(lang)),
        
        // Upload component
        "common.drag_csv_here" => Some(drag_csv_here(lang)),
        "common.or_click_to_select" => Some(or_click_to_select(lang)),
        "common.supported_formats" => Some(supported_formats(lang)),
        "common.auto_transform_ai" => Some(auto_transform_ai(lang)),
        "common.choose_csv_file" => Some(choose_csv_file(lang)),
        
        // Logs panel
        "common.logs" => Some(logs(lang)),
        "common.clear" => Some(clear(lang)),
        
        // File validation
        "common.remove" => Some(match lang {
            Language::English => "Remove",
            Language::French => "Supprimer",
            Language::Spanish => "Eliminar",
            Language::German => "Entfernen",
            Language::Japanese => "削除",
            Language::Korean => "제거",
            Language::Greek => "Αφαίρεση",
        }),
        "common.validate" => Some(match lang {
            Language::English => "Validate",
            Language::French => "Valider",
            Language::Spanish => "Validar",
            Language::German => "Validieren",
            Language::Japanese => "検証",
            Language::Korean => "검증",
            Language::Greek => "Επικύρωση",
        }),
        
        _ => None,
    }
}

