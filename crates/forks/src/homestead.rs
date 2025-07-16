//! Homestead hard fork.

use chains::{Mainnet, Morden};

use crate::HardForkMeta;

/// Homestead hard fork.
pub struct Homestead;

impl HardForkMeta<Mainnet> for Homestead {
    const BLOCK_NUMBER: u64 = 1_150_000;
}

impl HardForkMeta<Morden> for Homestead {
    const BLOCK_NUMBER: u64 = 494_000;
}
