//! EVM operation codes and mnemonics.

use derive_more::{Binary, LowerHex, Octal, UpperHex};
use strum::{Display, EnumCount, EnumIs, EnumIter, FromRepr, IntoEnumIterator, VariantNames};

/// EVM operation code.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    derive_more::Display,
    LowerHex,
    UpperHex,
    Binary,
    Octal,
    Hash,
)]
pub enum OpCode {
    /// A known opcode represented as a mnemonic.
    Known(Mnemonic),
    /// An unknown opcode represented as a byte.
    Unknown(u8),
}

impl OpCode {
    /// Returns [`true`] if the opcode is known.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
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
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
    /// assert_eq!(OpCode::Unknown(0xF).is_unknown(), true);
    /// assert_eq!(OpCode::Known(Mnemonic::GAS).is_unknown(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    /// Convert a byte into an [`OpCode`], returning [`OpCode::Unknown`] if no known mnemonic
    /// exists.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
    /// assert_eq!(OpCode::from_byte(0x5A), OpCode::Known(Mnemonic::GAS));
    /// assert_eq!(OpCode::from_byte(0xF), OpCode::Unknown(0xF));
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_byte(byte: u8) -> Self {
        match Mnemonic::from_repr(byte) {
            Some(mnemonic) => Self::Known(mnemonic),
            None => Self::Unknown(byte),
        }
    }

    /// Try to convert a byte into a known mnemonic.
    /// Returns `None` if the opcode is unknown.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
    /// assert_eq!(OpCode::try_from_byte(0x5A), Some(OpCode::Known(Mnemonic::GAS)));
    /// assert_eq!(OpCode::try_from_byte(0xF), None);
    /// ```
    #[must_use]
    #[inline]
    pub const fn try_from_byte(byte: u8) -> Option<Self> {
        if let Some(mnemonic) = Mnemonic::from_repr(byte) {
            Some(Self::Known(mnemonic))
        } else {
            None
        }
    }

    /// Convert opcode into a byte.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
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

    /// Returns a value signifying whether this opcode is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
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

    /// Returns a value signifying whether this opcode is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
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

    /// Returns a value signifying whether this opcode is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
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

    /// Returns a value signifying whether this opcode is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
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

    /// Returns [`true`] for opcodes that terminate execution of the smart contract.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::{OpCode, Mnemonic};
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
}

impl PartialEq<Mnemonic> for OpCode {
    #[inline]
    fn eq(&self, other: &Mnemonic) -> bool {
        u8::from(self) == *other as u8
    }
}

impl PartialOrd<Mnemonic> for OpCode {
    #[inline]
    fn partial_cmp(&self, other: &Mnemonic) -> Option<std::cmp::Ordering> {
        u8::from(self).partial_cmp(&(*other as u8))
    }
}

impl PartialEq<u8> for OpCode {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        u8::from(self).eq(other)
    }
}

impl PartialOrd<u8> for OpCode {
    #[inline]
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        u8::from(self).partial_cmp(other)
    }
}

impl From<OpCode> for u8 {
    #[inline]
    fn from(opcode: OpCode) -> Self {
        opcode.into_byte()
    }
}

impl From<&OpCode> for u8 {
    #[inline]
    fn from(opcode: &OpCode) -> Self {
        opcode.into_byte()
    }
}

impl From<u8> for OpCode {
    #[inline]
    fn from(byte: u8) -> Self {
        Self::from_byte(byte)
    }
}

impl From<Mnemonic> for OpCode {
    #[inline]
    fn from(value: Mnemonic) -> Self {
        Self::Known(value)
    }
}

