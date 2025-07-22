//! Shanghai execution upgrade.

use crate::{
    eip::macros::eip_set,
    eips::{eip3651::Eip3651, eip3855::Eip3855, eip3860::Eip3860, eip4895::Eip4895},
    execution::ExecutionUpgrade,
    forks::paris::Paris,
};

/// Shanghai execution upgrade.
pub struct Shanghai;

impl ExecutionUpgrade for Shanghai {
    type EipSet = eip_set!(Paris + Eip3651, Eip3855, Eip3860, Eip4895);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;
    use asm::instruction::{Add, Push};

    #[test]
    fn eip_support() {
        assert!(Shanghai::includes::<Eip2>());

        assert!(Shanghai::includes::<Eip3651>());
        assert!(Shanghai::includes::<Eip3855>());
        assert!(Shanghai::includes::<Eip3860>());
        assert!(Shanghai::includes::<Eip4895>());
    }

    #[test]
    fn instruction_support() {
        assert!(Shanghai::supports_instruction::<Add>());
        assert!(Shanghai::supports_instruction::<Push<0>>());
    }
}
