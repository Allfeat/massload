//! Protect page translations

use super::Language;

pub fn translate(key: &str, lang: Language) -> Option<&'static str> {
    match key {
        "protect.title" => Some(match lang {
            Language::English => "Protect",
            Language::French => "Protéger",
            Language::Spanish => "Proteger",
            Language::German => "Schützen",
            Language::Japanese => "保護",
            Language::Korean => "보호",
            Language::Greek => "Προστασία",
        }),
        "protect.coming_soon" => Some(match lang {
            Language::English => "IP protection features are coming soon to the unified hub.",
            Language::French => "Les fonctionnalités de protection IP arrivent bientôt sur le hub unifié.",
            Language::Spanish => "Las funciones de protección IP llegarán pronto al hub unificado.",
            Language::German => "IP-Schutzfunktionen kommen bald zum vereinheitlichten Hub.",
            Language::Japanese => "IP保護機能は統合ハブに近日公開予定。",
            Language::Korean => "IP 보호 기능이 곧 통합 허브에 출시됩니다.",
            Language::Greek => "Οι λειτουργίες προστασίας IP έρχονται σύντομα στο ενοποιημένο hub.",
        }),
        "protect.features_title" => Some(match lang {
            Language::English => "Upcoming Features",
            Language::French => "Fonctionnalités à venir",
            Language::Spanish => "Próximas funciones",
            Language::German => "Kommende Funktionen",
            Language::Japanese => "今後の機能",
            Language::Korean => "예정된 기능",
            Language::Greek => "Επερχόμενες λειτουργίες",
        }),
        "protect.feature1" => Some(match lang {
            Language::English => "Blockchain-certified timestamps",
            Language::French => "Horodatages certifiés blockchain",
            Language::Spanish => "Marcas de tiempo certificadas por blockchain",
            Language::German => "Blockchain-zertifizierte Zeitstempel",
            Language::Japanese => "ブロックチェーン認証タイムスタンプ",
            Language::Korean => "블록체인 인증 타임스탬프",
            Language::Greek => "Χρονοσήμανση πιστοποιημένη από blockchain",
        }),
        "protect.feature2" => Some(match lang {
            Language::English => "Proof of existence certificates",
            Language::French => "Certificats de preuve d'existence",
            Language::Spanish => "Certificados de prueba de existencia",
            Language::German => "Existenznachweiszertifikate",
            Language::Japanese => "存在証明書",
            Language::Korean => "존재 증명 인증서",
            Language::Greek => "Πιστοποιητικά απόδειξης ύπαρξης",
        }),
        "protect.feature3" => Some(match lang {
            Language::English => "Dispute resolution support",
            Language::French => "Support de résolution des litiges",
            Language::Spanish => "Soporte de resolución de disputas",
            Language::German => "Streitbeilegungsunterstützung",
            Language::Japanese => "紛争解決サポート",
            Language::Korean => "분쟁 해결 지원",
            Language::Greek => "Υποστήριξη επίλυσης διαφορών",
        }),
        "protect.current_version" => Some(match lang {
            Language::English => "Use current version at protect.allfeat.org",
            Language::French => "Utiliser la version actuelle sur protect.allfeat.org",
            Language::Spanish => "Usar versión actual en protect.allfeat.org",
            Language::German => "Aktuelle Version auf protect.allfeat.org verwenden",
            Language::Japanese => "protect.allfeat.orgで現在のバージョンを使用",
            Language::Korean => "protect.allfeat.org에서 현재 버전 사용",
            Language::Greek => "Χρήση τρέχουσας έκδοσης στο protect.allfeat.org",
        }),
        _ => None,
    }
}

