//! Berlin network upgrade.

use chains::{Goerli, Mainnet, Rinkeby, Ropsten};

use crate::{
    eip::macros::eip_set,
    eips::{eip2565::Eip2565, eip2718::Eip2718, eip2929::Eip2929, eip2930::Eip2930},
    execution::ExecutionUpgrade,
    forks::muir_glacier::MuirGlacier,
    network::{NetworkUpgrade, UpgradeActivation},
};

/// Berlin network upgrade.
pub struct Berlin;

impl NetworkUpgrade for Berlin {}

impl ExecutionUpgrade for Berlin {
    type EipSet = eip_set!(MuirGlacier + Eip2565, Eip2929, Eip2718, Eip2930);
}

impl UpgradeActivation<Mainnet> for Berlin {
    fn block() -> u64 {
        12_244_000
    }
}

impl UpgradeActivation<Rinkeby> for Berlin {
    fn block() -> u64 {
        8_290_928
    }
}

impl UpgradeActivation<Goerli> for Berlin {
    fn block() -> u64 {
        4_460_644
    }
}

impl UpgradeActivation<Ropsten> for Berlin {
    fn block() -> u64 {
        9_812_189
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7};
    use asm::instruction::Add;
    use chains::{Mainnet, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(Berlin::activation_block::<Mainnet>(), 12_244_000);
        assert_eq!(Berlin::activation_block::<Rinkeby>(), 8_290_928);
        assert_eq!(Berlin::activation_block::<Goerli>(), 4_460_644);
        assert_eq!(Berlin::activation_block::<Ropsten>(), 9_812_189);
    }

    #[test]
    fn eip_support() {
        assert!(Berlin::includes::<Eip2>());
        assert!(Berlin::includes::<Eip7>());

        assert!(Berlin::includes::<Eip2565>());
        assert!(Berlin::includes::<Eip2929>());
        assert!(Berlin::includes::<Eip2718>());
        assert!(Berlin::includes::<Eip2930>());
    }

    #[test]
    fn instruction_support() {
        assert!(Berlin::supports_instruction::<Add>());
    }
}
