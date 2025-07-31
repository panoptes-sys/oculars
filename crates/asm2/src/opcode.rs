//! EVM operation code.

use std::fmt::Display;

use crate::{macros::impl_byte_fmt, Mnemonic};

/// EVM operation code.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum OpCode {
    /// An opcode with a known [`Mnemonic`].
    Known(Mnemonic),
    /// An opcode without a known [`Mnemonic`]. Contains the raw byte.
    Unknown(u8),
}

impl OpCode {
    /// Returns [`true`] if the opcode is known.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_known(), true);
    /// assert_eq!(OpCode::Unknown(0xF).is_known(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }

    /// Returns [`true`] if the opcode is unknown.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Unknown(0xF).is_unknown(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_unknown(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    /// Converts a byte into an [`OpCode`], returning [`OpCode::Unknown`] if no known mnemonic
    /// exists.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::from_byte(0x5A), OpCode::Known(Mnemonic::GAS));
    /// assert_eq!(OpCode::from_byte(0xF), OpCode::Unknown(0xF));
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_byte(byte: u8) -> Self {
        match Mnemonic::from_byte(byte) {
            Some(mnemonic) => Self::Known(mnemonic),
            None => Self::Unknown(byte),
        }
    }

    /// Tries to convert a byte into a known mnemonic, returing [`None`] if no known
    /// mnemonic for this byte exists.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::try_from_byte(0x5A), Some(OpCode::Known(Mnemonic::GAS)));
    /// assert_eq!(OpCode::try_from_byte(0xF), None);
    /// ```
    #[must_use]
    #[inline]
    pub const fn try_from_byte(byte: u8) -> Option<Self> {
        if let Some(mnemonic) = Mnemonic::from_byte(byte) {
            Some(Self::Known(mnemonic))
        } else {
            None
        }
    }

    /// Converts this opcode into a byte.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).into_byte(), 0x5A);
    /// assert_eq!(OpCode::Unknown(0xF).into_byte(), 0xF);
    /// ```
    #[must_use]
    #[inline]
    pub const fn into_byte(self) -> u8 {
        match self {
            OpCode::Known(mnemonic) => mnemonic as u8,
            OpCode::Unknown(byte) => byte,
        }
    }

    /// Returns [`true`] if this opcode is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::PUSH7).is_push(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_push(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_push(&self) -> bool {
        match self {
            OpCode::Known(mnemonic) => mnemonic.is_push(),
            OpCode::Unknown(_) => false,
        }
    }

    /// Returns [`true`] if this opcode is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::DUP2).is_dup(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_dup(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_dup(&self) -> bool {
        match self {
            OpCode::Known(mnemonic) => mnemonic.is_dup(),
            OpCode::Unknown(_) => false,
        }
    }

    /// Returns [`true`] if this opcode is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::SWAP2).is_swap(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_swap(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_swap(&self) -> bool {
        match self {
            OpCode::Known(mnemonic) => mnemonic.is_swap(),
            OpCode::Unknown(_) => false,
        }
    }

    /// Returns [`true`] if this opcode is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::LOG2).is_log(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_log(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_log(&self) -> bool {
        match self {
            OpCode::Known(mnemonic) => mnemonic.is_log(),
            OpCode::Unknown(_) => false,
        }
    }

    /// Returns [`true`] if this mnemonic terminates execution of the smart contract.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::RETURN).is_terminator(), true);
    /// assert_eq!(OpCode::Unknown(0xF).is_terminator(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_terminator(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_terminator(&self) -> bool {
        match self {
            OpCode::Known(mnemonic) => mnemonic.is_terminator(),
            OpCode::Unknown(_) => true,
        }
    }

    /// Returns [`true`] if this mnemonic is a `JUMP`, `JUMPI` or a `JUMPDEST`.
    /// # Example
    /// ```
    /// # use oculars_asm2::{Mnemonic, opcode::OpCode};
    /// assert_eq!(OpCode::Known(Mnemonic::JUMP).is_control_flow(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::JUMPDEST).is_control_flow(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_control_flow(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_control_flow(&self) -> bool {
        match self {
            OpCode::Known(mnemonic) => mnemonic.is_control_flow(),
            OpCode::Unknown(_) => true,
        }
    }
}

impl_byte_fmt!(OpCode, LowerHex, UpperHex, Octal, Binary);

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(mnemonic) => write!(f, "{mnemonic}"),
            Self::Unknown(_) => write!(f, "UNKNOWN"),
        }
    }
}

impl From<u8> for OpCode {
    #[inline]
    fn from(byte: u8) -> Self {
        Self::from_byte(byte)
    }
}

