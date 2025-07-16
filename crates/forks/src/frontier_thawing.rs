//! Frontier Thawing network update.

use chains::Chain;
use eips::{Eip, IncludesEip};

use crate::{execution::ExecutionUpgrade, network::NetworkUpgrade};

/// Frontier Thawing network update.
pub struct FrontierThawing;

impl ExecutionUpgrade for FrontierThawing {}

impl<E: Eip> IncludesEip<E> for FrontierThawing {
    fn includes_eip() -> bool {
        false
    }
}

impl<C: Chain> NetworkUpgrade<C> for FrontierThawing {
    fn activation_block() -> u64 {
        200_000
    }
}
