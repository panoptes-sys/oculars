//! Ethereum chains.

/// Chain information.
pub trait ChainMeta {
    /// Chain id.
    const CHAIN_ID: u64;
    /// Network id.
    const NETWORK_ID: u64;
}

/// Main net.
pub struct Mainnet;

impl ChainMeta for Mainnet {
    const CHAIN_ID: u64 = 1;
    const NETWORK_ID: u64 = 1;
}

/// Ethereum Classic Testnet Morden.
pub struct Morden;

impl ChainMeta for Morden {
    const CHAIN_ID: u64 = 62;
    const NETWORK_ID: u64 = 2;
}