/// EVM operation code mnemonic.
#[repr(u8)]
#[non_exhaustive]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Display,
    FromRepr,
    EnumIs,
    EnumCount,
    Hash,
    EnumIter,
    VariantNames,
)]
pub enum Mnemonic {
    /// Halts execution.
    STOP = 0x00,
    /// Addition operation.
    ADD = 0x01,
    /// Multiplication operation.
    MUL = 0x02,
    /// Subtraction operation.
    SUB = 0x03,
    /// Integer division operation.
    DIV = 0x04,
    /// Signed integer division operation (truncated).
    SDIV = 0x05,
    /// Modulo remainder operation.
    MOD = 0x06,
    /// Signed modulo remainder operation.
    SMOD = 0x07,
    /// Modulo addition operation.
    ADDMOD = 0x08,
    /// Modulo multiplication operation.
    MULMOD = 0x09,
    /// Exponential operation.
    EXP = 0x0A,
    /// Extend length of two’s complement signed integer.
    SIGNEXTEND = 0x0B,
    /// Less-than comparison.
    LT = 0x10,
    /// Greater-than comparison.
    GT = 0x11,
    /// Signed less-than comparison.
    SLT = 0x12,
    /// Signed greater-than comparison.
    SGT = 0x13,
    /// Equality comparison.
    EQ = 0x14,
    /// Is-zero comparison.
    ISZERO = 0x15,
    /// Bitwise AND operation.
    AND = 0x16,
    /// Bitwise OR operation.
    OR = 0x17,
    /// Bitwise XOR operation.
    XOR = 0x18,
    /// Bitwise NOT operation.
    NOT = 0x19,
    /// Retrieve single byte from word.
    BYTE = 0x1A,
    /// Left shift operation.
    SHL = 0x1B,
    /// Logical right shift operation.
    SHR = 0x1C,
    /// Arithmetic (signed) right shift operation.
    SAR = 0x1D,
    /// Compute Keccak-256 hash.
    KECCAK256 = 0x20,
    /// Get address of currently executing account.
    ADDRESS = 0x30,
    /// Get balance of the given account.
    BALANCE = 0x31,
    /// Get execution origination address.
    ORIGIN = 0x32,
    /// Get caller address.
    CALLER = 0x33,
    /// Get deposited value by the instruction/transaction responsible for this execution.
    CALLVALUE = 0x34,
    /// Get input data of current environment.
    CALLDATALOAD = 0x35,
    /// Get size of input data in current environment.
    CALLDATASIZE = 0x36,
    /// Copy input data in current environment to memory.
    CALLDATACOPY = 0x37,
    /// Get size of code running in current environment.
    CODESIZE = 0x38,
    /// Copy code running in current environment to memory.
    CODECOPY = 0x39,
    /// Get price of gas in current environment.
    GASPRICE = 0x3A,
    /// Get size of an account’s code.
    EXTCODESIZE = 0x3B,
    /// Copy an account’s code to memory.
    EXTCODECOPY = 0x3C,
    /// Get size of output data from the previous call from the current environment.
    RETURNDATASIZE = 0x3D,
    /// Copy output data from the previous call to memory.
    RETURNDATACOPY = 0x3E,
    /// Get hash of an account’s code.
    EXTCODEHASH = 0x3F,
    /// Get the hash of one of the 256 most recent complete blocks.
    BLOCKHASH = 0x40,
    /// Get the block’s beneficiary address.
    COINBASE = 0x41,
    /// Get the block’s timestamp.
    TIMESTAMP = 0x42,
    /// Get the block’s number.
    NUMBER = 0x43,
    /// Get the block’s difficulty.
    PREVRANDAO = 0x44,
    /// Get the block’s gas limit.
    GASLIMIT = 0x45,
    /// Get the chain ID.
    CHAINID = 0x46,
    /// Get balance of currently executing account.
    SELFBALANCE = 0x47,
    /// Get the base fee.
    BASEFEE = 0x48,
    /// Get versioned hashes.
    BLOBHASH = 0x49,
    /// Returns the value of the blob base-fee of the current block.
    BLOBBASEFEE = 0x4A,
    /// Remove item from stack.
    POP = 0x50,
    /// Load word from memory.
    MLOAD = 0x51,
    /// Save word to memory.
    MSTORE = 0x52,
    /// Save byte to memory.
    MSTORE8 = 0x53,
    /// Load word from storage.
    SLOAD = 0x54,
    /// Save word to storage.
    SSTORE = 0x55,
    /// Alter the program counter.
    JUMP = 0x56,
    /// Conditionally alter the program counter.
    JUMPI = 0x57,
    /// Get the value of the program counter prior to the increment corresponding to this instruction.
    PC = 0x58,
    /// Get the size of active memory in bytes.
    MSIZE = 0x59,
    /// Get the amount of available gas, including the corresponding reduction for the cost of this instruction.
    GAS = 0x5A,
    /// Mark a valid destination for jumps.
    JUMPDEST = 0x5B,
    /// Load word from transient storage.
    TLOAD = 0x5C,
    /// Save word to transient storage.
    TSTORE = 0x5D,
    /// Copy memory areas.
    MCOPY = 0x5E,
    /// Place value 0 on stack.
    PUSH0 = 0x5F,
    /// Place 1 byte item on stack.
    PUSH1 = 0x60,
    /// Place 2 byte item on stack.
    PUSH2 = 0x61,
    /// Place 3 byte item on stack.
    PUSH3 = 0x62,
    /// Place 4 byte item on stack.
    PUSH4 = 0x63,
    /// Place 5 byte item on stack.
    PUSH5 = 0x64,
    /// Place 6 byte item on stack.
    PUSH6 = 0x65,
    /// Place 7 byte item on stack.
    PUSH7 = 0x66,
    /// Place 8 byte item on stack.
    PUSH8 = 0x67,
    /// Place 9 byte item on stack.
    PUSH9 = 0x68,
    /// Place 10 byte item on stack.
    PUSH10 = 0x69,
    /// Place 11 byte item on stack.
    PUSH11 = 0x6A,
    /// Place 12 byte item on stack.
    PUSH12 = 0x6B,
    /// Place 13 byte item on stack.
    PUSH13 = 0x6C,
    /// Place 14 byte item on stack.
    PUSH14 = 0x6D,
    /// Place 15 byte item on stack.
    PUSH15 = 0x6E,
    /// Place 16 byte item on stack.
    PUSH16 = 0x6F,
    /// Place 17 byte item on stack.
    PUSH17 = 0x70,
    /// Place 18 byte item on stack.
    PUSH18 = 0x71,
    /// Place 19 byte item on stack.
    PUSH19 = 0x72,
    /// Place 20 byte item on stack.
    PUSH20 = 0x73,
    /// Place 21 byte item on stack.
    PUSH21 = 0x74,
    /// Place 22 byte item on stack.
    PUSH22 = 0x75,
    /// Place 23 byte item on stack.
    PUSH23 = 0x76,
    /// Place 24 byte item on stack.
    PUSH24 = 0x77,
    /// Place 25 byte item on stack.
    PUSH25 = 0x78,
    /// Place 26 byte item on stack.
    PUSH26 = 0x79,
    /// Place 27 byte item on stack.
    PUSH27 = 0x7A,
    /// Place 28 byte item on stack.
    PUSH28 = 0x7B,
    /// Place 29 byte item on stack.
    PUSH29 = 0x7C,
    /// Place 30 byte item on stack.
    PUSH30 = 0x7D,
    /// Place 31 byte item on stack.
    PUSH31 = 0x7E,
    /// Place 32 byte (full word) item on stack.
    PUSH32 = 0x7F,
    /// Duplicate 1st stack item.
    DUP1 = 0x80,
    /// Duplicate 2nd stack item.
    DUP2 = 0x81,
    /// Duplicate 3rd stack item.
    DUP3 = 0x82,
    /// Duplicate 4th stack item.
    DUP4 = 0x83,
    /// Duplicate 5th stack item.
    DUP5 = 0x84,
    /// Duplicate 6th stack item.
    DUP6 = 0x85,
    /// Duplicate 7th stack item.
    DUP7 = 0x86,
    /// Duplicate 8th stack item.
    DUP8 = 0x87,
    /// Duplicate 9th stack item.
    DUP9 = 0x88,
    /// Duplicate 10th stack item.
    DUP10 = 0x89,
    /// Duplicate 11th stack item.
    DUP11 = 0x8A,
    /// Duplicate 12th stack item.
    DUP12 = 0x8B,
    /// Duplicate 13th stack item.
    DUP13 = 0x8C,
    /// Duplicate 14th stack item.
    DUP14 = 0x8D,
    /// Duplicate 15th stack item.
    DUP15 = 0x8E,
    /// Duplicate 16th stack item.
    DUP16 = 0x8F,
    /// Exchange 1st and 2nd stack items.
    SWAP1 = 0x90,
    /// Exchange 1st and 3rd stack items.
    SWAP2 = 0x91,
    /// Exchange 1st and 4th stack items.
    SWAP3 = 0x92,
    /// Exchange 1st and 5th stack items.
    SWAP4 = 0x93,
    /// Exchange 1st and 6th stack items.
    SWAP5 = 0x94,
    /// Exchange 1st and 7th stack items.
    SWAP6 = 0x95,
    /// Exchange 1st and 8th stack items.
    SWAP7 = 0x96,
    /// Exchange 1st and 9th stack items.
    SWAP8 = 0x97,
    /// Exchange 1st and 10th stack items.
    SWAP9 = 0x98,
    /// Exchange 1st and 11th stack items.
    SWAP10 = 0x99,
    /// Exchange 1st and 12th stack items.
    SWAP11 = 0x9A,
    /// Exchange 1st and 13th stack items.
    SWAP12 = 0x9B,
    /// Exchange 1st and 14th stack items.
    SWAP13 = 0x9C,
    /// Exchange 1st and 15th stack items.
    SWAP14 = 0x9D,
    /// Exchange 1st and 16th stack items.
    SWAP15 = 0x9E,
    /// Exchange 1st and 17th stack items.
    SWAP16 = 0x9F,
    /// Append log record with no topics.
    LOG0 = 0xA0,
    /// Append log record with one topic.
    LOG1 = 0xA1,
    /// Append log record with two topics.
    LOG2 = 0xA2,
    /// Append log record with three topics.
    LOG3 = 0xA3,
    /// Append log record with four topics.
    LOG4 = 0xA4,
    /// Create a new account with associated code.
    CREATE = 0xF0,
    /// Message-call into an account.
    CALL = 0xF1,
    /// Message-call into this account with alternative account’s code.
    CALLCODE = 0xF2,
    /// Halt execution returning output data.
    RETURN = 0xF3,
    /// Message-call into this account with an alternative account’s code, but persisting the current values for sender and value.
    DELEGATECALL = 0xF4,
    /// Create a new account with associated code at a predictable address.
    CREATE2 = 0xF5,
    /// Static message-call into an account.
    STATICCALL = 0xFA,
    /// Halt execution reverting state changes but returning data and remaining gas.
    REVERT = 0xFD,
    /// Designated invalid instruction.
    INVALID = 0xFE,
    /// Halt execution and register account for later deletion or send all Ether to address (post-Cancun).
    SELFDESTRUCT = 0xFF,
}

