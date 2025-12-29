//! Explorer service - Fetch on-chain MIDDS data using JS SDK bindings
//! 
//! Uses @allfeat/client via JavaScript bindings (like massload)

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Blockchain metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainMetrics {
    pub works: u32,
    pub recordings: u32,
    pub releases: u32,
    pub total: u32,
}

/// Musical work data from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicalWorkData {
    pub id: String,
    pub title: String,
    pub iswc: Option<String>,
    pub creators: Vec<CreatorData>,
    pub creation_year: Option<u16>,
    pub is_instrumental: bool,
    pub work_type: String,
    pub language: Option<String>,
    pub musical_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorData {
    pub name: String,
    pub roles: Vec<String>,
    pub id: serde_json::Value,
}

// Import JS functions from blockchain.js
#[wasm_bindgen(module = "/src/js/blockchain.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getBlockchainMetrics", catch)]
    async fn js_get_blockchain_metrics(rpc_url: &str) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name = "getAllMusicalWorks", catch)]
    async fn js_get_all_musical_works(rpc_url: &str) -> Result<JsValue, JsValue>;
}

/// Fetch blockchain metrics using JS SDK
pub async fn fetch_blockchain_metrics(rpc_url: &str) -> Result<BlockchainMetrics, String> {
    log::info!("📊 Fetching blockchain metrics via JS SDK...");
    
    let js_result = js_get_blockchain_metrics(rpc_url)
        .await
        .map_err(|e| format!("JS error: {:?}", e))?;
    
    let metrics: BlockchainMetrics = serde_wasm_bindgen::from_value(js_result)
        .map_err(|e| format!("Failed to deserialize metrics: {}", e))?;
    
    log::info!("✅ Metrics: {} works, {} recordings, {} releases", 
        metrics.works, metrics.recordings, metrics.releases);
    
    Ok(metrics)
}

/// Fetch all musical works from blockchain using JS SDK
pub async fn fetch_all_musical_works(rpc_url: &str) -> Result<Vec<MusicalWorkData>, String> {
    log::info!("🎵 Fetching all musical works via JS SDK...");
    
    let js_result = js_get_all_musical_works(rpc_url)
        .await
        .map_err(|e| format!("JS error: {:?}", e))?;
    
    let works: Vec<MusicalWorkData> = serde_wasm_bindgen::from_value(js_result)
        .map_err(|e| format!("Failed to deserialize works: {}", e))?;
    
    log::info!("✅ Fetched {} musical works", works.len());
    
    Ok(works)
}
