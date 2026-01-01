//! Wallet required confirmation dialog.
//! Displayed when the user tries to validate a file without being connected.

use leptos::*;
use allfeat_ui::components::IconWallet;
use crate::i18n::{use_language, Language};

/// Close icon SVG (X)
fn close_icon() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>"##
}

#[component]
pub fn WalletRequiredDialog(
    #[prop(into)] show: ReadSignal<bool>,
    on_connect_click: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let lang = use_language();

    let title = move || match lang.get() {
        Language::English => "Confirmation",
        Language::French => "Confirmation",
        Language::Spanish => "Confirmación",
        Language::German => "Bestätigung",
        Language::Japanese => "確認",
        Language::Korean => "확인",
        Language::Greek => "Επιβεβαίωση",
    };

    let message = move || match lang.get() {
        Language::English => "Please connect a wallet and select the account to use to continue:",
        Language::French => "Veuillez connecter un portefeuille et sélectionner le compte à utiliser pour continuer :",
        Language::Spanish => "Por favor, conecte una billetera y seleccione la cuenta a utilizar para continuar:",
        Language::German => "Bitte verbinden Sie eine Wallet und wählen Sie das zu verwendende Konto aus, um fortzufahren:",
        Language::Japanese => "続行するには、ウォレットを接続してアカウントを選択してください：",
        Language::Korean => "계속하려면 지갑을 연결하고 사용할 계정을 선택하세요:",
        Language::Greek => "Παρακαλώ συνδέστε ένα πορτοφόλι και επιλέξτε το λογαριασμό που θα χρησιμοποιήσετε για να συνεχίσετε:",
    };

    let button_text = move || match lang.get() {
        Language::English => "Connect wallet",
        Language::French => "Connecter wallet",
        Language::Spanish => "Conectar billetera",
        Language::German => "Wallet verbinden",
        Language::Japanese => "ウォレットを接続",
        Language::Korean => "지갑 연결",
        Language::Greek => "Σύνδεση πορτοφολιού",
    };

    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" on:click=move |_| on_close.call(())>
                <div class="modal-content confirmation-modal" on:click=|e| e.stop_propagation()>
                    <button 
                        class="modal-close-button" 
                        on:click=move |_| on_close.call(())
                        aria-label="Close"
                    >
                        <span inner_html=close_icon()></span>
                    </button>
                    
                    <h3 class="confirmation-title">{title}</h3>
                    <p class="confirmation-message">{message}</p>
                    <div class="confirmation-actions">
                        <button 
                            class="btn btn-primary wallet-connect-btn" 
                            on:click=move |_| {
                                on_close.call(());
                                on_connect_click.call(());
                            }
                        >
                            <span class="wallet-icon"><IconWallet/></span>
                            <span>{button_text}</span>
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

