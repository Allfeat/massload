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

/**
 * Helper to decode bytes to string
 */
function decodeBytes(bytes) {
    if (!bytes) return null;
    // If it's a hex string starting with 0x
    if (typeof bytes === 'string' && bytes.startsWith('0x')) {
        const hex = bytes.slice(2);
        const arr = new Uint8Array(hex.length / 2);
        for (let i = 0; i < hex.length; i += 2) {
            arr[i / 2] = parseInt(hex.substr(i, 2), 16);
        }
        return new TextDecoder().decode(arr);
    }
    // If it's a number (hex), convert to hex string first
    if (typeof bytes === 'number') {
        const hex = bytes.toString(16);
        const arr = new Uint8Array(hex.length / 2);
        for (let i = 0; i < hex.length; i += 2) {
            arr[i / 2] = parseInt(hex.substr(i, 2), 16);
        }
        return new TextDecoder().decode(arr);
    }
    // If it's already a Uint8Array or array
    if (bytes instanceof Uint8Array || Array.isArray(bytes)) {
        return new TextDecoder().decode(new Uint8Array(bytes));
    }
    return String(bytes);
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
        
        // Debug: log the first work structure
        if (sdkWorks.length > 0) {
            console.log('📋 First work structure:', JSON.stringify(sdkWorks[0], (key, value) =>
                typeof value === 'bigint' ? value.toString() + 'n' : value
            , 2));
        }
        
        // Check if extrinsic exists
        if (!client.tx.musicalWorks || !client.tx.musicalWorks.register) {
            throw new Error('Extrinsic musicalWorks.register not found in runtime. Is the chain using the correct runtime version?');
        }
        
        // Build calls
        console.log('🔨 Building transaction calls...');
        const calls = sdkWorks.map((work, index) => {
            try {
                const tx = client.tx.musicalWorks.register(work);
                console.log(`  ✓ Call ${index + 1} built successfully`);
                return tx.call;
            } catch (err) {
                console.error(`  ✗ Call ${index + 1} failed:`, err);
                throw new Error(`Failed to build call for work ${index + 1}: ${err.message}`);
            }
        });
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

/**
 * Get blockchain metrics (counts of works, recordings, releases)
 * @param {string} rpcUrl - RPC endpoint URL
 * @returns {Promise<{works: number, recordings: number, releases: number, total: number}>}
 */
export async function getBlockchainMetrics(rpcUrl) {
    try {
        console.log('📊 Fetching blockchain metrics...');
        const client = await getClient(rpcUrl);
        
        // Query storage entries for each MIDDS type
        const worksEntries = await client.query.musicalWorks.middsOf.entries();
        const recordingsEntries = await client.query.recordings?.middsOf?.entries() || [];
        const releasesEntries = await client.query.releases?.middsOf?.entries() || [];
        
        const works = worksEntries.length;
        const recordings = recordingsEntries.length;
        const releases = releasesEntries.length;
        
        console.log(`✅ Metrics: ${works} works, ${recordings} recordings, ${releases} releases`);
        
        return {
            works,
            recordings,
            releases,
            total: works + recordings + releases
        };
    } catch (error) {
        console.error('❌ Failed to fetch metrics:', error);
        throw new Error(`Failed to fetch metrics: ${error.message}`);
    }
}

/**
 * Get all musical works from blockchain
 * @param {string} rpcUrl - RPC endpoint URL
 * @returns {Promise<Array<{id: string, title: string, iswc: string, creators: Array, ...}>>}
 */
export async function getAllMusicalWorks(rpcUrl) {
    try {
        console.log('🎵 Fetching all musical works...');
        const client = await getClient(rpcUrl);
        
        // Query all musical works
        const entries = await client.query.musicalWorks.middsOf.entries();
        console.log(`📦 Found ${entries.length} entries`);
        
        const works = [];
        
        let workId = 0;
        for (const [key, value] of entries) {
            try {
                // The key is just the storage key, we'll use an incrementing ID
                const id = workId++;
                
                const title = decodeBytes(value.title) || 'Untitled';
                const iswc = decodeBytes(value.iswc);
                
                // Format creators nicely
                const creators = (value.creators || []).map(creator => {
                    let name = 'Unknown';
                    if (creator.id) {
                        if (creator.id.type === 'Ipi') {
                            name = `IPI ${creator.id.value}`;
                        } else if (creator.id.type === 'Isni') {
                            name = `ISNI ${creator.id.value}`;
                        } else if (creator.id.type === 'Both') {
                            name = `IPI ${creator.id.value.ipi} / ISNI ${creator.id.value.isni}`;
                        }
                    }
                    return {
                        name,
                        roles: creator.role ? [creator.role] : [],
                        id: creator.id || {}
                    };
                });
                
                // Format work type
                let workType = 'Original';
                if (value.workType) {
                    if (typeof value.workType === 'object' && value.workType.type) {
                        workType = value.workType.type;
                    } else {
                        workType = String(value.workType);
                    }
                }
                
                console.log(`✅ Work ${id}: ${title} (${iswc || 'no ISWC'})`);
                
                works.push({
                    id: String(id),
                    title,
                    iswc,
                    creators,
                    creationYear: value.creationYear || null,
                    isInstrumental: value.instrumental === true,
                    workType,
                    language: value.language || null,
                    musicalKey: value.key || null
                });
            } catch (entryError) {
                console.warn('⚠️ Failed to parse entry:', entryError);
            }
        }
        
        console.log(`✅ Fetched ${works.length} musical works`);
        return works;
        
    } catch (error) {
        console.error('❌ Failed to fetch musical works:', error);
        throw new Error(`Failed to fetch musical works: ${error.message}`);
    }
}

/**
 * Get all recordings from blockchain
 * @param {string} rpcUrl - RPC endpoint
 * @returns {Promise<Array>} Array of recording objects
 */
export async function getAllRecordings(rpcUrl) {
    try {
        console.log('🎙️ Fetching all recordings from blockchain...');
        
        const client = await getClient(rpcUrl);
        
        // Fetch all entries from recordings.middsOf storage map
        const entries = await client.query.recordings.middsOf.entries();
        
        const recordings = [];
        let id = 0;
        
        for (const [key, value] of entries) {
            try {
                id++;
                
                const title = decodeBytes(value.title) || 'Untitled';
                const isrc = decodeBytes(value.isrc);
                const musicalWorkId = String(value.musicalWorkId || id);
                
                // Format performers
                const performers = (value.performers || []).map(performer => {
                    let name = 'Unknown';
                    if (performer.id) {
                        if (performer.id.type === 'Ipi') {
                            name = `IPI ${performer.id.value}`;
                        } else if (performer.id.type === 'Isni') {
                            name = `ISNI ${performer.id.value}`;
                        } else if (performer.id.type === 'Both') {
                            name = `IPI ${performer.id.value.ipi} / ISNI ${performer.id.value.isni}`;
                        }
                    }
                    return {
                        name,
                        id: performer.id || {}
                    };
                });
                
                console.log(`✅ Recording ${id}: ${title} (${isrc || 'no ISRC'})`);
                
                recordings.push({
                    id: String(id),
                    title,
                    isrc,
                    musicalWorkId,
                    performers,
                    durationMs: value.duration || null,
                    recordingDate: value.recordingDate || null,
                    recordingLocation: value.recordingLocation || null
                });
            } catch (entryError) {
                console.warn('⚠️ Failed to parse recording entry:', entryError);
            }
        }
        
        console.log(`✅ Fetched ${recordings.length} recordings`);
        return recordings;
        
    } catch (error) {
        console.error('❌ Failed to fetch recordings:', error);
        throw new Error(`Failed to fetch recordings: ${error.message}`);
    }
}

/**
 * Get all releases from blockchain
 * @param {string} rpcUrl - RPC endpoint
 * @returns {Promise<Array>} Array of release objects
 */
export async function getAllReleases(rpcUrl) {
    try {
        console.log('💿 Fetching all releases from blockchain...');
        
        const client = await getClient(rpcUrl);
        
        // Fetch all entries from releases.middsOf storage map
        const entries = await client.query.releases.middsOf.entries();
        
        const releases = [];
        let id = 0;
        
        for (const [key, value] of entries) {
            try {
                id++;
                
                const title = decodeBytes(value.title) || 'Untitled';
                const upc = decodeBytes(value.upc);
                
                // Format release type
                let releaseType = 'Album';
                if (value.releaseType) {
                    if (typeof value.releaseType === 'object' && value.releaseType.type) {
                        releaseType = value.releaseType.type;
                    } else {
                        releaseType = String(value.releaseType);
                    }
                }
                
                // Recording IDs
                const recordingIds = (value.recordings || []).map((rec, idx) => String(rec || idx));
                
                console.log(`✅ Release ${id}: ${title} (${upc || 'no UPC'})`);
                
                releases.push({
                    id: String(id),
                    title,
                    upc,
                    releaseType,
                    releaseDate: value.releaseDate || null,
                    label: decodeBytes(value.label) || null,
                    catalogNumber: decodeBytes(value.catalogNumber) || null,
                    recordingIds,
                    totalTracks: value.totalTracks || null
                });
            } catch (entryError) {
                console.warn('⚠️ Failed to parse release entry:', entryError);
            }
        }
        
        console.log(`✅ Fetched ${releases.length} releases`);
        return releases;
        
    } catch (error) {
        console.error('❌ Failed to fetch releases:', error);
        throw new Error(`Failed to fetch releases: ${error.message}`);
    }
}
