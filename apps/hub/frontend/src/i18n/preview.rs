//! Preview modal translations

use super::Language;

pub fn translate(key: &str, lang: Language) -> Option<&'static str> {
    match key {
        // Preview modal messages
        "preview.connect_wallet_required" => Some(match lang {
            Language::English => "Please connect your wallet before signing",
            Language::French => "Veuillez connecter votre wallet avant de signer",
            Language::Spanish => "Por favor conecte su billetera antes de firmar",
            Language::German => "Bitte verbinden Sie Ihre Wallet vor dem Signieren",
            Language::Japanese => "署名する前にウォレットを接続してください",
            Language::Korean => "서명하기 전에 지갑을 연결하세요",
            Language::Greek => "Συνδέστε το πορτοφόλι σας πριν υπογράψετε",
        }),
        
        "preview.sending_works" => Some(match lang {
            Language::English => "Sending {count} works to blockchain...",
            Language::French => "Envoi de {count} œuvres à la blockchain...",
            Language::Spanish => "Enviando {count} obras a la blockchain...",
            Language::German => "Sende {count} Werke an die Blockchain...",
            Language::Japanese => "{count}作品をブロックチェーンに送信中...",
            Language::Korean => "{count}개의 작품을 블록체인에 전송 중...",
            Language::Greek => "Αποστολή {count} έργων στο blockchain...",
        }),
        
        "preview.failed" => Some(match lang {
            Language::English => "Failed",
            Language::French => "Échec",
            Language::Spanish => "Fallido",
            Language::German => "Fehlgeschlagen",
            Language::Japanese => "失敗",
            Language::Korean => "실패",
            Language::Greek => "Αποτυχία",
        }),
        
        "preview.unknown_error" => Some(match lang {
            Language::English => "Unknown error",
            Language::French => "Erreur inconnue",
            Language::Spanish => "Error desconocido",
            Language::German => "Unbekannter Fehler",
            Language::Japanese => "不明なエラー",
            Language::Korean => "알 수 없는 오류",
            Language::Greek => "Άγνωστο σφάλμα",
        }),
        
        "preview.cancel" => Some(match lang {
            Language::English => "Cancel",
            Language::French => "Annuler",
            Language::Spanish => "Cancelar",
            Language::German => "Abbrechen",
            Language::Japanese => "キャンセル",
            Language::Korean => "취소",
            Language::Greek => "Ακύρωση",
        }),
        
        "preview.iswc" => Some(match lang {
            Language::English => "ISWC",
            Language::French => "ISWC",
            Language::Spanish => "ISWC",
            Language::German => "ISWC",
            Language::Japanese => "ISWC",
            Language::Korean => "ISWC",
            Language::Greek => "ISWC",
        }),
        
        "preview.creators" => Some(match lang {
            Language::English => "Creators",
            Language::French => "Créateurs",
            Language::Spanish => "Creadores",
            Language::German => "Urheber",
            Language::Japanese => "クリエイター",
            Language::Korean => "창작자",
            Language::Greek => "Δημιουργοί",
        }),
        
        "preview.no_details" => Some(match lang {
            Language::English => "Details not available",
            Language::French => "Détails non disponibles",
            Language::Spanish => "Detalles no disponibles",
            Language::German => "Details nicht verfügbar",
            Language::Japanese => "詳細情報なし",
            Language::Korean => "세부 정보 없음",
            Language::Greek => "Λεπτομέρειες μη διαθέσιμες",
        }),
        
        "preview.estimated_cost" => Some(match lang {
            Language::English => "Estimated cost",
            Language::French => "Coût estimé",
            Language::Spanish => "Costo estimado",
            Language::German => "Geschätzte Kosten",
            Language::Japanese => "推定コスト",
            Language::Korean => "예상 비용",
            Language::Greek => "Εκτιμώμενο κόστος",
        }),
        
        "preview.sign_send" => Some(match lang {
            Language::English => "Sign & Send",
            Language::French => "Signer & Envoyer",
            Language::Spanish => "Firmar y Enviar",
            Language::German => "Signieren & Senden",
            Language::Japanese => "署名して送信",
            Language::Korean => "서명 및 전송",
            Language::Greek => "Υπογραφή & Αποστολή",
        }),
        
        "preview.connect_wallet" => Some(match lang {
            Language::English => "Connect your wallet",
            Language::French => "Connectez votre wallet",
            Language::Spanish => "Conecte su billetera",
            Language::German => "Wallet verbinden",
            Language::Japanese => "ウォレットを接続",
            Language::Korean => "지갑 연결",
            Language::Greek => "Συνδέστε το πορτοφόλι",
        }),
        
        _ => None,
    }
}

