//! Wallet state management.
//!
//! Provides a centralized, reactive state for wallet connection status,
//! address, balance, and other wallet-related data.

use leptos::*;
use serde::{Deserialize, Serialize};

/// Wallet connection state
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WalletState {
    /// Whether a wallet is currently connected
    pub connected: bool,
    /// The connected wallet address (if any)
    pub address: Option<String>,
    /// The wallet balance in formatted string (e.g. "123.45 MEL")
    pub balance: Option<String>,
    /// The wallet type/name (e.g. "SubWallet", "Talisman", "Polkadot.js")
    pub wallet_type: Option<String>,
}

impl WalletState {
    /// Create a new disconnected wallet state
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a connected wallet state
    pub fn connected(address: String, wallet_type: String) -> Self {
        Self {
            connected: true,
            address: Some(address),
            balance: None,
            wallet_type: Some(wallet_type),
        }
    }

    /// Check if wallet is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Get the wallet address
    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    /// Get the wallet balance
    pub fn balance(&self) -> Option<&str> {
        self.balance.as_deref()
    }

    /// Get a shortened version of the address (e.g. "5GrwV...7KvDd")
    pub fn short_address(&self) -> Option<String> {
        self.address.as_ref().map(|addr| {
            if addr.len() > 10 {
                format!("{}...{}", &addr[0..6], &addr[addr.len() - 4..])
            } else {
                addr.clone()
            }
        })
    }

    /// Disconnect the wallet
    pub fn disconnect(&mut self) {
        *self = Self::default();
    }

    /// Update the balance
    pub fn set_balance(&mut self, balance: String) {
        self.balance = Some(balance);
    }
}

/// Context key for wallet state
#[derive(Copy, Clone)]
pub struct WalletStateContext(pub RwSignal<WalletState>);

/// Provide wallet state to the component tree
pub fn provide_wallet_state() -> RwSignal<WalletState> {
    let state = create_rw_signal(WalletState::new());
    provide_context(WalletStateContext(state));
    state
}

/// Use wallet state from context
pub fn use_wallet_state() -> RwSignal<WalletState> {
    use_context::<WalletStateContext>()
        .expect("WalletStateContext not provided")
        .0
}

/// Hook to get wallet connection status
pub fn use_wallet_connected() -> Signal<bool> {
    let state = use_wallet_state();
    Signal::derive(move || state.get().connected)
}

/// Hook to get wallet address
pub fn use_wallet_address() -> Signal<Option<String>> {
    let state = use_wallet_state();
    Signal::derive(move || state.get().address)
}

/// Hook to get wallet balance
pub fn use_wallet_balance() -> Signal<Option<String>> {
    let state = use_wallet_state();
    Signal::derive(move || state.get().balance)
}

/// Hook to get short wallet address
pub fn use_wallet_short_address() -> Signal<Option<String>> {
    let state = use_wallet_state();
    Signal::derive(move || state.get().short_address())
}

/// Actions for wallet state
pub struct WalletActions {
    state: RwSignal<WalletState>,
}

impl WalletActions {
    /// Create wallet actions from state
    pub fn new(state: RwSignal<WalletState>) -> Self {
        Self { state }
    }

    /// Connect a wallet
    pub fn connect(&self, address: String, wallet_type: String) {
        self.state.update(|state| {
            *state = WalletState::connected(address, wallet_type);
        });
    }

    /// Disconnect the wallet
    pub fn disconnect(&self) {
        self.state.update(|state| {
            state.disconnect();
        });
    }

    /// Update the wallet balance
    pub fn set_balance(&self, balance: String) {
        self.state.update(|state| {
            state.set_balance(balance);
        });
    }
}

/// Hook to get wallet actions
pub fn use_wallet_actions() -> WalletActions {
    let state = use_wallet_state();
    WalletActions::new(state)
}

