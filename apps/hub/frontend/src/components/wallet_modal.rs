//! Wallet selection modal component
//! Matches register.allfeat.org wallet selection UI

use leptos::*;
use crate::i18n::{use_language, Language};

/// Available wallet types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalletType {
    SubWallet,
    Talisman,
    PolkadotJs,
}

impl WalletType {
    pub fn name(&self) -> &'static str {
        match self {
            WalletType::SubWallet => "SubWallet",
            WalletType::Talisman => "Talisman",
            WalletType::PolkadotJs => "Polkadot JS",
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (WalletType::SubWallet, Language::English) => "SubWallet browser extension",
            (WalletType::SubWallet, Language::French) => "Extension de navigateur SubWallet",
            (WalletType::SubWallet, Language::Spanish) => "Extensión de navegador SubWallet",
            (WalletType::SubWallet, Language::German) => "SubWallet Browser-Erweiterung",
            (WalletType::SubWallet, Language::Japanese) => "SubWallet ブラウザ拡張機能",
            (WalletType::SubWallet, Language::Korean) => "SubWallet 브라우저 확장",
            (WalletType::SubWallet, Language::Greek) => "Επέκταση περιηγητή SubWallet",

            (WalletType::Talisman, Language::English) => "Talisman browser extension",
            (WalletType::Talisman, Language::French) => "Extension de navigateur Talisman",
            (WalletType::Talisman, Language::Spanish) => "Extensión de navegador Talisman",
            (WalletType::Talisman, Language::German) => "Talisman Browser-Erweiterung",
            (WalletType::Talisman, Language::Japanese) => "Talisman ブラウザ拡張機能",
            (WalletType::Talisman, Language::Korean) => "Talisman 브라우저 확장",
            (WalletType::Talisman, Language::Greek) => "Επέκταση περιηγητή Talisman",

            (WalletType::PolkadotJs, Language::English) => "Polkadot JS browser extension",
            (WalletType::PolkadotJs, Language::French) => "Extension de navigateur Polkadot JS",
            (WalletType::PolkadotJs, Language::Spanish) => "Extensión de navegador Polkadot JS",
            (WalletType::PolkadotJs, Language::German) => "Polkadot JS Browser-Erweiterung",
            (WalletType::PolkadotJs, Language::Japanese) => "Polkadot JS ブラウザ拡張機能",
            (WalletType::PolkadotJs, Language::Korean) => "Polkadot JS 브라우저 확장",
            (WalletType::PolkadotJs, Language::Greek) => "Επέκταση περιηγητή Polkadot JS",
        }
    }

    pub fn logo_url(&self) -> &'static str {
        match self {
            WalletType::SubWallet => "/public/brands/subwallet.png",
            WalletType::Talisman => "/public/brands/talisman.png",
            WalletType::PolkadotJs => "/public/brands/polkadot-js.svg",
        }
    }
}

/// Close icon SVG
fn close_icon() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>"##
}

#[component]
pub fn WalletModal(
    #[prop(into)] show: Signal<bool>,
    on_select: Callback<WalletType>,
    on_close: Callback<()>,
) -> impl IntoView {
    let lang = use_language();

    let modal_title = move || match lang.get() {
        Language::English => "Connect your wallet",
        Language::French => "Connectez votre portefeuille",
        Language::Spanish => "Conecte su billetera",
        Language::German => "Verbinden Sie Ihre Wallet",
        Language::Japanese => "ウォレットを接続",
        Language::Korean => "지갑 연결",
        Language::Greek => "Συνδέστε το πορτοφόλι σας",
    };

    let modal_subtitle = move || match lang.get() {
        Language::English => "Select a wallet extension to connect:",
        Language::French => "Sélectionnez une extension de portefeuille à connecter :",
        Language::Spanish => "Seleccione una extensión de billetera para conectar:",
        Language::German => "Wählen Sie eine Wallet-Erweiterung zum Verbinden:",
        Language::Japanese => "接続するウォレット拡張機能を選択してください:",
        Language::Korean => "연결할 지갑 확장을 선택하세요:",
        Language::Greek => "Επιλέξτε μια επέκταση πορτοφολιού για σύνδεση:",
    };

    let close_text = move || match lang.get() {
        Language::English => "Close",
        Language::French => "Fermer",
        Language::Spanish => "Cerrar",
        Language::German => "Schließen",
        Language::Japanese => "閉じる",
        Language::Korean => "닫기",
        Language::Greek => "Κλείσιμο",
    };

    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" on:click=move |_| on_close.call(())>
                <div class="modal-content" on:click=|e| e.stop_propagation()>
                    <div class="modal-header">
                        <h2>{modal_title}</h2>
                        <p>{modal_subtitle}</p>
                    </div>

                    <div class="wallet-options">
                        {[WalletType::SubWallet, WalletType::Talisman, WalletType::PolkadotJs]
                            .iter()
                            .map(|&wallet| {
                                let current_lang = lang.get();
                                view! {
                                    <button 
                                        class="wallet-option"
                                        on:click=move |_| on_select.call(wallet)
                                    >
                                        <img 
                                            src=wallet.logo_url() 
                                            alt=wallet.name()
                                            class="wallet-logo"
                                        />
                                        <div class="wallet-info">
                                            <div class="wallet-name">{wallet.name()}</div>
                                            <div class="wallet-desc">{wallet.description(current_lang)}</div>
                                        </div>
                                    </button>
                                }
                            })
                            .collect_view()
                        }
                    </div>

                    <button class="modal-close" on:click=move |_| on_close.call(())>
                        <span inner_html=close_icon()></span>
                        <span>{close_text}</span>
                    </button>
                </div>
            </div>
        </Show>
    }
}

