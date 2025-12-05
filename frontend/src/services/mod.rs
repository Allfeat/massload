//! Backend and blockchain services.
//!
//! This module provides services for external communication:
//!
//! # Services
//!
//! - [`upload`] - CSV upload to massload backend
//! - [`wallet`] - Polkadot wallet extension integration (SubWallet, etc.)
//! - [`blockchain`] - Allfeat blockchain transaction submission
//!
//! # JavaScript Bindings
//!
//! Some services use JavaScript bindings located in `src/js/`:
//! - `wallet.js` - Web3 extension API
//! - `blockchain.js` - @allfeat/client SDK

pub mod upload;
pub mod wallet;
pub mod blockchain;

pub use upload::*;
pub use wallet::*;
pub use blockchain::*;
