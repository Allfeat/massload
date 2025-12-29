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

// =============================================================================
// Common structures
// =============================================================================

/// Creator/Contributor with Party ID (IPI/ISNI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorData {
    pub name: String,
    pub roles: Vec<String>,
    pub id: serde_json::Value, // PartyId enum serialized
}

/// Performer/Artist data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformerData {
    pub name: String,
    pub id: serde_json::Value, // PartyId enum serialized
}

// =============================================================================
// MIDDS Type: Musical Work
// =============================================================================

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

// =============================================================================
// MIDDS Type: Recording
// =============================================================================

/// Recording data from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingData {
    pub id: String,
    pub title: String,
    pub isrc: Option<String>,
    pub musical_work_id: String, // Reference to MusicalWork
    pub performers: Vec<PerformerData>,
    pub duration_ms: Option<u32>,
    pub recording_date: Option<String>, // ISO date
    pub recording_location: Option<String>,
}

// =============================================================================
// MIDDS Type: Release
// =============================================================================

/// Release data from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseData {
    pub id: String,
    pub title: String,
    pub upc: Option<String>,
    pub release_type: String, // "Album", "Single", "EP", etc.
    pub release_date: Option<String>, // ISO date
    pub label: Option<String>,
    pub catalog_number: Option<String>,
    pub recording_ids: Vec<String>, // References to Recordings
    pub total_tracks: Option<u16>,
}

// =============================================================================
// Generic MIDDS Item wrapper
// =============================================================================

/// Generic MIDDS item that can be Work, Recording, or Release
#[derive(Debug, Clone)]
pub enum MiddsItem {
    Work(MusicalWorkData),
    Recording(RecordingData),
    Release(ReleaseData),
}

impl MiddsItem {
    /// Get the item ID
    pub fn id(&self) -> &str {
        match self {
            MiddsItem::Work(w) => &w.id,
            MiddsItem::Recording(r) => &r.id,
            MiddsItem::Release(r) => &r.id,
        }
    }
    
    /// Get the item title
    pub fn title(&self) -> &str {
        match self {
            MiddsItem::Work(w) => &w.title,
            MiddsItem::Recording(r) => &r.title,
            MiddsItem::Release(r) => &r.title,
        }
    }
    
    /// Get the MIDDS type name
    pub fn type_name(&self) -> &'static str {
        match self {
            MiddsItem::Work(_) => "MusicalWork",
            MiddsItem::Recording(_) => "Recording",
            MiddsItem::Release(_) => "Release",
        }
    }
}

// Import JS functions from blockchain.js
#[wasm_bindgen(module = "/src/js/blockchain.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getBlockchainMetrics", catch)]
    async fn js_get_blockchain_metrics(rpc_url: &str) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name = "getAllMusicalWorks", catch)]
    async fn js_get_all_musical_works(rpc_url: &str) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name = "getAllRecordings", catch)]
    async fn js_get_all_recordings(rpc_url: &str) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name = "getAllReleases", catch)]
    async fn js_get_all_releases(rpc_url: &str) -> Result<JsValue, JsValue>;
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

/// Fetch all recordings from blockchain using JS SDK
pub async fn fetch_all_recordings(rpc_url: &str) -> Result<Vec<RecordingData>, String> {
    log::info!("🎙️ Fetching all recordings via JS SDK...");
    
    let js_result = js_get_all_recordings(rpc_url)
        .await
        .map_err(|e| format!("JS error: {:?}", e))?;
    
    let recordings: Vec<RecordingData> = serde_wasm_bindgen::from_value(js_result)
        .map_err(|e| format!("Failed to deserialize recordings: {}", e))?;
    
    log::info!("✅ Fetched {} recordings", recordings.len());
    
    Ok(recordings)
}

/// Fetch all releases from blockchain using JS SDK
pub async fn fetch_all_releases(rpc_url: &str) -> Result<Vec<ReleaseData>, String> {
    log::info!("💿 Fetching all releases via JS SDK...");
    
    let js_result = js_get_all_releases(rpc_url)
        .await
        .map_err(|e| format!("JS error: {:?}", e))?;
    
    let releases: Vec<ReleaseData> = serde_wasm_bindgen::from_value(js_result)
        .map_err(|e| format!("Failed to deserialize releases: {}", e))?;
    
    log::info!("✅ Fetched {} releases", releases.len());
    
    Ok(releases)
}
