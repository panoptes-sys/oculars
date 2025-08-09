//! London network upgrade.

use crate::{
    eip_set,
    eips::{
        eip1559::Eip1559, eip3198::Eip3198, eip3529::Eip3529, eip3541::Eip3541, eip3554::Eip3554,
    },
    execution::{ExecutionUpgrade, berlin::Berlin},
};

/// London network upgrade.
pub struct London;

impl ExecutionUpgrade for London {
    type EipSet = eip_set!(Berlin + Eip1559, Eip3198, Eip3529, Eip3541, Eip3554);
}

#[cfg(test)]
mod tests {
    use super::*;
    use asm::instruction::{Add, BaseFee};

    #[test]
    fn instruction_support() {
        assert!(London::supports_instruction(&Add));

        assert!(London::supports_instruction(&BaseFee));
    }
}
