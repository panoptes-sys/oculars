//! Mnemonic and instruction definitions.

use crate::instruction::{Dup, Log, Push, Swap, Unknown};

/// Defines the `Mnemonic` enum and implements a `VARIANTS` constant, a byte conversion and [`std::fmt::Display`] for the created enum.
macro_rules! define_mnemonics {
    ($($name: ident = $opcode: literal / $doc: literal),+) => {
        /// EVM operation code mnemonic.
        #[repr(u8)]
        #[non_exhaustive]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Mnemonic {
            $(
                #[doc = $doc]
                $name = $opcode
            ),+
        }

        impl Mnemonic {
            /// A static slice of all mnemonic variants.
            ///
            /// # Note
            /// The array index does *not* correspond to a mnemonic's opcode.
            ///
            /// # Example
            /// ```
            /// # use oculars_asm::Mnemonic;
            /// assert_eq!(Mnemonic::VARIANTS[0], Mnemonic::STOP);
            /// assert_eq!(Mnemonic::VARIANTS[2], Mnemonic::MUL);
            /// ```
            pub const VARIANTS: &[Self] = &[$(Self::$name),+];

            /// Attempts to parse a byte as a mnemonic. Returns [`None`] if the byte is not a known
            /// mnemonic.
            ///
            /// # Example
            /// ```
            /// # use oculars_asm::Mnemonic;
            /// assert_eq!(Mnemonic::from_byte(0x5A), Some(Mnemonic::GAS));
            /// assert_eq!(Mnemonic::from_byte(0xF), None);
            /// ```
            #[must_use]
            pub const fn from_byte(byte: u8) -> Option<Self> {
                match byte {
                    $(
                        $opcode => Some(Self::$name),
                    )+
                    _ => None
                }
            }
        }

        impl std::fmt::Display for Mnemonic {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                        Self::$name => stringify!($name),
                    )+
                })
            }
        }
    };
}

/// Defines an instruction by creating a struct and implementing [`crate::AssemblyInstruction`] for the
/// struct.
macro_rules! define_instruction {
    ($name: ident = $mnemonic: ident / $doc: literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::assembly::AssemblyInstruction for $name {
            #[inline]
            fn opcode(&self) -> $crate::opcode::OpCode {
                $crate::opcode::OpCode::Known($crate::Mnemonic::$mnemonic)
            }

            #[inline]
            fn mnemonic(&self) -> Option<$crate::mnemonic::Mnemonic> {
                Some($crate::Mnemonic::$mnemonic)
            }

            fn disassemble(bytes: &[u8]) -> Result<Self, $crate::assembly::DisassemblyError> {
                $crate::assembly::verify_opcode(bytes, $crate::Mnemonic::$mnemonic as u8)?;
                Ok($name)
            }
        }

        $crate::fmt::forward_opcode_fmt!($name, Display, LowerHex, UpperHex, Binary, Octal);
    };
    // Don't define an instruction struct if there is a `!` argument present.
    // This allows for manual implementation of instructions that can be generic (like `Push<N>`).
    ($name: ident = $mnemonic: ident / $doc: literal, !) => {};
}

/// Creates an enumeration containing all instructions.
macro_rules! define_instructions_enum {
    ($($name: ident, $value: path =/ $doc: literal),+) => {
        /// An EVM instruction.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Instruction {
            $(
                #[doc = $doc]
                $name($value),
            )+
            #[doc = "Unknown instruction."]
            Unknown(Unknown)
        }
    };
}

/// Creates a macro that matches an instruction and calls a function passing it as the argument.
macro_rules! define_instruction_matcher {
    (($d: tt) $($name: ident),+) => {
        /// Matches an instruction and calls the specified function, passing the instruction as the
        /// first argument and any optional arguments.
        #[macro_export]
        macro_rules! match_instruction {
            ($instr: ident, $fn: path $d(,$arg: expr)*) => {{
                use $crate::Instruction;

                match $instr {
                    $(
                        Instruction::$name(i) => $fn(i $d(,$arg)*),
                    )+
                    Instruction::Unknown(i) => $fn(i $d(,$arg)*)
                }
            }}
        }
    };
    ($($name: ident),+) => {
        define_instruction_matcher!(($) $($name),+);
    };
}

