//! London network upgrade.

use chains::{Goerli, Kovan, Mainnet, Rinkeby, Ropsten};

use crate::{
    eip::macros::eip_set,
    eips::{
        eip1559::Eip1559, eip3198::Eip3198, eip3529::Eip3529, eip3541::Eip3541, eip3554::Eip3554,
    },
    execution::ExecutionUpgrade,
    forks::berlin::Berlin,
    network::{NetworkUpgrade, UpgradeActivation},
};

/// London network upgrade.
pub struct London;

impl NetworkUpgrade for London {}

impl ExecutionUpgrade for London {
    type EipSet = eip_set!(Berlin + Eip1559, Eip3198, Eip3529, Eip3541, Eip3554);
}

impl UpgradeActivation<Mainnet> for London {
    fn block() -> u64 {
        12_965_000
    }
}

impl UpgradeActivation<Kovan> for London {
    fn block() -> u64 {
        26_741_100
    }
}

impl UpgradeActivation<Rinkeby> for London {
    fn block() -> u64 {
        8_897_988
    }
}

impl UpgradeActivation<Goerli> for London {
    fn block() -> u64 {
        5_062_605
    }
}

impl UpgradeActivation<Ropsten> for London {
    fn block() -> u64 {
        10_499_401
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7};
    use asm::instruction::{Add, BaseFee};
    use chains::{Mainnet, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(London::activation_block::<Mainnet>(), 12_965_000);
        assert_eq!(London::activation_block::<Kovan>(), 26_741_100);
        assert_eq!(London::activation_block::<Rinkeby>(), 8_897_988);
        assert_eq!(London::activation_block::<Goerli>(), 5_062_605);
        assert_eq!(London::activation_block::<Ropsten>(), 10_499_401);
    }

    #[test]
    fn eip_support() {
        assert!(London::includes::<Eip2>());
        assert!(London::includes::<Eip7>());

        assert!(London::includes::<Eip1559>());
        assert!(London::includes::<Eip3198>());
        assert!(London::includes::<Eip3529>());
        assert!(London::includes::<Eip3541>());
        assert!(London::includes::<Eip3554>());
    }

    #[test]
    fn instruction_support() {
        assert!(London::supports_instruction::<Add>());

        assert!(London::supports_instruction::<BaseFee>());
    }
}
