//! Frontier network update.

use crate::{
    eip::macros::eip_set, eips::genesis::Genesis, execution::ExecutionUpgrade,
    network::NetworkUpgrade,
};

/// Frontier network update.
pub struct Frontier;

impl ExecutionUpgrade for Frontier {
    type EipSet = eip_set!(Genesis);
}

impl NetworkUpgrade for Frontier {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;
    use asm::instruction::{DelegateCall, Invalid, Stop};
    use chains::Mainnet;

    #[test]
    fn activation() {
        assert_eq!(Frontier::activation_block::<Mainnet>(), 0);
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
