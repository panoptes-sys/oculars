//! Frontier Thawing network update.

use crate::{
    execution::ExecutionUpgrade,
    forks::frontier::Frontier,
    network::{NetworkUpgrade, UpgradeActivation},
};
use chains::Mainnet;

/// Frontier Thawing network update.
pub struct FrontierThawing;

impl ExecutionUpgrade for FrontierThawing {
    type EipSet = <Frontier as ExecutionUpgrade>::EipSet;
}

impl NetworkUpgrade for FrontierThawing {}

impl UpgradeActivation<Mainnet> for FrontierThawing {
    fn block() -> u64 {
        200_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;
    use asm::instruction::{DelegateCall, Invalid, Stop};
    use chains::{Mainnet, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(FrontierThawing::activation_block::<Mainnet>(), 200_000);
        assert_eq!(FrontierThawing::activation_block::<Ropsten>(), 0);
    }

    #[test]
    fn eip_support() {
        assert!(!Frontier::includes::<Eip2>());
    }

    #[test]
    fn instruction_support() {
        assert!(Frontier::supports_instruction::<Stop>());
        assert!(Frontier::supports_instruction::<Invalid>());
        assert!(!Frontier::supports_instruction::<DelegateCall>());
    }
}