/// Defines a macro that matches the opcode and disassembles the corresponding instruction.
macro_rules! define_instruction_disassembly {
    ($($name: ident, $value: path, $opcode: literal),+) => {
        /// Matches the opcode and disassembles the instruction.
        #[macro_export]
        macro_rules! disassemble_instruction {
            ($bytes: ident) => {{
                use $crate::assembly::*;
                use $crate::instruction::*;

                let opcode = *$bytes
                    .first()
                    .ok_or_else(|| DisassemblyError::UnexpectedLength {
                        got: $bytes.len(),
                        expected: 1,
                    })?;

                match opcode {
                    $(
                        $opcode => Ok(Instruction::$name(<$value>::disassemble($bytes)?)),
                    )+
                    _ => Ok(Instruction::Unknown(Unknown::disassemble($bytes)?))
                }
            }}
        }
    };
}

/// Defines the lists of instructions, mnemonics and additional helpers.
macro_rules! define_instructions {
    ($($mnemonic: ident, $name: ident, $struct: path $([$dont: tt])? = $opcode: literal / $doc: literal),+) => {
        pub mod mnemonic {
            define_mnemonics!($($mnemonic = $opcode / $doc),+);
        }

        pub mod instruction {
            use super::*;

            $(
                define_instruction!($name = $mnemonic / $doc $(,$dont)?);
            )+

            define_instructions_enum!($($name, $struct =/ $doc),+);

            pub(crate) mod macros {
                define_instruction_matcher!($($name),+);
                define_instruction_disassembly!($($name, $struct, $opcode),+);

                pub use match_instruction;
                pub use disassemble_instruction;
            }
        }
    };
}

