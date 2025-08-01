//! EVM mnemonics.

pub use crate::defs::mnemonic::Mnemonic;
use crate::fmt::forward_byte_fmt;
use std::cmp::Ordering;

impl Mnemonic {
    /// Converts mnemonic into its byte representation.
    ///
    /// # Examples
    /// ```
    /// # use oculars_asm2::Mnemonic;
    /// assert_eq!(Mnemonic::GAS.into_byte(), 0x5A);
    /// ```
    #[must_use]
    #[inline]
    pub const fn into_byte(self) -> u8 {
        self as u8
    }

    /// Returns [`true`] if this mnemonic is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::Mnemonic;
    /// assert_eq!(Mnemonic::PUSH7.is_push(), true);
    /// assert_eq!(Mnemonic::GAS.is_push(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_push(&self) -> bool {
        matches!(
            self,
            Self::PUSH0
                | Self::PUSH1
                | Self::PUSH2
                | Self::PUSH3
                | Self::PUSH4
                | Self::PUSH5
                | Self::PUSH6
                | Self::PUSH7
                | Self::PUSH8
                | Self::PUSH9
                | Self::PUSH10
                | Self::PUSH11
                | Self::PUSH12
                | Self::PUSH13
                | Self::PUSH14
                | Self::PUSH15
                | Self::PUSH16
                | Self::PUSH17
                | Self::PUSH18
                | Self::PUSH19
                | Self::PUSH20
                | Self::PUSH21
                | Self::PUSH22
                | Self::PUSH23
                | Self::PUSH24
                | Self::PUSH25
                | Self::PUSH26
                | Self::PUSH27
                | Self::PUSH28
                | Self::PUSH29
                | Self::PUSH30
                | Self::PUSH31
                | Self::PUSH32
        )
    }

    /// Returns [`true`] if this mnemonic is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::Mnemonic;
    /// assert_eq!(Mnemonic::DUP2.is_dup(), true);
    /// assert_eq!(Mnemonic::GAS.is_dup(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_dup(&self) -> bool {
        matches!(
            self,
            Self::DUP1
                | Self::DUP2
                | Self::DUP3
                | Self::DUP4
                | Self::DUP5
                | Self::DUP6
                | Self::DUP7
                | Self::DUP8
                | Self::DUP9
                | Self::DUP10
                | Self::DUP11
                | Self::DUP12
                | Self::DUP13
                | Self::DUP14
                | Self::DUP15
                | Self::DUP16
        )
    }

    /// Returns [`true`] if this mnemonic is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::Mnemonic;
    /// assert_eq!(Mnemonic::SWAP2.is_swap(), true);
    /// assert_eq!(Mnemonic::GAS.is_swap(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_swap(&self) -> bool {
        matches!(
            self,
            Self::SWAP1
                | Self::SWAP2
                | Self::SWAP3
                | Self::SWAP4
                | Self::SWAP5
                | Self::SWAP6
                | Self::SWAP7
                | Self::SWAP8
                | Self::SWAP9
                | Self::SWAP10
                | Self::SWAP11
                | Self::SWAP12
                | Self::SWAP13
                | Self::SWAP14
                | Self::SWAP15
                | Self::SWAP16
        )
    }

    /// Returns [`true`] if this mnemonic is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::Mnemonic;
    /// assert_eq!(Mnemonic::LOG2.is_log(), true);
    /// assert_eq!(Mnemonic::GAS.is_log(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_log(&self) -> bool {
        matches!(
            self,
            Self::LOG0 | Self::LOG1 | Self::LOG2 | Self::LOG3 | Self::LOG4
        )
    }

    /// Returns [`true`] if this mnemonic terminates execution of the smart contract.
    /// # Example
    /// ```
    /// # use oculars_asm2::Mnemonic;
    /// assert_eq!(Mnemonic::STOP.is_terminator(), true);
    /// assert_eq!(Mnemonic::REVERT.is_terminator(), true);
    /// assert_eq!(Mnemonic::INVALID.is_terminator(), true);
    /// assert_eq!(Mnemonic::GAS.is_terminator(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_terminator(&self) -> bool {
        matches!(
            self,
            Self::STOP | Self::RETURN | Self::REVERT | Self::INVALID | Self::SELFDESTRUCT
        )
    }

    /// Returns [`true`] if this mnemonic is a `JUMP`, `JUMPI` or a `JUMPDEST`.
    /// # Example
    /// ```
    /// # use oculars_asm2::Mnemonic;
    /// assert_eq!(Mnemonic::JUMP.is_control_flow(), true);
    /// assert_eq!(Mnemonic::JUMPDEST.is_control_flow(), true);
    /// assert_eq!(Mnemonic::GAS.is_control_flow(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_control_flow(&self) -> bool {
        matches!(self, Self::JUMP | Self::JUMPI | Self::JUMPDEST)
    }
}

impl From<Mnemonic> for u8 {
    #[inline]
    fn from(mnemonic: Mnemonic) -> Self {
        mnemonic.into_byte()
    }
}

forward_byte_fmt!(Mnemonic, LowerHex, UpperHex, Binary, Octal);

impl PartialEq<u8> for Mnemonic {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        (*self as u8).eq(other)
    }
}

impl PartialEq<Mnemonic> for u8 {
    #[inline]
    fn eq(&self, other: &Mnemonic) -> bool {
        self.eq(&(*other as u8))
    }
}

impl PartialOrd<u8> for Mnemonic {
    #[inline]
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        (*self as u8).partial_cmp(other)
    }
}

impl PartialOrd<Mnemonic> for u8 {
    #[inline]
    fn partial_cmp(&self, other: &Mnemonic) -> Option<Ordering> {
        self.partial_cmp(&(*other as u8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Mnemonic::{GAS, STOP};

    #[test]
    fn mnemonic_format() {
        assert_eq!(GAS.to_string(), "GAS");
        assert_eq!(format!("{GAS:?}"), "GAS");
        assert_eq!(format!("{GAS:x}"), "5a");
        assert_eq!(format!("{GAS:X}"), "5A");
        assert_eq!(format!("{GAS:o}"), "132");
        assert_eq!(format!("{GAS:b}"), "1011010");
    }

    #[test]
    fn mnemonic_eq() {
        assert_eq!(GAS, 0x5A);
        assert_ne!(GAS, 0x5B);

        assert_eq!(0x5A, GAS);
        assert_ne!(0x5B, GAS);

        assert_eq!(GAS, GAS);
        assert_ne!(GAS, STOP);
    }

    #[test]
    fn mnemonic_ord() {
        assert!(GAS < 0x5B);
        assert!(GAS <= 0x5A);
        assert!(GAS >= 0x5A);
        assert!(GAS > 0x59);

        assert!(0x5B > GAS);
        assert!(0x5A >= GAS);
        assert!(0x5A <= GAS);
        assert!(0x59 < GAS);

        assert!(GAS > STOP);
    }

    #[test]
    fn mnemonic_byte_conversions() {
        assert_eq!(GAS.into_byte(), 0x5A);
        assert_eq!(u8::from(GAS), 0x5A);
        assert_eq!(Mnemonic::from_byte(0x5A), Some(GAS));
        assert_eq!(Mnemonic::from_byte(0xF), None);
    }
}
