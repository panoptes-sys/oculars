//! Cancun execution upgrade.

use crate::{
    eip::macros::eip_set,
    eips::{
        eip1153::Eip1153, eip4788::Eip4788, eip4844::Eip4844, eip5656::Eip5656, eip6780::Eip6780,
        eip7516::Eip7516,
    },
    execution::ExecutionUpgrade,
    forks::shanghai::Shanghai,
};

/// Cancun execution upgrade.
pub struct Cancun;

impl ExecutionUpgrade for Cancun {
    type EipSet = eip_set!(
        Shanghai + Eip1153,
        Eip4788,
        Eip4844,
        Eip5656,
        Eip6780,
        Eip7516
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;
    use asm::instruction::{Add, BlobBaseFee, MCopy, TLoad, TStore};

    #[test]
    fn eip_support() {
        assert!(Cancun::includes::<Eip2>());

        assert!(Cancun::includes::<Eip4788>());
        assert!(Cancun::includes::<Eip4844>());
        assert!(Cancun::includes::<Eip5656>());
        assert!(Cancun::includes::<Eip6780>());
        assert!(Cancun::includes::<Eip7516>());
    }

    #[test]
    fn instruction_support() {
        assert!(Cancun::supports_instruction::<Add>());

        assert!(Cancun::supports_instruction::<TLoad>());
        assert!(Cancun::supports_instruction::<TStore>());
        assert!(Cancun::supports_instruction::<MCopy>());
        assert!(Cancun::supports_instruction::<BlobBaseFee>());
    }
}
