//! Ethereum chains.

/// Chain information.
pub trait Chain {
    /// Chain id.
    const CHAIN_ID: u64;
    /// Network id.
    const NETWORK_ID: u64;
}

/// Ethereum Mainnet.
pub struct Mainnet;

impl Chain for Mainnet {
    const CHAIN_ID: u64 = 1;
    const NETWORK_ID: u64 = 1;
}

/// Ethereum Classic Testnet Morden.
pub struct Morden;

impl Chain for Morden {
    const CHAIN_ID: u64 = 62;
    const NETWORK_ID: u64 = 2;
}

/// Ethereum Ropsten testnet.
pub struct Ropsten;

impl Chain for Ropsten {
    const CHAIN_ID: u64 = 3;
    const NETWORK_ID: u64 = 3;
}

/// Ethereum Kovan testnet.
pub struct Kovan;

impl Chain for Kovan {
    const CHAIN_ID: u64 = 42;
    const NETWORK_ID: u64 = 42;
}

/// Ethereum Rinkeby testnet.
pub struct Rinkeby;

impl Chain for Rinkeby {
    const CHAIN_ID: u64 = 4;
    const NETWORK_ID: u64 = 4;
}

/// Ethereum Görli testnet.
pub struct Goerli;

impl Chain for Goerli {
    const CHAIN_ID: u64 = 5;
    const NETWORK_ID: u64 = 5;
}
