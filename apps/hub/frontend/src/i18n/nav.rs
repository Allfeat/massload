//! Navigation translations

use super::Language;

pub fn translate(key: &str, lang: Language) -> Option<&'static str> {
    match key {
        "nav.home" => Some(match lang {
            Language::English => "Home",
            Language::French => "Accueil",
            Language::Spanish => "Inicio",
            Language::German => "Startseite",
            Language::Japanese => "ホーム",
            Language::Korean => "홈",
            Language::Greek => "Αρχική",
        }),
        "nav.apps" => Some(match lang {
            Language::English => "Apps",
            Language::French => "Applications",
            Language::Spanish => "Aplicaciones",
            Language::German => "Apps",
            Language::Japanese => "アプリ",
            Language::Korean => "앱",
            Language::Greek => "Εφαρμογές",
        }),
        "nav.massload" => Some(match lang {
            Language::English => "Mass Load",
            Language::French => "Mass Load",
            Language::Spanish => "Carga Masiva",
            Language::German => "Massenladen",
            Language::Japanese => "一括登録",
            Language::Korean => "대량 등록",
            Language::Greek => "Μαζική Φόρτωση",
        }),
        "nav.register" => Some(match lang {
            Language::English => "Register",
            Language::French => "Enregistrer",
            Language::Spanish => "Registrar",
            Language::German => "Registrieren",
            Language::Japanese => "登録",
            Language::Korean => "등록",
            Language::Greek => "Εγγραφή",
        }),
        "nav.protect" => Some(match lang {
            Language::English => "Protect",
            Language::French => "Protéger",
            Language::Spanish => "Proteger",
            Language::German => "Schützen",
            Language::Japanese => "保護",
            Language::Korean => "보호",
            Language::Greek => "Προστασία",
        }),
        "nav.explore" => Some(match lang {
            Language::English => "Explore",
            Language::French => "Explorer",
            Language::Spanish => "Explorar",
            Language::German => "Erkunden",
            Language::Japanese => "探索",
            Language::Korean => "탐색",
            Language::Greek => "Εξερεύνηση",
        }),
        "nav.docs" => Some(match lang {
            Language::English => "Documentation",
            Language::French => "Documentation",
            Language::Spanish => "Documentación",
            Language::German => "Dokumentation",
            Language::Japanese => "ドキュメント",
            Language::Korean => "문서",
            Language::Greek => "Τεκμηρίωση",
        }),
        "nav.website" => Some(match lang {
            Language::English => "Website",
            Language::French => "Site web",
            Language::Spanish => "Sitio web",
            Language::German => "Webseite",
            Language::Japanese => "ウェブサイト",
            Language::Korean => "웹사이트",
            Language::Greek => "Ιστοσελίδα",
        }),
        "nav.how_it_works" => Some(match lang {
            Language::English => "How it works",
            Language::French => "Comment ça marche",
            Language::Spanish => "Cómo funciona",
            Language::German => "Wie es funktioniert",
            Language::Japanese => "仕組み",
            Language::Korean => "작동 방식",
            Language::Greek => "Πώς λειτουργεί",
        }),
        _ => None,
    }
}

