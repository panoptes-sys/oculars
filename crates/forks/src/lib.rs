//! Ethereum forks.

// pub enum HardFork {
//     Frontier,
//     FrontierThawing,
//     Homestead,
//     DAO,
//     TangerineWhistle,
//     SpuriousDragon,
//     Byzantium,
//     Constantinople,
//     Istanbul,
//     MuirGlacier,
//     StakingDeposit,
//     Beacon,
//     Berlin,
//     London,
//     Altair,
//     ArrowGlacier,
//     GrayGlacier,
//     Bellatrix,
//     Paris,
//     Shappela,
//     Dencun,
//     Pectra,
// }

// impl HardFork {
//     pub fn latest() -> Self {
//         Self::Pectra
//     }
// }

pub mod frontier;
pub mod frontier_thawing;
pub mod homestead;

use chains::{ChainMeta, Mainnet, Morden};

struct Homestead;

impl HardForkMeta<Mainnet> for Homestead {
    const BLOCK_NUMBER: u64 = 1_150_000;
}

impl HardForkMeta<Morden> for Homestead {
    const BLOCK_NUMBER: u64 = 1_150_000;
}

pub trait HardForkMeta<C: ChainMeta> {
    const BLOCK_NUMBER: u64;
}
