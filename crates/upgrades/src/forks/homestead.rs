//! Homestead network update.

use crate::{
    eip::macros::eip_set,
    eips::{eip2::Eip2, eip7::Eip7, eip8::Eip8},
    execution::ExecutionUpgrade,
    forks::frontier_thawing::FrontierThawing,
    network::{NetworkUpgrade, UpgradeActivation},
};
use chains::{Mainnet, Morden};

/// Homestead network update.
pub struct Homestead;

impl ExecutionUpgrade for Homestead {
    type EipSet = eip_set!(FrontierThawing + Eip2, Eip7, Eip8);
}

impl NetworkUpgrade for Homestead {}

impl UpgradeActivation<Mainnet> for Homestead {
    fn block() -> u64 {
        1_150_000
    }
}

impl UpgradeActivation<Morden> for Homestead {
    fn block() -> u64 {
        494_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip150::Eip150;
    use asm::instruction::{Add, DelegateCall};
    use chains::{Mainnet, Morden, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(Homestead::activation_block::<Mainnet>(), 1_150_000);
        assert_eq!(Homestead::activation_block::<Morden>(), 494_000);
        assert_eq!(Homestead::activation_block::<Ropsten>(), 0);
    }

    #[test]
    fn eip_support() {
        assert!(Homestead::includes::<Eip2>());
        assert!(Homestead::includes::<Eip7>());
        assert!(Homestead::includes::<Eip8>());

        assert!(!Homestead::includes::<Eip150>());
    }

    #[test]
    fn instruction_support() {
        assert!(Homestead::supports_instruction::<Add>());
        assert!(Homestead::supports_instruction::<DelegateCall>());
    }
}