impl From<OpCode> for u8 {
    #[inline]
    fn from(opcode: OpCode) -> Self {
        opcode.into_byte()
    }
}

impl From<Mnemonic> for OpCode {
    fn from(mnemonic: Mnemonic) -> Self {
        Self::Known(mnemonic)
    }
}

impl PartialEq<u8> for OpCode {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        self.into_byte().eq(other)
    }
}

impl PartialEq<OpCode> for u8 {
    #[inline]
    fn eq(&self, other: &OpCode) -> bool {
        self.eq(&other.into_byte())
    }
}

impl PartialOrd<u8> for OpCode {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.into_byte().partial_cmp(other)
    }
}

impl PartialOrd<OpCode> for u8 {
    fn partial_cmp(&self, other: &OpCode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.into_byte())
    }
}

impl PartialEq<Mnemonic> for OpCode {
    fn eq(&self, other: &Mnemonic) -> bool {
        match self {
            OpCode::Known(mnemonic) => mnemonic.eq(other),
            OpCode::Unknown(_) => false,
        }
    }
}

impl PartialEq<OpCode> for Mnemonic {
    fn eq(&self, other: &OpCode) -> bool {
        match other {
            OpCode::Known(other) => self.eq(other),
            OpCode::Unknown(_) => false,
        }
    }
}

impl PartialOrd<Mnemonic> for OpCode {
    fn partial_cmp(&self, other: &Mnemonic) -> Option<std::cmp::Ordering> {
        match self {
            OpCode::Known(mnemonic) => mnemonic.partial_cmp(other),
            OpCode::Unknown(_) => None,
        }
    }
}

impl PartialOrd<OpCode> for Mnemonic {
    fn partial_cmp(&self, other: &OpCode) -> Option<std::cmp::Ordering> {
        match other {
            OpCode::Known(other) => self.partial_cmp(other),
            OpCode::Unknown(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_format() {
        let gas = OpCode::Known(Mnemonic::GAS);
        assert_eq!(gas.to_string(), "GAS");
        assert_eq!(format!("{gas:x}"), "5a");
        assert_eq!(format!("{gas:X}"), "5A");
        assert_eq!(format!("{gas:b}"), "1011010");
        assert_eq!(format!("{gas:o}"), "132");

        let unknown = OpCode::from(0xF);
        assert_eq!(format!("{unknown}"), "UNKNOWN");
        assert_eq!(format!("{unknown:x}"), "f");
        assert_eq!(format!("{unknown:X}"), "F");
        assert_eq!(format!("{unknown:b}"), "1111");
        assert_eq!(format!("{unknown:o}"), "17");
    }

    #[test]
    fn opcode_eq() {
        let gas = OpCode::Known(Mnemonic::GAS);
        assert_eq!(gas, 0x5A);
        assert_eq!(gas, 0x5A);
        assert_ne!(gas, 0x5B);

        let unk = OpCode::Unknown(0x0);
        assert_eq!(unk, 0x0);
        assert_ne!(unk, 0x5A);

        assert_eq!(gas, gas);
        assert_ne!(gas, unk);

        assert_eq!(gas, Mnemonic::GAS);
        assert_eq!(Mnemonic::GAS, gas);

        assert_ne!(gas, Mnemonic::STOP);
        assert_ne!(Mnemonic::STOP, gas);

        assert_ne!(unk, Mnemonic::STOP);
        assert_ne!(Mnemonic::STOP, unk);
    }

    #[test]
    fn opcode_ord() {
        let gas = OpCode::Known(Mnemonic::GAS);

        assert!(gas < 0x5B);
        assert!(gas <= 0x5A);
        assert!(gas >= 0x5A);
        assert!(gas > 0x59);

        assert!(0x5B > gas);
        assert!(0x5A >= gas);
        assert!(0x5A <= gas);
        assert!(0x59 < gas);

        assert!(gas > Mnemonic::STOP);

        let unk = OpCode::Unknown(0x0);
        assert!(unk >= 0x0);
        assert_eq!(unk.partial_cmp(&Mnemonic::STOP), None);
    }

    #[test]
    fn opcode_conversions() {
        assert_eq!(OpCode::Known(Mnemonic::GAS).into_byte(), 0x5A);
        assert_eq!(OpCode::from_byte(0x5A), OpCode::Known(Mnemonic::GAS));
        assert_eq!(OpCode::from_byte(0xF), OpCode::Unknown(0xF));
        assert_eq!(OpCode::from(0xF), OpCode::Unknown(0xF));
        assert_eq!(OpCode::from(0x5A), OpCode::Known(Mnemonic::GAS));
    }
}
