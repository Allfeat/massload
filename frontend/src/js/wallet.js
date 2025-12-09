/**
 * Wallet integration with SubWallet (primary), Polkadot.js, Talisman
 * 
 * SubWallet est le wallet recommandé pour Allfeat
 * Docs: https://docs.subwallet.app/main/integration/integration-instructions
 * 
 * Compatible extensions:
 * - SubWallet: https://subwallet.app/ (RECOMMENDED)
 * - Polkadot.js: https://polkadot.js.org/extension/
 * - Talisman: https://talisman.xyz/
 */

/**
 * Get available wallets
 * @returns {Array<{name: string, key: string, available: boolean}>}
 */
export function getAvailableWallets() {
    const wallets = [
        { name: 'SubWallet', key: 'subwallet-js', available: false },
        { name: 'Talisman', key: 'talisman', available: false },
        { name: 'Polkadot JS', key: 'polkadot-js', available: false },
    ];

    if (window.injectedWeb3) {
        for (const wallet of wallets) {
            wallet.available = !!window.injectedWeb3[wallet.key];
        }
    }

    return wallets;
}

/**
 * Connect to a specific wallet
 * @param {string} walletKey - 'subwallet-js', 'talisman', or 'polkadot-js'
 * @returns {Promise<{address: string, name: string}>}
 */
export async function connectSpecificWallet(walletKey) {
    try {
        if (!window.injectedWeb3 || !window.injectedWeb3[walletKey]) {
            throw new Error(`${walletKey} extension not found`);
        }

        console.log(`✅ Connecting to ${walletKey}...`);
        const walletExtension = window.injectedWeb3[walletKey];
        
        // Enable the wallet
        const extension = await walletExtension.enable('Mass Load - Allfeat');
        
        // Get accounts
        const accounts = await extension.accounts.get();
        
        if (accounts.length === 0) {
            throw new Error('No accounts found in wallet. Please create an account first.');
        }

        console.log(`✅ ${walletKey}: ${accounts.length} account(s) found`);

        // Return the first account
        return {
            address: accounts[0].address,
            name: accounts[0].name || `${walletKey} Account`
        };
    } catch (error) {
        console.error(`❌ ${walletKey} connection failed:`, error);
        throw error;
    }
}

/**
 * Connecte le wallet et retourne le premier compte (auto-detect)
 * @returns {Promise<{address: string, name: string}>}
 */
export async function connectWallet() {
    try {
        // Méthode 1: Vérifier si SubWallet est installé (priorité)
        if (window.injectedWeb3 && window.injectedWeb3['subwallet-js']) {
            console.log('✅ SubWallet detected!');
            const subWalletExtension = window.injectedWeb3['subwallet-js'];
            
            // Activer SubWallet
            const extension = await subWalletExtension.enable('Mass Load - Allfeat');
            
            // Récupérer les comptes
            const accounts = await extension.accounts.get();
            
            if (accounts.length === 0) {
                throw new Error('No accounts found in SubWallet. Please create an account first.');
            }

            console.log(`✅ SubWallet: ${accounts.length} account(s) found`);

            // Retourner le premier compte
            return {
                address: accounts[0].address,
                name: accounts[0].name || 'SubWallet Account'
            };
        }

        // Méthode 2: Fallback sur @polkadot/extension-dapp (Polkadot.js, Talisman, etc.)
        console.log('🔌 SubWallet not found, using @polkadot/extension-dapp...');
        
        const { web3Enable, web3Accounts } = await import(
            'https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.46.6/+esm'
        );
        
        // Activer l'extension pour notre app
        const extensions = await web3Enable('Mass Load - Allfeat');
        
        if (extensions.length === 0) {
            throw new Error(
                'No Polkadot extension found.\n\n' +
                'Please install one of:\n' +
                '• SubWallet (recommended): https://subwallet.app/\n' +
                '• Polkadot.js: https://polkadot.js.org/extension/\n' +
                '• Talisman: https://talisman.xyz/'
            );
        }

        console.log(`✅ Found ${extensions.length} extension(s):`, extensions.map(e => e.name));

        // Récupérer les comptes
        const accounts = await web3Accounts();
        
        if (accounts.length === 0) {
            throw new Error('No accounts found in wallet. Please create an account first.');
        }

        console.log(`✅ Found ${accounts.length} account(s)`);

        // Retourner le premier compte
        return {
            address: accounts[0].address,
            name: accounts[0].meta.name || 'Account 1'
        };
    } catch (error) {
        console.error('❌ Wallet connection failed:', error);
        throw error;
    }
}

/**
 * Récupère tous les comptes disponibles
 * @returns {Promise<Array<{address: string, name: string}>>}
 */
export async function getAccounts() {
    try {
        // Méthode 1: SubWallet (priorité)
        if (window.injectedWeb3 && window.injectedWeb3['subwallet-js']) {
            const subWalletExtension = window.injectedWeb3['subwallet-js'];
            const extension = await subWalletExtension.enable('Mass Load - Allfeat');
            const accounts = await extension.accounts.get();
            
            return accounts.map(account => ({
                address: account.address,
                name: account.name || 'SubWallet Account'
            }));
        }

        // Méthode 2: Fallback sur @polkadot/extension-dapp
        const { web3Enable, web3Accounts } = await import(
            'https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.46.6/+esm'
        );

        // Activer l'extension
        await web3Enable('Mass Load - Allfeat');

        // Récupérer tous les comptes
        const accounts = await web3Accounts();
        
        return accounts.map(account => ({
            address: account.address,
            name: account.meta.name || 'Unknown'
        }));
    } catch (error) {
        console.error('❌ Failed to get accounts:', error);
        throw error;
    }
}

