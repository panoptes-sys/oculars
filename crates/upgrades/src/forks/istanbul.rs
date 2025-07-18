//! Istanbul network upgrade.

use chains::{Goerli, Kovan, Mainnet, Rinkeby, Ropsten};

use crate::{
    eip::macros::eip_set,
    eips::{
        eip1108::Eip1108, eip1344::Eip1344, eip152::Eip152, eip1884::Eip1884, eip2028::Eip2028,
        eip2200::Eip2200,
    },
    execution::ExecutionUpgrade,
    forks::petersburg::Petersburg,
    network::{NetworkUpgrade, UpgradeActivation},
};

/// Istanbul network upgrade.
pub struct Istanbul;

impl ExecutionUpgrade for Istanbul {
    type EipSet = eip_set!(
        Petersburg + Eip152,
        Eip1108,
        Eip1344,
        Eip1884,
        Eip2028,
        Eip2200
    );
}

impl NetworkUpgrade for Istanbul {}

impl UpgradeActivation<Mainnet> for Istanbul {
    fn block() -> u64 {
        9_069_000
    }
}

impl UpgradeActivation<Ropsten> for Istanbul {
    fn block() -> u64 {
        6_485_000
    }
}

impl UpgradeActivation<Kovan> for Istanbul {
    fn block() -> u64 {
        14_111_141
    }
}

impl UpgradeActivation<Rinkeby> for Istanbul {
    fn block() -> u64 {
        5_435_345
    }
}

impl UpgradeActivation<Goerli> for Istanbul {
    fn block() -> u64 {
        1_561_651
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7};
    use asm::instruction::{Add, ChainId, SelfBalance};
    use chains::{Kovan, Mainnet, Rinkeby, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(Istanbul::activation_block::<Mainnet>(), 9_069_000);
        assert_eq!(Istanbul::activation_block::<Ropsten>(), 6_485_000);
        assert_eq!(Istanbul::activation_block::<Kovan>(), 14_111_141);
        assert_eq!(Istanbul::activation_block::<Rinkeby>(), 5_435_345);
        assert_eq!(Istanbul::activation_block::<Goerli>(), 1_561_651);
    }

    #[test]
    fn eip_support() {
        assert!(Istanbul::includes::<Eip2>());
        assert!(Istanbul::includes::<Eip7>());

        assert!(Istanbul::includes::<Eip152>());
        assert!(Istanbul::includes::<Eip1108>());
        assert!(Istanbul::includes::<Eip1344>());
        assert!(Istanbul::includes::<Eip1884>());
        assert!(Istanbul::includes::<Eip2028>());
        assert!(Istanbul::includes::<Eip2200>());
    }

    #[test]
    fn instruction_support() {
        assert!(Istanbul::supports_instruction::<Add>());

        assert!(Istanbul::supports_instruction::<ChainId>());
        assert!(Istanbul::supports_instruction::<SelfBalance>());
    }
}
