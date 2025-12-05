/**
 * Blockchain integration with Allfeat using @allfeat/client (official SDK)
 * 
 * Uses all SDK features:
 * - MelodieClient for connection
 * - signAndSend with callback for finalization
 * - dispatchError handling
 * - Balance checking
 * - Proper disconnection
 */

import { AllfeatProvider, MelodieClient } from 'https://cdn.jsdelivr.net/npm/@allfeat/client@0.2.2/+esm';
import { web3Enable, web3FromAddress } from 'https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.53.1/+esm';

let web3Initialized = false;
let cachedClient = null;
let cachedRpcUrl = null;

/**
 * Convert IPI numbers to BigInt (SDK requirement)
 */
function convertIpiToBigInt(work) {
    return {
        ...work,
        creators: (work.creators || []).map(creator => ({
            ...creator,
            id: convertPartyIdToBigInt(creator.id)
        }))
    };
}

function convertPartyIdToBigInt(id) {
    if (!id || !id.type) return id;
    
    if (id.type === 'Ipi') {
        return { type: 'Ipi', value: BigInt(id.value) };
    }
    if (id.type === 'Both') {
        return {
            type: 'Both',
            value: {
                ipi: BigInt(id.value.ipi),
                isni: id.value.isni
            }
        };
    }
    return id;
}

async function ensureWeb3Enabled() {
    if (!web3Initialized) {
        console.log('🔌 Initializing web3...');
        const extensions = await web3Enable('Mass Load');
        if (extensions.length === 0) {
            throw new Error('No wallet extension found. Please install SubWallet, Polkadot.js or Talisman.');
        }
        console.log(`✅ Web3 initialized with ${extensions.length} extension(s)`);
        web3Initialized = true;
    }
}

async function getClient(rpcUrl) {
    if (cachedClient && cachedRpcUrl === rpcUrl) {
        return cachedClient;
    }
    
    // Disconnect old client if different URL
    if (cachedClient && cachedRpcUrl !== rpcUrl) {
        try {
            await cachedClient.disconnect();
        } catch (e) {
            console.warn('Disconnect failed:', e);
        }
    }
    
    console.log('🔌 Connecting to blockchain:', rpcUrl);
    const provider = new AllfeatProvider(rpcUrl);
    cachedClient = await MelodieClient.create(provider);
    cachedRpcUrl = rpcUrl;
    console.log('✅ Connected to blockchain');
    
    return cachedClient;
}

/**
 * Check wallet balance before transaction
 */
async function checkBalance(client, address) {
    try {
        const balance = await client.getBalanceOf(address);
        const balanceFeat = Number(balance) / 10 ** 12;
        console.log(`💰 Balance: ${balanceFeat.toFixed(4)} FEAT`);
        
        if (balanceFeat < 1) {
            console.warn('⚠️ Warning: Low balance! Transaction may fail.');
        }
        return balanceFeat;
    } catch (e) {
        console.warn('Could not check balance:', e.message);
        return null;
    }
}

/**
 * Submit a batch of musical works with proper finalization waiting
 */