impl Mnemonic {
    /// Returns an iterator over all mnemonics.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::Mnemonic;
    /// let mut mnemonics = Mnemonic::iter();
    /// assert_eq!(mnemonics.next(), Some(Mnemonic::STOP));
    /// assert_eq!(mnemonics.next(), Some(Mnemonic::ADD));
    /// ```
    #[must_use]
    #[inline]
    pub fn iter() -> MnemonicIter {
        <Self as IntoEnumIterator>::iter()
    }

    /// Returns a static slice of mnemonic names.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::Mnemonic;
    /// assert_eq!(Mnemonic::variants()[0], "STOP");
    /// assert_eq!(Mnemonic::variants()[1], "ADD");
    /// ```
    pub const fn variants() -> &'static [&'static str] {
        Self::VARIANTS
    }

    /// Returns a value signifying whether this mnemonic is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::Mnemonic;
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

    /// Returns a value signifying whether this mnemonic is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::Mnemonic;
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

    /// Returns a value signifying whether this mnemonic is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::Mnemonic;
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

    /// Returns a value signifying whether this mnemonic is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use eva_asm::opcode::Mnemonic;
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

    /// Returns [`true`] for mnemonics that terminate execution of the smart contract.
    /// # Example
    /// ```
    /// # use eva_asm::opcode::Mnemonic;
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
}

