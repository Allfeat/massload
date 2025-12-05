//! Allfeat blockchain interaction service.
//!
//! Uses @allfeat/client SDK directly via JavaScript for signing and submitting.
//! This is simpler than the prepare/sign/submit flow because the SDK handles everything.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use crate::config::BLOCKCHAIN_RPC;

/// Result of a transaction submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionResult {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
    #[serde(default)]
    pub work_results: Vec<WorkResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkResult {
    pub iswc: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Blockchain service using @allfeat/client SDK.
pub struct BlockchainService {
    rpc_url: String,
}

impl BlockchainService {
    /// Create a new service.
    pub fn new() -> Self {
        Self { 
            rpc_url: BLOCKCHAIN_RPC.to_string() 
        }
    }
    
    /// Submit works using @allfeat/client SDK with wallet signer.
    /// The SDK handles signing and submission directly.
    pub async fn submit_works(
        &self,
        works_json: Value,
        wallet_address: Option<String>,
    ) -> Result<SubmissionResult, String> {
        let address = wallet_address.ok_or("No wallet address provided")?;
        
        let works_array = works_json.as_array()
            .ok_or("Works must be an array")?;
        
        if works_array.is_empty() {
            return Err("No works to submit".to_string());
        }
        
        log::info!("📤 Submitting {} works via @allfeat/client SDK...", works_array.len());

        // Call JavaScript SDK directly - it handles signing with the wallet
        let works_str = serde_json::to_string(&works_json)
            .map_err(|e| format!("Failed to serialize works: {}", e))?;

        let promise = submit_batch_js(&self.rpc_url, &works_str, &address);
        
        let js_result = JsFuture::from(promise)
            .await
            .map_err(|e| {
                let error_msg = js_sys::Reflect::get(&e, &"message".into())
                .ok()
                .and_then(|v| v.as_string())
                    .or_else(|| e.as_string())
                    .unwrap_or_else(|| "Unknown JS error".to_string());
                format!("SDK error: {}", error_msg)
            })?;
        
        // Parse result array from JS
        let results: Vec<JsSubmitResult> = serde_wasm_bindgen::from_value(js_result)
            .map_err(|e| format!("Failed to parse SDK result: {}", e))?;

        // Check if all succeeded
        let all_success = results.iter().all(|r| r.success);
        let first_hash = results.first().and_then(|r| r.hash.clone());
        let first_error = results.iter().find(|r| !r.success).and_then(|r| r.error.clone());
        
        if all_success {
            log::info!("✅ All {} works submitted successfully!", results.len());
        } else {
            log::error!("❌ Some works failed: {:?}", first_error);
        }

        Ok(SubmissionResult {
            success: all_success,
            tx_hash: first_hash.clone(),
            block_hash: first_hash,
            error: first_error,
            work_results: results.into_iter().enumerate().map(|(i, r)| {
                let iswc = works_array.get(i)
                    .and_then(|w| w.get("iswc"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                WorkResult {
                    iswc,
                    success: r.success,
                    error: r.error,
                }
            }).collect(),
        })
    }

    /// Estimate cost for a batch of works.
    pub fn estimate_cost(&self, work_count: usize) -> String {
        let cost = work_count as f32 * 0.05;
        format!("{:.2} AFT", cost)
    }
}

/// JavaScript result from SDK submission
#[derive(Debug, Clone, Deserialize)]
struct JsSubmitResult {
    hash: Option<String>,
    success: bool,
    error: Option<String>,
}

/// Wallet balance info
#[derive(Debug, Clone, Deserialize)]
pub struct WalletBalance {
    pub balance: f64,
    pub formatted: String,
}

/// Get wallet balance from blockchain
pub async fn get_wallet_balance(wallet_address: &str) -> Result<WalletBalance, String> {
    let promise = get_balance_js(BLOCKCHAIN_RPC, wallet_address);
    
    let js_result = JsFuture::from(promise)
        .await
        .map_err(|e| {
            let error_msg = js_sys::Reflect::get(&e, &"message".into())
                .ok()
                .and_then(|v| v.as_string())
                .or_else(|| e.as_string())
                .unwrap_or_else(|| "Unknown error".to_string());
            format!("Failed to get balance: {}", error_msg)
        })?;
    
    serde_wasm_bindgen::from_value(js_result)
        .map_err(|e| format!("Failed to parse balance: {}", e))
}

/// JavaScript functions from blockchain.js
#[wasm_bindgen(module = "/src/js/blockchain.js")]
extern "C" {
    #[wasm_bindgen(js_name = "submitMusicalWorksBatch")]
    fn submit_batch_js(
        rpc_url: &str,
        works_json: &str,
        wallet_address: &str,
    ) -> js_sys::Promise;
    
    #[wasm_bindgen(js_name = "getWalletBalance")]
    fn get_balance_js(
        rpc_url: &str,
        wallet_address: &str,
    ) -> js_sys::Promise;
}
