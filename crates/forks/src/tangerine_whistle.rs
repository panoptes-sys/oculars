//! Tangerine Whistle network update.

use chains::Mainnet;
use eips::{Eip, IncludesEip, eip150::Eip150};

use crate::{execution::ExecutionUpgrade, homestead::Homestead, network::NetworkUpgrade};

/// Tangerine Whistle network update.
pub struct TangerineWhistle;

impl ExecutionUpgrade for TangerineWhistle {}

impl IncludesEip<Eip150> for TangerineWhistle {
    fn includes_eip() -> bool {
        true
    }
}

impl<E: Eip> IncludesEip<E> for TangerineWhistle {
    default fn includes_eip() -> bool {
        Homestead::includes::<E>()
    }
}

impl NetworkUpgrade<Mainnet> for TangerineWhistle {
    fn activation_block() -> u64 {
        2_463_000
    }
}
