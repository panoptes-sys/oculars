//! Constantinople network upgrade.

use chains::{Kovan, Mainnet, Rinkeby, Ropsten};

use crate::{
    eip::macros::eip_set,
    eips::{
        eip145::Eip145, eip1014::Eip1014, eip1052::Eip1052, eip1234::Eip1234, eip1283::Eip1283,
    },
    execution::ExecutionUpgrade,
    forks::byzantium::Byzantium,
    network::{NetworkUpgrade, UpgradeActivation},
};

/// Constantinople (Metropolis/Constantinople, Metropolis part 2) network upgrade.
pub struct Constantinople;

impl ExecutionUpgrade for Constantinople {
    type EipSet = eip_set!(Byzantium + Eip145, Eip1014, Eip1052, Eip1234, Eip1283);
}

impl NetworkUpgrade for Constantinople {}

impl UpgradeActivation<Mainnet> for Constantinople {
    fn block() -> u64 {
        7_280_000
    }
}

impl UpgradeActivation<Ropsten> for Constantinople {
    fn block() -> u64 {
        4_230_000
    }
}

impl UpgradeActivation<Kovan> for Constantinople {
    fn block() -> u64 {
        9_200_000
    }
}

impl UpgradeActivation<Rinkeby> for Constantinople {
    fn block() -> u64 {
        3_660_663
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7};
    use asm::instruction::{Add, Create2, ExtCodeHash, Sar, Shl, Shr};
    use chains::{Kovan, Mainnet, Rinkeby, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(Constantinople::activation_block::<Mainnet>(), 7_280_000);
        assert_eq!(Constantinople::activation_block::<Ropsten>(), 4_230_000);
        assert_eq!(Constantinople::activation_block::<Kovan>(), 9_200_000);
        assert_eq!(Constantinople::activation_block::<Rinkeby>(), 3_660_663);
    }

    #[test]
    fn eip_support() {
        assert!(Constantinople::includes::<Eip2>());
        assert!(Constantinople::includes::<Eip7>());

        assert!(Constantinople::includes::<Eip145>());
        assert!(Constantinople::includes::<Eip1014>());
        assert!(Constantinople::includes::<Eip1052>());
        assert!(Constantinople::includes::<Eip1234>());
        assert!(Constantinople::includes::<Eip1283>());
    }

    #[test]
    fn instruction_support() {
        assert!(Constantinople::supports_instruction::<Add>());

        assert!(Constantinople::supports_instruction::<Shl>());
        assert!(Constantinople::supports_instruction::<Shr>());
        assert!(Constantinople::supports_instruction::<Sar>());
        assert!(Constantinople::supports_instruction::<Create2>());
        assert!(Constantinople::supports_instruction::<ExtCodeHash>());
    }
}
