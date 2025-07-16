//! Spurious Dragon network update.

use chains::{Mainnet, Morden};
use eips::{Eip, IncludesEip, eip155::Eip155, eip160::Eip160, eip161::Eip161, eip170::Eip170};

use crate::{
    execution::ExecutionUpgrade, network::NetworkUpgrade, tangerine_whistle::TangerineWhistle,
};

/// Spurious Dragon network update.
pub struct SpuriousDragon;

impl ExecutionUpgrade for SpuriousDragon {}

impl IncludesEip<Eip155> for SpuriousDragon {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip160> for SpuriousDragon {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip161> for SpuriousDragon {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip170> for SpuriousDragon {
    fn includes_eip() -> bool {
        true
    }
}

impl<E: Eip> IncludesEip<E> for SpuriousDragon {
    default fn includes_eip() -> bool {
        TangerineWhistle::includes::<E>()
    }
}

impl NetworkUpgrade<Mainnet> for SpuriousDragon {
    fn activation_block() -> u64 {
        2_675_000
    }
}

impl NetworkUpgrade<Morden> for SpuriousDragon {
    fn activation_block() -> u64 {
        1_885_000
    }
}
