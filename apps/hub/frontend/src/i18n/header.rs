//! Header translations

use super::Language;

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

pub fn translate(key: &str, lang: Language) -> Option<&'static str> {
    match key {
        "header.connect_wallet" => Some(connect_wallet(lang)),
        "header.toggle_theme" => Some(toggle_theme(lang)),
        _ => None,
    }
}

