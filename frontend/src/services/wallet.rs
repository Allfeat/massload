//! Wrapper pour SubWallet et autres extensions Polkadot.js compatibles

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// Informations sur un compte wallet
#[derive(Debug, Clone)]
pub struct WalletAccount {
    pub address: String,
    pub name: Option<String>,
}

/// Wrapper Rust pour la connexion au wallet (SubWallet, Polkadot.js, Talisman, etc.)
pub struct PolkadotWallet;

impl PolkadotWallet {
    /// Vérifie si une extension compatible est installée
    pub fn is_available() -> bool {
        let window = web_sys::window().expect("no global window");
        
        // Vérifier si injectedWeb3 existe
        let has_injected = js_sys::Reflect::get(&window, &JsValue::from_str("injectedWeb3"))
            .map(|v| !v.is_null() && !v.is_undefined())
            .unwrap_or(false);
        
        if has_injected {
            log::info!("✅ Polkadot extension detected");
        } else {
            log::warn!("⚠️  No Polkadot extension found");
        }
        
        has_injected
    }

    /// Connecte le wallet et retourne le premier compte disponible
    /// Compatible avec SubWallet, Polkadot.js, Talisman
    pub async fn connect() -> Result<WalletAccount, String> {
        if !Self::is_available() {
            return Err(
                "No Polkadot extension found. Please install SubWallet, Polkadot.js or Talisman extension.".to_string()
            );
        }

        log::info!("🔌 Connecting to wallet...");

        // Appeler la fonction JS
        let promise = connect_wallet();
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| format!("Failed to connect wallet: {:?}", e))?;

        // Parser le résultat
        let address = js_sys::Reflect::get(&result, &JsValue::from_str("address"))
            .map_err(|e| format!("Failed to get address: {:?}", e))?
            .as_string()
            .ok_or_else(|| "Address is not a string".to_string())?;

        let name = js_sys::Reflect::get(&result, &JsValue::from_str("name"))
            .ok()
            .and_then(|v| v.as_string());

        log::info!("✅ Connected to wallet: {}", address);

        Ok(WalletAccount { address, name })
    }

    /// Récupère tous les comptes disponibles
    pub async fn get_accounts() -> Result<Vec<WalletAccount>, String> {
        if !Self::is_available() {
            return Err("No Polkadot extension found".to_string());
        }

        let promise = get_accounts();
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| format!("Failed to get accounts: {:?}", e))?;

        // Convertir le résultat en Vec
        let array = js_sys::Array::from(&result);
        let mut accounts = Vec::new();

        for i in 0..array.length() {
            let account_obj = array.get(i);
            
            if let Some(address) = js_sys::Reflect::get(&account_obj, &JsValue::from_str("address"))
                .ok()
                .and_then(|v| v.as_string())
            {
                let name = js_sys::Reflect::get(&account_obj, &JsValue::from_str("name"))
                    .ok()
                    .and_then(|v| v.as_string());
                
                accounts.push(WalletAccount { address, name });
            }
        }

        Ok(accounts)
    }
}

/// Import des fonctions JavaScript depuis wallet.js
#[wasm_bindgen(module = "/src/js/wallet.js")]
extern "C" {
    #[wasm_bindgen(js_name = "connectWallet")]
    fn connect_wallet() -> js_sys::Promise;
    
    #[wasm_bindgen(js_name = "getAccounts")]
    fn get_accounts() -> js_sys::Promise;
}
