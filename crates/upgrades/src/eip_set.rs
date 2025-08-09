//! A set of EIPs.

use asm::{AssemblyInstruction, Mnemonic};

use crate::eip::Eip;

/// A set of EIPs.
pub trait EipSet {
    /// Returns [`true`] if this set of EIPs support a [`Mnemonic`].
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{eip_set, eip_set::EipSet, eips::{eip1014::Eip1014, eip7::Eip7}};
    /// # use asm::Mnemonic;
    /// type A = eip_set!(Eip7, Eip1014);
    /// assert!(A::supports_mnemonic(Mnemonic::DELEGATECALL));
    /// assert!(A::supports_mnemonic(Mnemonic::CREATE2));
    /// ```
    #[must_use]
    #[inline]
    fn supports_mnemonic(_mnemonic: Mnemonic) -> bool {
        false
    }

    /// Returns [`true`] if this set of EIPs support an instruction.
    ///
    /// # Example
    /// ```
    /// # use oculars_upgrades::{eip_set, eip_set::EipSet, eips::{eip1014::Eip1014, eip7::Eip7}};
    /// # use asm::instruction::*;
    /// type A = eip_set!(Eip7, Eip1014);
    /// assert!(A::supports_instruction(&DelegateCall));
    /// assert!(A::supports_instruction(&Create2));
    /// ```
    #[must_use]
    #[inline]
    fn supports_instruction<I: AssemblyInstruction>(instruction: &I) -> bool {
        match instruction.mnemonic() {
            Some(mnemonic) => Self::supports_mnemonic(mnemonic),
            None => false,
        }
    }
}

impl EipSet for () {}

impl<A: Eip, B: EipSet> EipSet for (A, B) {
    #[inline]
    fn supports_mnemonic(mnemonic: Mnemonic) -> bool {
        A::introduced_mnemonic(mnemonic) || B::supports_mnemonic(mnemonic)
    }
}

/// EIP set macros.
pub mod macros {
    /// Creates a new EIP list.
    #[macro_export]
    macro_rules! eip_set {
        ($a: ident) => {
            ($a, ())
        };
        ($a: ident, $($b: ident),+) => {
            ($a, $crate::eip_set!($($b),+))
        };
        ($upgrade: ident + $a: ident)=> {
            ($a, <$upgrade as $crate::execution::ExecutionUpgrade>::EipSet)
        };
        ($upgrade: ident + $a: ident, $($rest: ident),+) => {
            ($a, eip_set!($upgrade + $($rest),+))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        eip_set,
        eips::{eip2::Eip2, eip7::Eip7},
    };

    #[test]
    fn eip_set_mnemonic_support() {
        type A = eip_set!(Eip2, Eip7);
        assert!(A::supports_mnemonic(Mnemonic::DELEGATECALL));
        assert!(!A::supports_mnemonic(Mnemonic::STOP));
    }
}
