//! Petersburg network upgrade.

use chains::Chain;

use crate::{
    eip::macros::eip_set,
    eips::{eip145::Eip145, eip1014::Eip1014, eip1052::Eip1052, eip1234::Eip1234},
    execution::ExecutionUpgrade,
    forks::{byzantium::Byzantium, constantinople::Constantinople},
    network::{NetworkUpgrade, UpgradeActivation},
};

/// Petersburg (St. Peretsfork, Peter's Fork, Constantinople Fix) network upgrade.
pub struct Petersburg;

impl ExecutionUpgrade for Petersburg {
    type EipSet = eip_set!(Byzantium + Eip145, Eip1014, Eip1052, Eip1234);
}

impl NetworkUpgrade for Petersburg {}

impl<C: Chain> UpgradeActivation<C> for Petersburg {
    fn block() -> u64 {
        <Constantinople as UpgradeActivation<C>>::block()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7, eip1283::Eip1283};
    use asm::instruction::{Add, Create2, ExtCodeHash, Sar, Shl, Shr};
    use chains::{Kovan, Mainnet, Rinkeby, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(
            Petersburg::activation_block::<Mainnet>(),
            Constantinople::activation_block::<Mainnet>()
        );
        assert_eq!(
            Petersburg::activation_block::<Ropsten>(),
            Constantinople::activation_block::<Ropsten>()
        );
        assert_eq!(
            Petersburg::activation_block::<Kovan>(),
            Constantinople::activation_block::<Kovan>()
        );
        assert_eq!(
            Petersburg::activation_block::<Rinkeby>(),
            Constantinople::activation_block::<Rinkeby>()
        );
    }

    #[test]
    fn eip_support() {
        assert!(Petersburg::includes::<Eip2>());
        assert!(Petersburg::includes::<Eip7>());

        assert!(Petersburg::includes::<Eip145>());
        assert!(Petersburg::includes::<Eip1014>());
        assert!(Petersburg::includes::<Eip1052>());
        assert!(Petersburg::includes::<Eip1234>());
        assert!(!Petersburg::includes::<Eip1283>());
    }

    #[test]
    fn instruction_support() {
        assert!(Petersburg::supports_instruction::<Add>());

        assert!(Petersburg::supports_instruction::<Shl>());
        assert!(Petersburg::supports_instruction::<Shr>());
        assert!(Petersburg::supports_instruction::<Sar>());
        assert!(Petersburg::supports_instruction::<Create2>());
        assert!(Petersburg::supports_instruction::<ExtCodeHash>());
    }
}
