//! Network context for blockchain RPC selection
//! Allows users to switch between testnet and devnet dynamically

use leptos::*;
use serde::{Deserialize, Serialize};

const NETWORK_STORAGE_KEY: &str = "allfeat_network";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Devnet,
    Melodie,
}

impl Network {
    pub fn rpc_url(&self) -> &'static str {
        match self {
            Network::Devnet => "wss://dev.rpc.allfeat.org",
            Network::Melodie => "wss://melodie-rpc.allfeat.io",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Network::Devnet => "Devnet",
            Network::Melodie => "Melodie",
        }
    }

    pub fn all() -> [Network; 2] {
        [Network::Devnet, Network::Melodie]
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Melodie
    }
}

/// Load network from localStorage
fn load_network_from_storage() -> Network {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(value)) = storage.get_item(NETWORK_STORAGE_KEY) {
                if let Ok(network) = serde_json::from_str(&value) {
                    return network;
                }
            }
        }
    }
    Network::default()
}

/// Save network to localStorage
fn save_network_to_storage(network: Network) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(json) = serde_json::to_string(&network) {
                let _ = storage.set_item(NETWORK_STORAGE_KEY, &json);
            }
        }
    }
}

/// Provide network context to the app
pub fn provide_network_context() {
    let initial_network = load_network_from_storage();
    let (network, set_network) = create_signal(initial_network);
    
    // Save to localStorage whenever it changes
    create_effect(move |_| {
        let current = network.get();
        save_network_to_storage(current);
    });
    
    provide_context(network);
    provide_context(set_network);
}

/// Get current network (read-only)
pub fn use_network() -> ReadSignal<Network> {
    use_context::<ReadSignal<Network>>()
        .expect("Network context not provided. Call provide_network_context() in your root component.")
}

/// Get network setter
pub fn use_set_network() -> WriteSignal<Network> {
    use_context::<WriteSignal<Network>>()
        .expect("Network context not provided. Call provide_network_context() in your root component.")
}

/// Get current RPC URL (reactive)
pub fn use_rpc_url() -> Memo<String> {
    let network = use_network();
    create_memo(move |_| network.get().rpc_url().to_string())
}

