//! Homestead network update.

use chains::{Chain, Mainnet, Morden};
use eips::{Eip, IncludesEip, eip2::Eip2, eip7::Eip7};

use crate::{execution::ExecutionUpgrade, network::NetworkUpgrade};

/// Homestead network update.
pub struct Homestead;

impl ExecutionUpgrade for Homestead {}

impl IncludesEip<Eip7> for Homestead {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip2> for Homestead {
    fn includes_eip() -> bool {
        true
    }
}

impl<E: Eip> IncludesEip<E> for Homestead {
    default fn includes_eip() -> bool {
        false
    }
}

impl NetworkUpgrade<Mainnet> for Homestead {
    fn activation_block() -> u64 {
        1_150_000
    }
}

impl NetworkUpgrade<Morden> for Homestead {
    fn activation_block() -> u64 {
        494_000
    }
}

impl<C: Chain> NetworkUpgrade<C> for Homestead {
    default fn activation_block() -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use eips::eip150::Eip150;

    use super::*;

    #[test]
    fn eip_support() {
        assert!(Homestead::includes::<Eip2>());
        assert!(Homestead::includes::<Eip7>());
        assert!(!Homestead::includes::<Eip150>());
    }
}
