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
        
        "preview.title" => Some(match lang {
            Language::English => "Preview & Register",
            Language::French => "Aperçu & Enregistrement",
            Language::Spanish => "Vista Previa y Registro",
            Language::German => "Vorschau & Registrierung",
            Language::Japanese => "プレビュー＆登録",
            Language::Korean => "미리보기 및 등록",
            Language::Greek => "Προεπισκόπηση & Εγγραφή",
        }),
        
        "preview.total_works" => Some(match lang {
            Language::English => "Total works",
            Language::French => "Total des œuvres",
            Language::Spanish => "Total de obras",
            Language::German => "Gesamte Werke",
            Language::Japanese => "総作品数",
            Language::Korean => "총 작품",
            Language::Greek => "Σύνολο έργων",
        }),
        
        "preview.works_count" => Some(match lang {
            Language::English => "works",
            Language::French => "œuvres",
            Language::Spanish => "obras",
            Language::German => "Werke",
            Language::Japanese => "作品",
            Language::Korean => "작품",
            Language::Greek => "έργα",
        }),
        
        "preview.page" => Some(match lang {
            Language::English => "Page",
            Language::French => "Page",
            Language::Spanish => "Página",
            Language::German => "Seite",
            Language::Japanese => "ページ",
            Language::Korean => "페이지",
            Language::Greek => "Σελίδα",
        }),
        
        "preview.of" => Some(match lang {
            Language::English => "of",
            Language::French => "sur",
            Language::Spanish => "de",
            Language::German => "von",
            Language::Japanese => "の",
            Language::Korean => "중",
            Language::Greek => "από",
        }),
        
        "preview.items_per_page" => Some(match lang {
            Language::English => "Items per page",
            Language::French => "Éléments par page",
            Language::Spanish => "Elementos por página",
            Language::German => "Elemente pro Seite",
            Language::Japanese => "ページあたりの項目",
            Language::Korean => "페이지당 항목",
            Language::Greek => "Στοιχεία ανά σελίδα",
        }),
        
        "preview.previous" => Some(match lang {
            Language::English => "Previous",
            Language::French => "Précédent",
            Language::Spanish => "Anterior",
            Language::German => "Zurück",
            Language::Japanese => "前へ",
            Language::Korean => "이전",
            Language::Greek => "Προηγούμενο",
        }),
        
        "preview.next" => Some(match lang {
            Language::English => "Next",
            Language::French => "Suivant",
            Language::Spanish => "Siguiente",
            Language::German => "Weiter",
            Language::Japanese => "次へ",
            Language::Korean => "다음",
            Language::Greek => "Επόμενο",
        }),
        
        "preview.register_on_chain" => Some(match lang {
            Language::English => "Submit to Blockchain",
            Language::French => "Soumettre à la blockchain",
            Language::Spanish => "Enviar a la Blockchain",
            Language::German => "An Blockchain Übermitteln",
            Language::Japanese => "ブロックチェーンに提出",
            Language::Korean => "블록체인에 제출",
            Language::Greek => "Υποβολή στο Blockchain",
        }),
        
        "preview.invalid_work_data" => Some(match lang {
            Language::English => "Invalid work data",
            Language::French => "Données d'œuvre invalides",
            Language::Spanish => "Datos de obra no válidos",
            Language::German => "Ungültige Werkdaten",
            Language::Japanese => "無効な作品データ",
            Language::Korean => "유효하지 않은 작품 데이터",
            Language::Greek => "Μη έγκυρα δεδομένα έργου",
        }),
        
        // Confirmation dialog
        "preview.confirm_title" => Some(match lang {
            Language::English => "Confirm Registration",
            Language::French => "Confirmer l'Enregistrement",
            Language::Spanish => "Confirmar Registro",
            Language::German => "Registrierung Bestätigen",
            Language::Japanese => "登録を確認",
            Language::Korean => "등록 확인",
            Language::Greek => "Επιβεβαίωση Εγγραφής",
        }),
        
        "preview.confirm_message" => Some(match lang {
            Language::English => "You are about to register these works on the blockchain. This action is irreversible and will cost transaction fees.",
            Language::French => "Vous êtes sur le point d'enregistrer ces œuvres sur la blockchain. Cette action est irréversible et coûtera des frais de transaction.",
            Language::Spanish => "Está a punto de registrar estas obras en la blockchain. Esta acción es irreversible y costará tarifas de transacción.",
            Language::German => "Sie sind dabei, diese Werke auf der Blockchain zu registrieren. Diese Aktion ist unumkehrbar und kostet Transaktionsgebühren.",
            Language::Japanese => "これらの作品をブロックチェーンに登録しようとしています。この操作は元に戻せず、取引手数料がかかります。",
            Language::Korean => "이 작품들을 블록체인에 등록하려고 합니다. 이 작업은 되돌릴 수 없으며 거래 수수료가 발생합니다.",
            Language::Greek => "Πρόκειται να εγγράψετε αυτά τα έργα στο blockchain. Αυτή η ενέργεια είναι μη αναστρέψιμη και θα κοστίσει τέλη συναλλαγής.",
        }),
        
        "preview.confirm" => Some(match lang {
            Language::English => "Confirm",
            Language::French => "Confirmer",
            Language::Spanish => "Confirmar",
            Language::German => "Bestätigen",
            Language::Japanese => "確認",
            Language::Korean => "확인",
            Language::Greek => "Επιβεβαίωση",
        }),
        
        // Success message
        "preview.success_title" => Some(match lang {
            Language::English => "Registration Successful!",
            Language::French => "Enregistrement Réussi !",
            Language::Spanish => "¡Registro Exitoso!",
            Language::German => "Registrierung Erfolgreich!",
            Language::Japanese => "登録成功！",
            Language::Korean => "등록 성공!",
            Language::Greek => "Επιτυχής Εγγραφή!",
        }),
        
        "preview.works_registered" => Some(match lang {
            Language::English => "works registered on-chain",
            Language::French => "œuvres enregistrées sur la blockchain",
            Language::Spanish => "obras registradas en cadena",
            Language::German => "Werke auf Chain registriert",
            Language::Japanese => "作品がチェーンに登録されました",
            Language::Korean => "작품이 체인에 등록됨",
            Language::Greek => "έργα εγγεγραμμένα on-chain",
        }),
        
        "preview.tx_hash" => Some(match lang {
            Language::English => "Transaction Hash",
            Language::French => "Hash de Transaction",
            Language::Spanish => "Hash de Transacción",
            Language::German => "Transaktions-Hash",
            Language::Japanese => "トランザクションハッシュ",
            Language::Korean => "트랜잭션 해시",
            Language::Greek => "Hash Συναλλαγής",
        }),
        
        "preview.back_to_upload" => Some(match lang {
            Language::English => "Upload Another File",
            Language::French => "Charger un Autre Fichier",
            Language::Spanish => "Cargar Otro Archivo",
            Language::German => "Eine Andere Datei Hochladen",
            Language::Japanese => "別のファイルをアップロード",
            Language::Korean => "다른 파일 업로드",
            Language::Greek => "Ανεβάστε Άλλο Αρχείο",
        }),
        
        _ => None,
    }
}

