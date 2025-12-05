use leptos::*;
use crate::services::wallet::PolkadotWallet;
use crate::services::blockchain::get_wallet_balance;

#[component]
pub fn Header(
    wallet_connected: ReadSignal<bool>,
    wallet_address: ReadSignal<Option<String>>,
    set_wallet_connected: WriteSignal<bool>,
    set_wallet_address: WriteSignal<Option<String>>,
) -> impl IntoView {
    // Balance state
    let (balance, set_balance) = create_signal(None::<String>);
    
    // Handler pour connexion wallet
    let on_wallet_click = move |_| {
        if !wallet_connected.get() {
            log::info!("🔑 Attempting to connect wallet...");
            
            spawn_local(async move {
                match PolkadotWallet::connect().await {
                    Ok(account) => {
                        log::info!("✅ Wallet connected: {}", account.address);
                        set_wallet_connected.set(true);
                        set_wallet_address.set(Some(account.address.clone()));
                        
                        // Fetch balance
                        match get_wallet_balance(&account.address).await {
                            Ok(bal) => {
                                log::info!("💰 Balance: {} MEL", bal.formatted);
                                set_balance.set(Some(bal.formatted));
                            }
                            Err(e) => {
                                log::warn!("Could not fetch balance: {}", e);
                                set_balance.set(Some("?".to_string()));
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("❌ Wallet connection failed: {}", e);
                    }
                }
            });
        }
    };

    view! {
        <header>
            <div class="header-left">
                <a href="#" class="logo">"ALLFEAT"</a>
                <span class="badge">
                    {move || {
                        if let Some(bal) = balance.get() {
                            format!("{} MEL", bal)
                        } else {
                            "-- MEL".to_string()
                        }
                    }}
                </span>
            </div>
            <div class="header-right">
                <div 
                    class="wallet-status" 
                    class:connected=move || wallet_connected.get()
                    on:click=on_wallet_click
                    style="cursor: pointer;"
                >
                    <span class="wallet-dot" class:connected=move || wallet_connected.get()></span>
                    <span id="walletText">
                        {move || if let Some(addr) = wallet_address.get() {
                            format!("{}...{}", &addr[0..6.min(addr.len())], &addr[addr.len().saturating_sub(4)..])
                        } else {
                            "Connect Wallet".to_string()
                        }}
                    </span>
                </div>
            </div>
        </header>
    }
}