define_instructions!(
    STOP,             Stop,             Stop =              0x00 /     "Halts execution.",
    ADD,              Add,              Add =               0x01 /     "Addition operation.",
    MUL,              Mul,              Mul =               0x02 /     "Multiplication operation.",
    SUB,              Sub,              Sub =               0x03 /     "Subtraction operation.",
    DIV,              Div,              Div =               0x04 /     "Integer division operation.",
    SDIV,             SDiv,             SDiv =              0x05 /     "Signed integer division operation (truncated.",
    MOD,              Mod,              Mod =               0x06 /     "Modulo remainder operation.",
    SMOD,             SMod,             SMod =              0x07 /     "Signed modulo remainder operation.",
    ADDMOD,           AddMod,           AddMod =            0x08 /     "Modulo addition operation.",
    MULMOD,           MulMod,           MulMod =            0x09 /     "Modulo multiplication operation.",
    EXP,              Exp,              Exp =               0x0A /     "Exponential operation.",
    SIGNEXTEND,       SignExtend,       SignExtend =        0x0B /     "Extend length of two's complement signed integer.",
    LT,               Lt,               Lt =                0x10 /     "Less-than comparison.",
    GT,               Gt,               Gt =                0x11 /     "Greater-than comparison.",
    SLT,              SLt,              SLt =               0x12 /     "Signed less-than comparison.",
    SGT,              SGt,              SGt =               0x13 /     "Signed greater-than comparison.",
    EQ,               Eq,               Eq =                0x14 /     "Equality comparison.",
    ISZERO,           IsZero,           IsZero =            0x15 /     "Is-zero comparison.",
    AND,              And,              And =               0x16 /     "Bitwise AND operation.",
    OR,               Or,               Or =                0x17 /     "Bitwise OR operation.",
    XOR,              Xor,              Xor =               0x18 /     "Bitwise XOR operation.",
    NOT,              Not,              Not =               0x19 /     "Bitwise NOT operation.",
    BYTE,             Byte,             Byte =              0x1A /     "Retrieve single byte from word.",
    SHL,              Shl,              Shl =               0x1B /     "Left shift operation.",
    SHR,              Shr,              Shr =               0x1C /     "Logical right shift operation.",
    SAR,              Sar,              Sar =               0x1D /     "Arithmetic (signed) right shift operation.",
    KECCAK256,        Keccak256,        Keccak256 =         0x20 /     "Compute Keccak-256 hash.",
    ADDRESS,          Address,          Address =           0x30 /     "Get address of currently executing account.",
    BALANCE,          Balance,          Balance =           0x31 /     "Get balance of the given account.",
    ORIGIN,           Origin,           Origin =            0x32 /     "Get execution origination address.",
    CALLER,           Caller,           Caller =            0x33 /     "Get caller address.",
    CALLVALUE,        CallValue,        CallValue =         0x34 /     "Get deposited value by the instruction transaction responsible for this execution.",
    CALLDATALOAD,     CallDataLoad,     CallDataLoad =      0x35 /     "Get input data of current environment.",
    CALLDATASIZE,     CallDataSize,     CallDataSize =      0x36 /     "Get size of input data in current environment.",
    CALLDATACOPY,     CallDataCopy,     CallDataCopy =      0x37 /     "Copy input data in current environment to memory.",
    CODESIZE,         CodeSize,         CodeSize =          0x38 /     "Get size of code running in current environment.",
    CODECOPY,         CodeCopy,         CodeCopy =          0x39 /     "Copy code running in current environment to memory.",
    GASPRICE,         GasPrice,         GasPrice =          0x3A /     "Get price of gas in current environment.",
    EXTCODESIZE,      ExtCodeSize,      ExtCodeSize =       0x3B /     "Get size of an account's code.",
    EXTCODECOPY,      ExtCodeCopy,      ExtCodeCopy =       0x3C /     "Copy an account's code to memory.",
    RETURNDATASIZE,   ReturnDataSize,   ReturnDataSize =    0x3D /     "Get size of output data from the previous call from the current environment.",
    RETURNDATACOPY,   ReturnDataCopy,   ReturnDataCopy =    0x3E /     "Copy output data from the previous call to memory.",
    EXTCODEHASH,      ExtCodeHash,      ExtCodeHash =       0x3F /     "Get hash of an account's code.",
    BLOCKHASH,        BlockHash,        BlockHash =         0x40 /     "Get the hash of one of the 256 most recent complete blocks.",
    COINBASE,         CoinBase,         CoinBase =          0x41 /     "Get the block's beneficiary address.",
    TIMESTAMP,        Timestamp,        Timestamp =         0x42 /     "Get the block's timestamp.",
    NUMBER,           Number,           Number =            0x43 /     "Get the block's number.",
    PREVRANDAO,       PrevRandao,       PrevRandao =        0x44 /     "Get the block's difficulty.",
    GASLIMIT,         GasLimit,         GasLimit =          0x45 /     "Get the block's gas limit.",
    CHAINID,          ChainId,          ChainId =           0x46 /     "Get the chain ID.",
    SELFBALANCE,      SelfBalance,      SelfBalance =       0x47 /     "Get balance of currently executing account.",
    BASEFEE,          BaseFee,          BaseFee =           0x48 /     "Get the base fee.",
    BLOBHASH,         BlobHash,         BlobHash =          0x49 /     "Get versioned hashes.",
    BLOBBASEFEE,      BlobBaseFee,      BlobBaseFee =       0x4A /     "Returns the value of the blob base-fee of the current block.",
    POP,              Pop,              Pop =               0x50 /     "Remove item from stack.",
    MLOAD,            MLoad,            MLoad =             0x51 /     "Load word from memory.",
    MSTORE,           MStore,           MStore =            0x52 /     "Save word to memory.",
    MSTORE8,          MStore8,          MStore8 =           0x53 /     "Save byte to memory.",
    SLOAD,            SLoad,            SLoad =             0x54 /     "Load word from storage.",
    SSTORE,           SStore,           SStore =            0x55 /     "Save word to storage.",
    JUMP,             Jump,             Jump =              0x56 /     "Alter the program counter.",
    JUMPI,            JumpI,            JumpI =             0x57 /     "Conditionally alter the program counter.",
    PC,               Pc,               Pc =                0x58 /     "Get the value of the program counter prior to the increment corresponding to this instruction.",
    MSIZE,            MSize,            MSize =             0x59 /     "Get the size of active memory in bytes.",
    GAS,              Gas,              Gas =               0x5A /     "Get the amount of available gas including the corresponding reduction for the cost of this instruction.",
    JUMPDEST,         JumpDest,         JumpDest =          0x5B /     "Mark a valid destination for jumps.",
    TLOAD,            TLoad,            TLoad =             0x5C /     "Load word from transient storage.",
    TSTORE,           TStore,           TStore =            0x5D /     "Save word to transient storage.",
    MCOPY,            MCopy,            MCopy =             0x5E /     "Copy memory areas.",
    PUSH0,            Push0,            Push<0> [!] =       0x5F /     "Place value 0 on stack.",
    PUSH1,            Push1,            Push<1> [!] =       0x60 /     "Place 1 byte item on stack.",
    PUSH2,            Push2,            Push<2> [!] =       0x61 /     "Place 2 byte item on stack.",
    PUSH3,            Push3,            Push<3> [!] =       0x62 /     "Place 3 byte item on stack.",
    PUSH4,            Push4,            Push<4> [!] =       0x63 /     "Place 4 byte item on stack.",
    PUSH5,            Push5,            Push<5> [!] =       0x64 /     "Place 5 byte item on stack.",
    PUSH6,            Push6,            Push<6> [!] =       0x65 /     "Place 6 byte item on stack.",
    PUSH7,            Push7,            Push<7> [!] =       0x66 /     "Place 7 byte item on stack.",
    PUSH8,            Push8,            Push<8> [!] =       0x67 /     "Place 8 byte item on stack.",
    PUSH9,            Push9,            Push<9> [!] =       0x68 /     "Place 9 byte item on stack.",
    PUSH10,           Push10,           Push<10> [!] =      0x69 /     "Place 10 byte item on stack.",
    PUSH11,           Push11,           Push<11> [!] =      0x6A /     "Place 11 byte item on stack.",
    PUSH12,           Push12,           Push<12> [!] =      0x6B /     "Place 12 byte item on stack.",
    PUSH13,           Push13,           Push<13> [!] =      0x6C /     "Place 13 byte item on stack.",
    PUSH14,           Push14,           Push<14> [!] =      0x6D /     "Place 14 byte item on stack.",
    PUSH15,           Push15,           Push<15> [!] =      0x6E /     "Place 15 byte item on stack.",
    PUSH16,           Push16,           Push<16> [!] =      0x6F /     "Place 16 byte item on stack.",
    PUSH17,           Push17,           Push<17> [!] =      0x70 /     "Place 17 byte item on stack.",
    PUSH18,           Push18,           Push<18> [!] =      0x71 /     "Place 18 byte item on stack.",
    PUSH19,           Push19,           Push<19> [!] =      0x72 /     "Place 19 byte item on stack.",
    PUSH20,           Push20,           Push<20> [!] =      0x73 /     "Place 20 byte item on stack.",
    PUSH21,           Push21,           Push<21> [!] =      0x74 /     "Place 21 byte item on stack.",
    PUSH22,           Push22,           Push<22> [!] =      0x75 /     "Place 22 byte item on stack.",
    PUSH23,           Push23,           Push<23> [!] =      0x76 /     "Place 23 byte item on stack.",
    PUSH24,           Push24,           Push<24> [!] =      0x77 /     "Place 24 byte item on stack.",
    PUSH25,           Push25,           Push<25> [!] =      0x78 /     "Place 25 byte item on stack.",
    PUSH26,           Push26,           Push<26> [!] =      0x79 /     "Place 26 byte item on stack.",
    PUSH27,           Push27,           Push<27> [!] =      0x7A /     "Place 27 byte item on stack.",
    PUSH28,           Push28,           Push<28> [!] =      0x7B /     "Place 28 byte item on stack.",
    PUSH29,           Push29,           Push<29> [!] =      0x7C /     "Place 29 byte item on stack.",
    PUSH30,           Push30,           Push<30> [!] =      0x7D /     "Place 30 byte item on stack.",
    PUSH31,           Push31,           Push<31> [!] =      0x7E /     "Place 31 byte item on stack.",
    PUSH32,           Push32,           Push<32> [!] =      0x7F /     "Place 32 byte (full word) item on stack.",
    DUP1,             Dup1,             Dup<1> [!] =        0x80 /     "Duplicate 1st stack item.",
    DUP2,             Dup2,             Dup<2> [!] =        0x81 /     "Duplicate 2nd stack item.",
    DUP3,             Dup3,             Dup<3> [!] =        0x82 /     "Duplicate 3rd stack item.",
    DUP4,             Dup4,             Dup<4> [!] =        0x83 /     "Duplicate 4th stack item.",
    DUP5,             Dup5,             Dup<5> [!] =        0x84 /     "Duplicate 5th stack item.",
    DUP6,             Dup6,             Dup<6> [!] =        0x85 /     "Duplicate 6th stack item.",
    DUP7,             Dup7,             Dup<7> [!] =        0x86 /     "Duplicate 7th stack item.",
    DUP8,             Dup8,             Dup<8> [!] =        0x87 /     "Duplicate 8th stack item.",
    DUP9,             Dup9,             Dup<9> [!] =        0x88 /     "Duplicate 9th stack item.",
    DUP10,            Dup10,            Dup<10> [!] =       0x89 /     "Duplicate 10th stack item.",
    DUP11,            Dup11,            Dup<11> [!] =       0x8A /     "Duplicate 11th stack item.",
    DUP12,            Dup12,            Dup<12> [!] =       0x8B /     "Duplicate 12th stack item.",
    DUP13,            Dup13,            Dup<13> [!] =       0x8C /     "Duplicate 13th stack item.",
    DUP14,            Dup14,            Dup<14> [!] =       0x8D /     "Duplicate 14th stack item.",
    DUP15,            Dup15,            Dup<15> [!] =       0x8E /     "Duplicate 15th stack item.",
    DUP16,            Dup16,            Dup<16> [!] =       0x8F /     "Duplicate 16th stack item.",
    SWAP1,            Swap1,            Swap<1> [!] =       0x90 /     "Exchange 1st and 2nd stack items.",
    SWAP2,            Swap2,            Swap<2> [!] =       0x91 /     "Exchange 1st and 3rd stack items.",
    SWAP3,            Swap3,            Swap<3> [!] =       0x92 /     "Exchange 1st and 4th stack items.",
    SWAP4,            Swap4,            Swap<4> [!] =       0x93 /     "Exchange 1st and 5th stack items.",
    SWAP5,            Swap5,            Swap<5> [!] =       0x94 /     "Exchange 1st and 6th stack items.",
    SWAP6,            Swap6,            Swap<6> [!] =       0x95 /     "Exchange 1st and 7th stack items.",
    SWAP7,            Swap7,            Swap<7> [!] =       0x96 /     "Exchange 1st and 8th stack items.",
    SWAP8,            Swap8,            Swap<8> [!] =       0x97 /     "Exchange 1st and 9th stack items.",
    SWAP9,            Swap9,            Swap<9> [!] =       0x98 /     "Exchange 1st and 10th stack items.",
    SWAP10,           Swap10,           Swap<10> [!] =      0x99 /     "Exchange 1st and 11th stack items.",
    SWAP11,           Swap11,           Swap<11> [!] =      0x9A /     "Exchange 1st and 12th stack items.",
    SWAP12,           Swap12,           Swap<12> [!] =      0x9B /     "Exchange 1st and 13th stack items.",
    SWAP13,           Swap13,           Swap<13> [!] =      0x9C /     "Exchange 1st and 14th stack items.",
    SWAP14,           Swap14,           Swap<14> [!] =      0x9D /     "Exchange 1st and 15th stack items.",
    SWAP15,           Swap15,           Swap<15> [!] =      0x9E /     "Exchange 1st and 16th stack items.",
    SWAP16,           Swap16,           Swap<16> [!] =      0x9F /     "Exchange 1st and 17th stack items.",
    LOG0,             Log0,             Log<0> [!] =        0xA0 /     "Append log record with no topics.",
    LOG1,             Log1,             Log<1> [!] =        0xA1 /     "Append log record with one topic.",
    LOG2,             Log2,             Log<2> [!] =        0xA2 /     "Append log record with two topics.",
    LOG3,             Log3,             Log<3> [!] =        0xA3 /     "Append log record with three topics.",
    LOG4,             Log4,             Log<4> [!] =        0xA4 /     "Append log record with four topics.",
    CREATE,           Create,           Create =            0xF0 /     "Create a new account with associated code.",
    CALL,             Call,             Call =              0xF1 /     "Message-call into an account.",
    CALLCODE,         CallCode,         CallCode =          0xF2 /     "Message-call into this account with alternative account's code.",
    RETURN,           Return,           Return =            0xF3 /     "Halt execution returning output data.",
    DELEGATECALL,     DelegateCall,     DelegateCall =      0xF4 /     "Message-call into this account with an alternative account's code but persisting the current values for sender and value.",
    CREATE2,          Create2,          Create2 =           0xF5 /     "Create a new account with associated code at a predictable address.",
    STATICCALL,       StaticCall,       StaticCall =        0xFA /     "Static message-call into an account.",
    REVERT,           Revert,           Revert =            0xFD /     "Halt execution reverting state changes but returning data and remaining gas.",
    INVALID,          Invalid,          Invalid =           0xFE /     "Designated invalid instruction.",
    SELFDESTRUCT,     SelfDestruct,     SelfDestruct =      0xFF /     "Halt execution and register account for later deletion or send all Ether to address (post-Cancun."
);

