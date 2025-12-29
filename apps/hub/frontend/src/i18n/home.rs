//! Home page translations

use super::Language;

pub fn translate(key: &str, lang: Language) -> Option<&'static str> {
    match key {
        "home.welcome" => Some(match lang {
            Language::English => "Welcome to Allfeat Apps",
            Language::French => "Bienvenue sur Allfeat Apps",
            Language::Spanish => "Bienvenido a Allfeat Apps",
            Language::German => "Willkommen bei Allfeat Apps",
            Language::Japanese => "Allfeat Appsへようこそ",
            Language::Korean => "Allfeat Apps에 오신 것을 환영합니다",
            Language::Greek => "Καλώς ήρθατε στο Allfeat Apps",
        }),
        "home.subtitle" => Some(match lang {
            Language::English => "Register and protect your musical works on the blockchain.",
            Language::French => "Enregistrez et protégez vos œuvres musicales sur la blockchain.",
            Language::Spanish => "Registre y proteja sus obras musicales en blockchain.",
            Language::German => "Registrieren und schützen Sie Ihre Musikwerke auf der Blockchain.",
            Language::Japanese => "ブロックチェーン上で音楽作品を登録・保護します。",
            Language::Korean => "블록체인에서 음악 작품을 등록하고 보호하세요.",
            Language::Greek => "Καταχωρίστε και προστατέψτε τα μουσικά σας έργα στο blockchain.",
        }),
        
        // Feature cards
        "home.massload.title" => Some(match lang {
            Language::English => "Mass Load",
            Language::French => "Mass Load",
            Language::Spanish => "Carga masiva",
            Language::German => "Massenladen",
            Language::Japanese => "一括読み込み",
            Language::Korean => "대량 로드",
            Language::Greek => "Μαζική φόρτωση",
        }),
        "home.massload.desc" => Some(match lang {
            Language::English => "Bulk register hundreds of works from a CSV file with AI-powered transformation.",
            Language::French => "Enregistrez des centaines d'œuvres depuis un fichier CSV avec transformation IA.",
            Language::Spanish => "Registre cientos de obras desde un archivo CSV con transformación IA.",
            Language::German => "Registrieren Sie Hunderte von Werken aus einer CSV-Datei mit KI-Transformation.",
            Language::Japanese => "AIによる変換でCSVファイルから数百の作品を一括登録。",
            Language::Korean => "AI 변환으로 CSV 파일에서 수백 개의 작품을 대량 등록.",
            Language::Greek => "Μαζική εγγραφή εκατοντάδων έργων από αρχείο CSV με μετασχηματισμό AI.",
        }),
        "home.register.title" => Some(match lang {
            Language::English => "Register",
            Language::French => "Enregistrer",
            Language::Spanish => "Registrar",
            Language::German => "Registrieren",
            Language::Japanese => "登録",
            Language::Korean => "등록",
            Language::Greek => "Εγγραφή",
        }),
        "home.register.desc" => Some(match lang {
            Language::English => "Register a single musical work with full metadata control.",
            Language::French => "Enregistrez une œuvre musicale avec contrôle complet des métadonnées.",
            Language::Spanish => "Registre una obra musical con control total de metadatos.",
            Language::German => "Registrieren Sie ein einzelnes Werk mit voller Metadaten-Kontrolle.",
            Language::Japanese => "完全なメタデータ制御で単一の作品を登録。",
            Language::Korean => "완전한 메타데이터 제어로 단일 작품 등록.",
            Language::Greek => "Καταχωρίστε ένα μουσικό έργο με πλήρη έλεγχο μεταδεδομένων.",
        }),
        "home.protect.title" => Some(match lang {
            Language::English => "Protect",
            Language::French => "Protéger",
            Language::Spanish => "Proteger",
            Language::German => "Schützen",
            Language::Japanese => "保護",
            Language::Korean => "보호",
            Language::Greek => "Προστασία",
        }),
        "home.protect.desc" => Some(match lang {
            Language::English => "Protect your intellectual property with blockchain-certified timestamps.",
            Language::French => "Protégez votre propriété intellectuelle avec des horodatages certifiés blockchain.",
            Language::Spanish => "Proteja su propiedad intelectual con marcas de tiempo certificadas por blockchain.",
            Language::German => "Schützen Sie Ihr geistiges Eigentum mit Blockchain-zertifizierten Zeitstempeln.",
            Language::Japanese => "ブロックチェーン認証タイムスタンプで知的財産を保護。",
            Language::Korean => "블록체인 인증 타임스탬프로 지적 재산 보호.",
            Language::Greek => "Προστατέψτε την πνευματική σας ιδιοκτησία με blockchain-πιστοποιημένες χρονοσφραγίδες.",
        }),
        
        // Resources
        "home.resources.title" => Some(match lang {
            Language::English => "Resources",
            Language::French => "Ressources",
            Language::Spanish => "Recursos",
            Language::German => "Ressourcen",
            Language::Japanese => "リソース",
            Language::Korean => "리소스",
            Language::Greek => "Πόροι",
        }),
        "home.resources.getting_started" => Some(match lang {
            Language::English => "Getting Started",
            Language::French => "Premiers pas",
            Language::Spanish => "Comenzar",
            Language::German => "Erste Schritte",
            Language::Japanese => "はじめに",
            Language::Korean => "시작하기",
            Language::Greek => "Ξεκινώντας",
        }),
        "home.resources.documentation" => Some(match lang {
            Language::English => "Documentation",
            Language::French => "Documentation",
            Language::Spanish => "Documentación",
            Language::German => "Dokumentation",
            Language::Japanese => "ドキュメント",
            Language::Korean => "문서",
            Language::Greek => "Τεκμηρίωση",
        }),
        "home.resources.community" => Some(match lang {
            Language::English => "Community",
            Language::French => "Communauté",
            Language::Spanish => "Comunidad",
            Language::German => "Community",
            Language::Japanese => "コミュニティ",
            Language::Korean => "커뮤니티",
            Language::Greek => "Κοινότητα",
        }),
        "home.resources.github" => Some(match lang {
            Language::English => "GitHub",
            Language::French => "GitHub",
            Language::Spanish => "GitHub",
            Language::German => "GitHub",
            Language::Japanese => "GitHub",
            Language::Korean => "GitHub",
            Language::Greek => "GitHub",
        }),
        
        // Stats
        "home.stats.works" => Some(match lang {
            Language::English => "Works Registered",
            Language::French => "Œuvres enregistrées",
            Language::Spanish => "Obras registradas",
            Language::German => "Registrierte Werke",
            Language::Japanese => "登録作品数",
            Language::Korean => "등록된 작품",
            Language::Greek => "Εγγεγραμμένα έργα",
        }),
        "home.stats.creators" => Some(match lang {
            Language::English => "Creators",
            Language::French => "Créateurs",
            Language::Spanish => "Creadores",
            Language::German => "Ersteller",
            Language::Japanese => "クリエイター",
            Language::Korean => "창작자",
            Language::Greek => "Δημιουργοί",
        }),
        "home.stats.network" => Some(match lang {
            Language::English => "Network",
            Language::French => "Réseau",
            Language::Spanish => "Red",
            Language::German => "Netzwerk",
            Language::Japanese => "ネットワーク",
            Language::Korean => "네트워크",
            Language::Greek => "Δίκτυο",
        }),
        
        _ => None,
    }
}

