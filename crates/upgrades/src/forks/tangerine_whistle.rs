//! Tangerine Whistle network update.

use crate::{
    eips::eip150::Eip150,
    execution::ExecutionUpgrade,
    forks::homestead::Homestead,
    network::{NetworkUpgrade, UpgradeActivation},
};
use chains::Mainnet;

/// Tangerine Whistle network update.
pub struct TangerineWhistle;

impl ExecutionUpgrade for TangerineWhistle {
    type EipSet = (Eip150, <Homestead as ExecutionUpgrade>::EipSet);
}

impl NetworkUpgrade for TangerineWhistle {}

impl UpgradeActivation<Mainnet> for TangerineWhistle {
    fn block() -> u64 {
        2_463_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;
    use asm::instruction::{Add, DelegateCall};
    use chains::{Mainnet, Morden, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(TangerineWhistle::activation_block::<Mainnet>(), 2_463_000);
        assert_eq!(TangerineWhistle::activation_block::<Morden>(), 0);
        assert_eq!(TangerineWhistle::activation_block::<Ropsten>(), 0);
    }

    #[test]
    fn eip_support() {
        assert!(TangerineWhistle::includes::<Eip2>());
        assert!(TangerineWhistle::includes::<Eip150>());
    }

    #[test]
    fn instruction_support() {
        assert!(TangerineWhistle::supports_instruction::<Add>());
        assert!(TangerineWhistle::supports_instruction::<DelegateCall>());
    }
}