impl PartialEq<OpCode> for Mnemonic {
    #[inline]
    fn eq(&self, other: &OpCode) -> bool {
        *self as u8 == u8::from(other)
    }
}

impl PartialOrd<OpCode> for Mnemonic {
    #[inline]
    fn partial_cmp(&self, other: &OpCode) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&u8::from(other))
    }
}

impl PartialEq<u8> for Mnemonic {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }
}

impl PartialOrd<u8> for Mnemonic {
    #[inline]
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(other)
    }
}

/// Implement formatting for mnemonics.
macro_rules! impl_mnemonic_fmt {
    ($($fmt: ident),+) => {
        $(impl std::fmt::$fmt for Mnemonic {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::$fmt::fmt(&(*self as u8), f)
            }
        })+
    };
}

impl_mnemonic_fmt!(LowerHex, UpperHex, Binary, Octal);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mnemonic_fmt() {
        let gas = Mnemonic::GAS;
        assert_eq!(format!("{gas}"), "GAS");
        assert_eq!(format!("{gas:x}"), "5a");
        assert_eq!(format!("{gas:X}"), "5A");
        assert_eq!(format!("{gas:b}"), "1011010");
        assert_eq!(format!("{gas:o}"), "132");
    }

    #[test]
    fn mnemonic_cmp() {
        let gas = Mnemonic::GAS;
        let add = Mnemonic::ADD;

        assert_eq!(gas, gas);
        assert_ne!(gas, add);
        assert!(gas > add);
        assert_eq!(gas, OpCode::Known(Mnemonic::GAS));
        assert_ne!(gas, OpCode::Known(Mnemonic::ADD));
        assert!(gas > OpCode::Known(Mnemonic::ADD));
        assert_eq!(gas, 0x5A);
        assert!(gas > 0x1);
    }

    #[test]
    fn opcode_fmt() {
        let gas = OpCode::Known(Mnemonic::GAS);
        assert_eq!(format!("{gas}"), "GAS");
        assert_eq!(format!("{gas:x}"), "5a");
        assert_eq!(format!("{gas:X}"), "5A");
        assert_eq!(format!("{gas:b}"), "1011010");
        assert_eq!(format!("{gas:o}"), "132");

        let unknown = OpCode::from(0xF);
        assert_eq!(format!("{unknown}"), "15");
        assert_eq!(format!("{unknown:x}"), "f");
        assert_eq!(format!("{unknown:X}"), "F");
        assert_eq!(format!("{unknown:b}"), "1111");
        assert_eq!(format!("{unknown:o}"), "17");
    }

    #[test]
    fn opcode_conversions() {
        let gas = OpCode::Known(Mnemonic::GAS);

        assert_eq!(u8::from(gas), 0x5A);
        assert_eq!(u8::from(&gas), 0x5A);
        assert_eq!(gas.into_byte(), 0x5A);
        assert_eq!(gas, OpCode::from(Mnemonic::GAS));
    }

    #[test]
    fn opcode_cmp() {
        let gas = OpCode::Known(Mnemonic::GAS);
        let add = OpCode::Known(Mnemonic::ADD);

        assert_eq!(gas, gas);
        assert!(gas > add);
        assert_ne!(gas, add);
        assert_eq!(gas, Mnemonic::GAS);
        assert!(gas > Mnemonic::ADD);
        assert_eq!(gas, 0x5A);
        assert!(gas > 0x1);
    }
}