#[cfg(test)]
mod tests {
    use super::{instruction::Add, mnemonic::Mnemonic};
    use crate::{AssemblyInstruction, OpCode};

    #[expect(non_camel_case_types, reason = "le funny")]
    #[test]
    fn define_mnemonics_works() {
        define_mnemonics!(
            PAY_VITALIK = 0xF / "Pays Vitalik",
            REVERT = 0xFD / "Reverts"
        );

        assert_eq!(Mnemonic::VARIANTS[0], Mnemonic::PAY_VITALIK);
        assert_eq!(Mnemonic::VARIANTS[1], Mnemonic::REVERT);
        assert_eq!(Mnemonic::from_byte(0xF), Some(Mnemonic::PAY_VITALIK));
        assert_eq!(Mnemonic::from_byte(0x5A), None);
        assert_eq!(Mnemonic::PAY_VITALIK.to_string(), "PAY_VITALIK");
    }

    #[test]
    fn instructions_are_defined_properly() {
        assert_eq!(Mnemonic::from_byte(0x1), Some(Mnemonic::ADD));
        assert_eq!(Mnemonic::ADD.into_byte(), 0x1);
        assert_eq!(Add.size(), 1);
        assert_eq!(Add.opcode(), OpCode::Known(Mnemonic::ADD));
    }
}