export async function submitMusicalWorksBatch(rpcUrl, worksJson, walletAddress) {
    try {
        const works = JSON.parse(worksJson);
        console.log(`📤 Submitting batch of ${works.length} musical works...`);
        console.log('   RPC:', rpcUrl);
        console.log('   Wallet:', walletAddress);

        const client = await getClient(rpcUrl);
        
        // Check balance
        await checkBalance(client, walletAddress);
        
        // Get wallet signer
        console.log('🔑 Getting signer from wallet...');
        await ensureWeb3Enabled();
        const injector = await web3FromAddress(walletAddress);
        
        if (!injector || !injector.signer) {
            throw new Error('Signer not available from wallet');
        }
        console.log('✅ Signer obtained');

        // Convert IPI values to BigInt
        console.log('📦 Preparing works for SDK...');
        const sdkWorks = works.map(work => convertIpiToBigInt(work));
        
        // Build calls
        const calls = sdkWorks.map(work => 
            client.tx.musicalWorks.register(work).call
        );
        console.log(`✅ ${calls.length} transactions prepared`);

        // Create batch
        console.log('📤 Creating batch transaction...');
        const batchTx = client.tx.utility.batchAll(calls);
        
        // Sign and send with callback for finalization
        console.log('✍️ Signing and submitting (waiting for finalization)...');
        
        const result = await new Promise((resolve, reject) => {
            // Timeout after 60 seconds
            const timeout = setTimeout(() => {
                reject(new Error('Transaction timeout after 60s'));
            }, 60000);
            
            batchTx.signAndSend(walletAddress, { signer: injector.signer }, (txResult) => {
                const { status, dispatchError } = txResult;
                
                console.log(`   → Status: ${status?.type || 'unknown'}`);
                
                // Wait for finalization
                if (status?.type === 'BestChainBlockIncluded' || status?.type === 'Finalized') {
                    clearTimeout(timeout);
                    
                    // Check for dispatch errors
                    if (dispatchError) {
                        let errorMsg = 'Transaction dispatch error';
                        if (dispatchError.isModule) {
                            errorMsg = `Module error: ${JSON.stringify(dispatchError.asModule)}`;
                        } else if (typeof dispatchError === 'object') {
                            errorMsg = JSON.stringify(dispatchError);
                        }
                        reject(new Error(errorMsg));
                    } else {
                        resolve({
                            blockHash: status.value?.blockHash || 'unknown',
                            status: status.type
                        });
                    }
                }
                
                // Handle errors
                if (status?.type === 'Invalid' || status?.type === 'Drop') {
                    clearTimeout(timeout);
                    reject(new Error(`Transaction ${status.type}`));
                }
            }).catch((err) => {
                clearTimeout(timeout);
                reject(err);
            });
        });
        
        console.log(`🎉 Batch finalized in block:`, result.blockHash);

        return works.map(() => ({
            hash: result.blockHash,
            success: true,
            error: null
        }));
        
    } catch (error) {
        console.error('❌ Batch submission failed:', error);
        
        let works = [];
        try {
            works = JSON.parse(worksJson);
        } catch (e) {
            works = [{}];
        }
        
        return works.map(() => ({
            hash: null,
            success: false,
            error: error.message || 'Unknown error'
        }));
    }
}

export async function submitMusicalWork(rpcUrl, workJson, walletAddress) {
    const result = await submitMusicalWorksBatch(rpcUrl, `[${workJson}]`, walletAddress);
    return result[0];
}

/**
 * Get wallet balance from blockchain
 * @param {string} rpcUrl - RPC endpoint URL
 * @param {string} walletAddress - Wallet SS58 address
 * @returns {Promise<{balance: number, formatted: string}>}
 */
export async function getWalletBalance(rpcUrl, walletAddress) {
    try {
        const client = await getClient(rpcUrl);
        const balance = await client.getBalanceOf(walletAddress);
        const balanceNum = Number(balance) / 10 ** 12;
        
        // Format with appropriate precision
        let formatted;
        if (balanceNum >= 1000) {
            formatted = balanceNum.toFixed(0);
        } else if (balanceNum >= 1) {
            formatted = balanceNum.toFixed(2);
        } else {
            formatted = balanceNum.toFixed(4);
        }
        
        console.log(`💰 Balance for ${walletAddress.slice(0,8)}...: ${formatted} MEL`);
        
        return {
            balance: balanceNum,
            formatted: formatted
        };
    } catch (e) {
        console.error('Failed to get balance:', e);
        return {
            balance: 0,
            formatted: '?'
        };
    }
}

/**
 * Disconnect from blockchain (cleanup)
 */
export async function disconnect() {
    if (cachedClient) {
        try {
            await cachedClient.disconnect();
            console.log('🔌 Disconnected from blockchain');
        } catch (e) {
            console.warn('Disconnect error:', e);
        }
        cachedClient = null;
        cachedRpcUrl = null;
    }
}
