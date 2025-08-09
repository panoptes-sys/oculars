//! Ethereum Improvement Proposals.

use asm::{AssemblyInstruction, Mnemonic};

/// An Ethereum Improvement Proposal.
pub trait Eip {
    /// EIP number.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{eips::eip7::Eip7, eip::{Eip}};
    /// assert_eq!(Eip7::NUMBER, 7);
    /// ```
    const NUMBER: u32;

    /// Returns [`true`] if this EIP introduced a new [`Mnemonic`].
    /// ```
    /// # use oculars_upgrades::{eips::eip7::Eip7, eip::{Eip}};
    /// # use asm::Mnemonic;
    /// assert!(Eip7::introduced_mnemonic(Mnemonic::DELEGATECALL));
    /// assert!(!Eip7::introduced_mnemonic(Mnemonic::GAS));
    /// ```
    #[must_use]
    #[inline]
    fn introduced_mnemonic(_mnemonic: Mnemonic) -> bool {
        false
    }

    /// Returns [`true`] if this EIP introduced a new instruction.
    /// ```
    /// # use oculars_upgrades::{eips::eip7::Eip7, eip::{Eip}};
    /// # use asm::instruction::*;
    /// assert!(Eip7::introduced_instruction(&DelegateCall));
    /// assert!(Eip7::introduced_instruction(&Instruction::DelegateCall(DelegateCall)));
    /// assert!(!Eip7::introduced_instruction(&Gas));
    /// assert!(!Eip7::introduced_instruction(&Unknown::new(0xF)));
    /// ```
    #[must_use]
    #[inline]
    fn introduced_instruction<I: AssemblyInstruction>(instruction: &I) -> bool {
        match instruction.mnemonic() {
            Some(mnemonic) => Self::introduced_mnemonic(mnemonic),
            None => false,
        }
    }
}

/// EIP helper macros.
pub mod macros {
    /// Checks if `mnemonic` is equal to any of the `introduced` mnemonics.
    macro_rules! introduced_mnemonics {
        ($mnemonic: ident, $($introduced: ident),+) => {
            $($mnemonic == asm::Mnemonic::$introduced)||+
        };
    }

    pub(crate) use introduced_mnemonics;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eip::macros::introduced_mnemonics;
    use asm::instruction::{Add, Stop};

    #[test]
    fn eip_instruction_introduction() {
        struct EipThatIntroducesStop;

        impl Eip for EipThatIntroducesStop {
            const NUMBER: u32 = 1;

            fn introduced_mnemonic(mnemonic: Mnemonic) -> bool {
                mnemonic == Mnemonic::STOP
            }
        }

        assert!(EipThatIntroducesStop::introduced_mnemonic(Mnemonic::STOP));
        assert!(!EipThatIntroducesStop::introduced_mnemonic(Mnemonic::ADD));

        assert!(EipThatIntroducesStop::introduced_instruction(&Stop));
        assert!(!EipThatIntroducesStop::introduced_instruction(&Add));
    }

    #[test]
    fn eip_multiple_instruction_introduction() {
        struct EipThatIntroducesStopAndAdd;

        impl Eip for EipThatIntroducesStopAndAdd {
            const NUMBER: u32 = 1;

            fn introduced_mnemonic(mnemonic: Mnemonic) -> bool {
                mnemonic == Mnemonic::STOP || mnemonic == Mnemonic::ADD
            }
        }

        assert!(EipThatIntroducesStopAndAdd::introduced_mnemonic(
            Mnemonic::STOP
        ));
        assert!(EipThatIntroducesStopAndAdd::introduced_mnemonic(
            Mnemonic::ADD
        ));

        assert!(EipThatIntroducesStopAndAdd::introduced_instruction(&Stop));
        assert!(EipThatIntroducesStopAndAdd::introduced_instruction(&Add));
    }

    #[test]
    fn introduced_mnemonics_macro_works() {
        let m = Mnemonic::STOP;

        assert!(introduced_mnemonics!(m, STOP, GAS));
        assert!(!introduced_mnemonics!(m, GAS));
    }
}
