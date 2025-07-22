//! Byzantium network upgrade.

use crate::{
    eip::macros::eip_set,
    eips::{
        eip100::Eip100, eip140::Eip140, eip196::Eip196, eip197::Eip197, eip198::Eip198,
        eip211::Eip211, eip214::Eip214,
    },
    execution::ExecutionUpgrade,
    forks::spurious_dragon::SpuriousDragon,
    network::{NetworkUpgrade, UpgradeActivation},
};
use chains::{Mainnet, Ropsten};

/// Byzantium (Metropolis/Byzantium, Metropolis part 1) network upgrade.
pub struct Byzantium;

impl ExecutionUpgrade for Byzantium {
    type EipSet = eip_set!(
        SpuriousDragon + Eip100,
        Eip140,
        Eip196,
        Eip197,
        Eip198,
        Eip211,
        Eip214
    );
}

impl NetworkUpgrade for Byzantium {}

impl UpgradeActivation<Mainnet> for Byzantium {
    fn block() -> u64 {
        4_370_000
    }
}

impl UpgradeActivation<Ropsten> for Byzantium {
    fn block() -> u64 {
        1_700_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7};
    use asm::instruction::{Add, DelegateCall, ReturnDataCopy, ReturnDataSize, Revert, StaticCall};
    use chains::{Mainnet, Morden, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(Byzantium::activation_block::<Mainnet>(), 4_370_000);
        assert_eq!(Byzantium::activation_block::<Morden>(), 0);
        assert_eq!(Byzantium::activation_block::<Ropsten>(), 1_700_000);
    }

    #[test]
    fn eip_support() {
        assert!(Byzantium::includes::<Eip2>());
        assert!(Byzantium::includes::<Eip7>());

        assert!(Byzantium::includes::<Eip100>());
        assert!(Byzantium::includes::<Eip140>());
        assert!(Byzantium::includes::<Eip196>());
        assert!(Byzantium::includes::<Eip197>());
        assert!(Byzantium::includes::<Eip198>());
        assert!(Byzantium::includes::<Eip211>());
        assert!(Byzantium::includes::<Eip214>());
    }

    #[test]
    fn instruction_support() {
        assert!(Byzantium::supports_instruction::<Add>());
        assert!(Byzantium::supports_instruction::<DelegateCall>());

        assert!(Byzantium::supports_instruction::<Revert>());
        assert!(Byzantium::supports_instruction::<ReturnDataSize>());
        assert!(Byzantium::supports_instruction::<ReturnDataCopy>());
        assert!(Byzantium::supports_instruction::<StaticCall>());
    }
}
