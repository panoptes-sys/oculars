//! Byzantium network upgrade.

use chains::{Mainnet, Ropsten};
use eips::{
    Eip, IncludesEip, eip100::Eip100, eip140::Eip140, eip196::Eip196, eip197::Eip197,
    eip198::Eip198, eip211::Eip211, eip214::Eip214, eip649::Eip649, eip658::Eip658,
};

use crate::{
    execution::ExecutionUpgrade, network::NetworkUpgrade, spurious_dragon::SpuriousDragon,
};

/// Byzantium (Metropolis/Byzantium, Metropolis part 1) network upgrade.
pub struct Byzantium;

impl ExecutionUpgrade for Byzantium {}

impl IncludesEip<Eip100> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip140> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip196> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip197> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip198> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip211> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip214> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip649> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl IncludesEip<Eip658> for Byzantium {
    fn includes_eip() -> bool {
        true
    }
}

impl<E: Eip> IncludesEip<E> for Byzantium {
    default fn includes_eip() -> bool {
        SpuriousDragon::includes::<E>()
    }
}

impl NetworkUpgrade<Mainnet> for Byzantium {
    fn activation_block() -> u64 {
        4_370_000
    }
}

impl NetworkUpgrade<Ropsten> for Byzantium {
    fn activation_block() -> u64 {
        1_700_000
    }
}
