//! Frontier network update.

use chains::Chain;
use eips::{Eip, IncludesEip};

use crate::{execution::ExecutionUpgrade, network::NetworkUpgrade};

/// Frontier network update.
pub struct Frontier;

impl ExecutionUpgrade for Frontier {}

impl<E: Eip> IncludesEip<E> for Frontier {
    fn includes_eip() -> bool {
        false
    }
}

impl<C: Chain> NetworkUpgrade<C> for Frontier {
    fn activation_block() -> u64 {
        0
    }
}
