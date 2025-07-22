//! Muir Glacier network upgrade.

use chains::{Mainnet, Ropsten};

use crate::{
    eip::macros::eip_set,
    eips::eip2384::Eip2384,
    execution::ExecutionUpgrade,
    forks::istanbul::Istanbul,
    network::{NetworkUpgrade, UpgradeActivation},
};

/// Muir Glacier network upgrade.
pub struct MuirGlacier;

impl ExecutionUpgrade for MuirGlacier {
    type EipSet = eip_set!(Istanbul + Eip2384);
}

impl NetworkUpgrade for MuirGlacier {}

impl UpgradeActivation<Mainnet> for MuirGlacier {
    fn block() -> u64 {
        9_200_000
    }
}

impl UpgradeActivation<Ropsten> for MuirGlacier {
    fn block() -> u64 {
        7_117_117
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7};
    use asm::instruction::{Add, ChainId, SelfBalance};
    use chains::{Mainnet, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(MuirGlacier::activation_block::<Mainnet>(), 9_200_000);
        assert_eq!(MuirGlacier::activation_block::<Ropsten>(), 7_117_117);
    }

    #[test]
    fn eip_support() {
        assert!(MuirGlacier::includes::<Eip2>());
        assert!(MuirGlacier::includes::<Eip7>());

        assert!(MuirGlacier::includes::<Eip2384>());
    }

    #[test]
    fn instruction_support() {
        assert!(MuirGlacier::supports_instruction::<Add>());

        assert!(MuirGlacier::supports_instruction::<ChainId>());
        assert!(MuirGlacier::supports_instruction::<SelfBalance>());
    }
}
