//! Spurious Dragon network update.

use crate::{
    eip::macros::eip_set,
    eips::{eip155::Eip155, eip160::Eip160, eip161::Eip161, eip170::Eip170},
    execution::ExecutionUpgrade,
    forks::tangerine_whistle::TangerineWhistle,
    network::{NetworkUpgrade, UpgradeActivation},
};
use chains::{Mainnet, Morden};

/// Spurious Dragon network update.
pub struct SpuriousDragon;

impl ExecutionUpgrade for SpuriousDragon {
    type EipSet = eip_set!(TangerineWhistle + Eip155, Eip160, Eip161, Eip170);
}

impl NetworkUpgrade for SpuriousDragon {}

impl UpgradeActivation<Mainnet> for SpuriousDragon {
    fn block() -> u64 {
        2_675_000
    }
}

impl UpgradeActivation<Morden> for SpuriousDragon {
    fn block() -> u64 {
        1_885_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::{eip2::Eip2, eip7::Eip7};
    use asm::instruction::{Add, DelegateCall};
    use chains::{Mainnet, Morden, Ropsten};

    #[test]
    fn activation() {
        assert_eq!(SpuriousDragon::activation_block::<Mainnet>(), 2_675_000);
        assert_eq!(SpuriousDragon::activation_block::<Morden>(), 1_885_000);
        assert_eq!(SpuriousDragon::activation_block::<Ropsten>(), 0);
    }

    #[test]
    fn eip_support() {
        assert!(SpuriousDragon::includes::<Eip2>());
        assert!(SpuriousDragon::includes::<Eip7>());

        assert!(SpuriousDragon::includes::<Eip155>());
        assert!(SpuriousDragon::includes::<Eip160>());
        assert!(SpuriousDragon::includes::<Eip161>());
        assert!(SpuriousDragon::includes::<Eip170>());
    }

    #[test]
    fn instruction_support() {
        assert!(SpuriousDragon::supports_instruction::<Add>());
        assert!(SpuriousDragon::supports_instruction::<DelegateCall>());
    }
}
