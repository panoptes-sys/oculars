//! EVM instruction set.

mod arithmetic;
mod bitwise;
mod block;
mod environment;
mod flow;
mod logging;
mod memory;
mod stack;
mod storage;
mod system;

use std::{
    cmp::Eq as EqTrait,
    fmt::{Debug, Display},
    hash::Hash,
};

pub use arithmetic::*;
pub use bitwise::*;
pub use block::*;
use derive_more::Display;
pub use environment::*;
pub use flow::*;
pub use logging::*;
pub use memory::*;
pub use stack::*;
pub use storage::*;
pub use system::*;

use crate::opcode::OpCode;

/// General instruction information.
pub trait InstructionMeta: Display + Debug + Clone + Copy + PartialEq + EqTrait + Hash {
    /// Return the operation code associated with this instruction.
    ///
    /// # Example
    /// ```
    /// # use asm::{opcode::{OpCode, Mnemonic}, instruction::{Unknown, Gas, InstructionMeta}};
    /// assert_eq!(Gas.opcode(), OpCode::Known(Mnemonic::GAS));
    /// assert_eq!(Unknown(0xF).opcode(), OpCode::Unknown(0xF));
    /// ```
    fn opcode(&self) -> OpCode;

    /// Returns a value signifying whether this instruction is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use asm::instruction::{Push, Gas, InstructionMeta};
    /// assert_eq!(Push::new([0; 10]).is_push(), true);
    /// assert_eq!(Gas.is_push(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_push(&self) -> bool {
        self.opcode().is_push()
    }

    /// Returns a value signifying whether this instruction is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use asm::instruction::{Dup, Gas, InstructionMeta};
    /// assert_eq!(Dup::<10>::new().is_dup(), true);
    /// assert_eq!(Gas.is_dup(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_dup(&self) -> bool {
        self.opcode().is_dup()
    }

    /// Returns a value signifying whether this instruction is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use asm::instruction::{Swap, Gas, InstructionMeta};
    /// assert_eq!(Swap::<10>::new().is_swap(), true);
    /// assert_eq!(Gas.is_swap(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_swap(&self) -> bool {
        self.opcode().is_swap()
    }

    /// Returns a value signifying whether this instruction is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use asm::instruction::{Log, Gas, InstructionMeta};
    /// assert_eq!(Log::<3>::new().is_log(), true);
    /// assert_eq!(Gas.is_log(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_log(&self) -> bool {
        self.opcode().is_log()
    }

    /// Returns [`true`] for instructions that terminate execution of the smart contract.
    ///
    /// # Example
    /// ```
    /// # use asm::instruction::{Return, Unknown, Gas, InstructionMeta};
    /// assert_eq!(Return.is_terminator(), true);
    /// assert_eq!(Unknown(0xF).is_terminator(), true);
    /// assert_eq!(Gas.is_terminator(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_terminator(&self) -> bool {
        self.opcode().is_terminator()
    }
}

/// EVM instruction.
#[non_exhaustive]
#[derive(Display, Debug, Clone, Copy, PartialEq, EqTrait, Hash)]
pub enum Instruction {
    /// Halts execution.
    Stop(Stop),
    /// Addition operation.
    Add(Add),
    /// Multiplication operation.
    Mul(Mul),
    /// Subtraction operation.
    Sub(Sub),
    /// Integer division operation.
    Div(Div),
    /// Signed integer division operation (truncated).
    SDiv(SDiv),
    /// Modulo remainder operation.
    Mod(Mod),
    /// Signed modulo remainder operation.
    SMod(SMod),
    /// Modulo addition operation.
    AddMod(AddMod),
    /// Modulo multiplication operation.
    MulMod(MulMod),
    /// Exponential operation.
    Exp(Exp),
    /// Extend length of two’s complement signed integer.
    SignExtend(SignExtend),
    /// Less-than comparison.
    Lt(Lt),
    /// Greater-than comparison.
    Gt(Gt),
    /// Signed less-than comparison.
    SLt(SLt),
    /// Signed greater-than comparison.
    SGt(SGt),
    /// Equality comparison.
    Eq(Eq),
    /// Is-zero comparison.
    IsZero(IsZero),
    /// Bitwise AND operation.
    And(And),
    /// Bitwise OR operation.
    Or(Or),
    /// Bitwise XOR operation.
    Xor(Xor),
    /// Bitwise NOT operation.
    Not(Not),
    /// Retrieve single byte from word.
    Byte(Byte),
    /// Left shift operation.
    Shl(Shl),
    /// Logical right shift operation.
    Shr(Shr),
    /// Arithmetic (signed) right shift operation.
    Sar(Sar),
    /// Compute Keccak-256 hash.
    Keccak256(Keccak256),
    /// Get address of currently executing account.
    Address(Address),
    /// Get balance of the given account.
    Balance(Balance),
    /// Get execution origination address.
    Origin(Origin),
    /// Get caller address.
    Caller(Caller),
    /// Get deposited value by the instruction/transaction responsible for this execution.
    CallValue(CallValue),
    /// Get input data of current environment.
    CallDataLoad(CallDataLoad),
    /// Get size of input data in current environment.
    CallDataSize(CallDataSize),
    /// Copy input data in current environment to memory.
    CallDataCopy(CallDataCopy),
    /// Get size of code running in current environment.
    CodeSize(CodeSize),
    /// Copy code running in current environment to memory.
    CodeCopy(CodeCopy),
    /// Get price of gas in current environment.
    GasPrice(GasPrice),
    /// Get size of an account’s code.
    ExtCodeSize(ExtCodeSize),
    /// Copy an account’s code to memory.
    ExtCodeCopy(ExtCodeCopy),
    /// Get size of output data from the previous call from the current environment.
    ReturnDataSize(ReturnDataSize),
    /// Copy output data from the previous call to memory.
    ReturnDataCopy(ReturnDataCopy),
    /// Get hash of an account’s code.
    ExtCodeHash(ExtCodeHash),
    /// Get the hash of one of the 256 most recent complete blocks.
    BlockHash(BlockHash),
    /// Get the block’s beneficiary address.
    CoinBase(CoinBase),
    /// Get the block’s timestamp.
    Timestamp(Timestamp),
    /// Get the block’s number.
    Number(Number),
    /// Get the block’s difficulty.
    PrevRandao(PrevRandao),
    /// Get the block’s gas limit.
    GasLimit(GasLimit),
    /// Get the chain ID.
    ChainId(ChainId),
    /// Get balance of currently executing account.
    SelfBalance(SelfBalance),
    /// Get the base fee.
    BaseFee(BaseFee),
    /// Get versioned hashes.
    BlobHash(BlobHash),
    /// Returns the value of the blob base-fee of the current block.
    BlobBaseFee(BlobBaseFee),
    /// Remove item from stack.
    Pop(Pop),
    /// Load word from memory.
    MLoad(MLoad),
    /// Save word to memory.
    MStore(MStore),
    /// Save byte to memory.
    MStore8(MStore8),
    /// Load word from storage.
    SLoad(SLoad),
    /// Save word to storage.
    SStore(SStore),
    /// Alter the program counter.
    Jump(Jump),
    /// Conditionally alter the program counter.
    JumpI(JumpI),
    /// Get the value of the program counter prior to the increment corresponding to this instruction.
    Pc(Pc),
    /// Get the size of active memory in bytes.
    MSize(MSize),
    /// Get the amount of available gas, including the corresponding reduction for the cost of this instruction.
    Gas(Gas),
    /// Mark a valid destination for jumps.
    JumpDest(JumpDest),
    /// Load word from transient storage.
    TLoad(TLoad),
    /// Save word to transient storage.
    TStore(TStore),
    /// Copy memory areas.
    MCopy(MCopy),
    /// Place value 0 on stack.
    Push0(Push<0>),
    /// Place 1 byte item on stack.
    Push1(Push<1>),
    /// Place 2 byte item on stack.
    Push2(Push<2>),
    /// Place 3 byte item on stack.
    Push3(Push<3>),
    /// Place 4 byte item on stack.
    Push4(Push<4>),
    /// Place 5 byte item on stack.
    Push5(Push<5>),
    /// Place 6 byte item on stack.
    Push6(Push<6>),
    /// Place 7 byte item on stack.
    Push7(Push<7>),
    /// Place 8 byte item on stack.
    Push8(Push<8>),
    /// Place 9 byte item on stack.
    Push9(Push<9>),
    /// Place 10 byte item on stack.
    Push10(Push<10>),
    /// Place 11 byte item on stack.
    Push11(Push<11>),
    /// Place 12 byte item on stack.
    Push12(Push<12>),
    /// Place 13 byte item on stack.
    Push13(Push<13>),
    /// Place 14 byte item on stack.
    Push14(Push<14>),
    /// Place 15 byte item on stack.
    Push15(Push<15>),
    /// Place 16 byte item on stack.
    Push16(Push<16>),
    /// Place 17 byte item on stack.
    Push17(Push<17>),
    /// Place 18 byte item on stack.
    Push18(Push<18>),
    /// Place 19 byte item on stack.
    Push19(Push<19>),
    /// Place 20 byte item on stack.
    Push20(Push<20>),
    /// Place 21 byte item on stack.
    Push21(Push<21>),
    /// Place 22 byte item on stack.
    Push22(Push<22>),
    /// Place 23 byte item on stack.
    Push23(Push<23>),
    /// Place 24 byte item on stack.
    Push24(Push<24>),
    /// Place 25 byte item on stack.
    Push25(Push<25>),
    /// Place 26 byte item on stack.
    Push26(Push<26>),
    /// Place 27 byte item on stack.
    Push27(Push<27>),
    /// Place 28 byte item on stack.
    Push28(Push<28>),
    /// Place 29 byte item on stack.
    Push29(Push<29>),
    /// Place 30 byte item on stack.
    Push30(Push<30>),
    /// Place 31 byte item on stack.
    Push31(Push<31>),
    /// Place 32 byte item on stack.
    Push32(Push<32>),
    /// Duplicate 1st stack item.
    Dup1(Dup<1>),
    /// Duplicate 2nd stack item.
    Dup2(Dup<2>),
    /// Duplicate 3rd stack item.
    Dup3(Dup<3>),
    /// Duplicate 4th stack item.
    Dup4(Dup<4>),
    /// Duplicate 5th stack item.
    Dup5(Dup<5>),
    /// Duplicate 6th stack item.
    Dup6(Dup<6>),
    /// Duplicate 7th stack item.
    Dup7(Dup<7>),
    /// Duplicate 8th stack item.
    Dup8(Dup<8>),
    /// Duplicate 9th stack item.
    Dup9(Dup<9>),
    /// Duplicate 10th stack item.
    Dup10(Dup<10>),
    /// Duplicate 11th stack item.
    Dup11(Dup<11>),
    /// Duplicate 12th stack item.
    Dup12(Dup<12>),
    /// Duplicate 13th stack item.
    Dup13(Dup<13>),
    /// Duplicate 14th stack item.
    Dup14(Dup<14>),
    /// Duplicate 15th stack item.
    Dup15(Dup<15>),
    /// Duplicate 16th stack item.
    Dup16(Dup<16>),
    /// Exchange 1st and 2nd stack items.
    Swap1(Swap<1>),
    /// Exchange 1st and 3rd stack items.
    Swap2(Swap<2>),
    /// Exchange 1st and 4th stack items.
    Swap3(Swap<3>),
    /// Exchange 1st and 5th stack items.
    Swap4(Swap<4>),
    /// Exchange 1st and 6th stack items.
    Swap5(Swap<5>),
    /// Exchange 1st and 7th stack items.
    Swap6(Swap<6>),
    /// Exchange 1st and 8th stack items.
    Swap7(Swap<7>),
    /// Exchange 1st and 9th stack items.
    Swap8(Swap<8>),
    /// Exchange 1st and 10th stack items.
    Swap9(Swap<9>),
    /// Exchange 1st and 11th stack items.
    Swap10(Swap<10>),
    /// Exchange 1st and 12th stack items.
    Swap11(Swap<11>),
    /// Exchange 1st and 13th stack items.
    Swap12(Swap<12>),
    /// Exchange 1st and 14th stack items.
    Swap13(Swap<13>),
    /// Exchange 1st and 15th stack items.
    Swap14(Swap<14>),
    /// Exchange 1st and 16th stack items.
    Swap15(Swap<15>),
    /// Exchange 1st and 17th stack items.
    Swap16(Swap<16>),
    /// Append log record with no topics.
    Log0(Log<0>),
    /// Append log record with one topic.
    Log1(Log<1>),
    /// Append log record with two topics.
    Log2(Log<2>),
    /// Append log record with three topics.
    Log3(Log<3>),
    /// Append log record with four topics.
    Log4(Log<4>),
    /// Create a new account with associated code.
    Create(Create),
    /// Message-call into an account.
    Call(Call),
    /// Message-call into this account with alternative account’s code.
    CallCode(CallCode),
    /// Halt execution returning output data.
    Return(Return),
    /// Message-call into this account with an alternative account’s code, but persisting the current values for sender and value.
    DelegateCall(DelegateCall),
    /// Create a new account with associated code at a predictable address.
    Create2(Create2),
    /// Static message-call into an account.
    StaticCall(StaticCall),
    /// Halt execution reverting state changes but returning data and remaining gas.
    Revert(Revert),
    /// Designated invalid instruction.
    Invalid(Invalid),
    /// Halt execution and register account for later deletion or send all Ether to address (post-Cancun).
    SelfDestruct(SelfDestruct),
    /// Unidentified instruction.
    Unknown(Unknown),
}

/// Match each instruction variant and call the specified function passing the instruction as the
/// first argument and any other specified arguments after.
#[macro_export]
macro_rules! for_each_instruction {
    ($instr: ident, $fn: path $(,$arg: expr)*) => {{
        use $crate::instruction::Instruction;

        match $instr {
            Instruction::Stop(i) => $fn(i$(,$arg)*),
            Instruction::Add(i) => $fn(i$(,$arg)*),
            Instruction::Mul(i) => $fn(i$(,$arg)*),
            Instruction::Sub(i) => $fn(i$(,$arg)*),
            Instruction::Div(i) => $fn(i$(,$arg)*),
            Instruction::SDiv(i) => $fn(i$(,$arg)*),
            Instruction::Mod(i) => $fn(i$(,$arg)*),
            Instruction::SMod(i) => $fn(i$(,$arg)*),
            Instruction::AddMod(i) => $fn(i$(,$arg)*),
            Instruction::MulMod(i) => $fn(i$(,$arg)*),
            Instruction::Exp(i) => $fn(i$(,$arg)*),
            Instruction::SignExtend(i) => $fn(i$(,$arg)*),
            Instruction::Lt(i) => $fn(i$(,$arg)*),
            Instruction::Gt(i) => $fn(i$(,$arg)*),
            Instruction::SLt(i) => $fn(i$(,$arg)*),
            Instruction::SGt(i) => $fn(i$(,$arg)*),
            Instruction::Eq(i) => $fn(i$(,$arg)*),
            Instruction::IsZero(i) => $fn(i$(,$arg)*),
            Instruction::And(i) => $fn(i$(,$arg)*),
            Instruction::Or(i) => $fn(i$(,$arg)*),
            Instruction::Xor(i) => $fn(i$(,$arg)*),
            Instruction::Not(i) => $fn(i$(,$arg)*),
            Instruction::Byte(i) => $fn(i$(,$arg)*),
            Instruction::Shl(i) => $fn(i$(,$arg)*),
            Instruction::Shr(i) => $fn(i$(,$arg)*),
            Instruction::Sar(i) => $fn(i$(,$arg)*),
            Instruction::Keccak256(i) => $fn(i$(,$arg)*),
            Instruction::Address(i) => $fn(i$(,$arg)*),
            Instruction::Balance(i) => $fn(i$(,$arg)*),
            Instruction::Origin(i) => $fn(i$(,$arg)*),
            Instruction::Caller(i) => $fn(i$(,$arg)*),
            Instruction::CallValue(i) => $fn(i$(,$arg)*),
            Instruction::CallDataLoad(i) => $fn(i$(,$arg)*),
            Instruction::CallDataSize(i) => $fn(i$(,$arg)*),
            Instruction::CallDataCopy(i) => $fn(i$(,$arg)*),
            Instruction::CodeSize(i) => $fn(i$(,$arg)*),
            Instruction::CodeCopy(i) => $fn(i$(,$arg)*),
            Instruction::GasPrice(i) => $fn(i$(,$arg)*),
            Instruction::ExtCodeSize(i) => $fn(i$(,$arg)*),
            Instruction::ExtCodeCopy(i) => $fn(i$(,$arg)*),
            Instruction::ReturnDataSize(i) => $fn(i$(,$arg)*),
            Instruction::ReturnDataCopy(i) => $fn(i$(,$arg)*),
            Instruction::ExtCodeHash(i) => $fn(i$(,$arg)*),
            Instruction::BlockHash(i) => $fn(i$(,$arg)*),
            Instruction::CoinBase(i) => $fn(i$(,$arg)*),
            Instruction::Timestamp(i) => $fn(i$(,$arg)*),
            Instruction::Number(i) => $fn(i$(,$arg)*),
            Instruction::PrevRandao(i) => $fn(i$(,$arg)*),
            Instruction::GasLimit(i) => $fn(i$(,$arg)*),
            Instruction::ChainId(i) => $fn(i$(,$arg)*),
            Instruction::SelfBalance(i) => $fn(i$(,$arg)*),
            Instruction::BaseFee(i) => $fn(i$(,$arg)*),
            Instruction::BlobHash(i) => $fn(i$(,$arg)*),
            Instruction::BlobBaseFee(i) => $fn(i$(,$arg)*),
            Instruction::Pop(i) => $fn(i$(,$arg)*),
            Instruction::MLoad(i) => $fn(i$(,$arg)*),
            Instruction::MStore(i) => $fn(i$(,$arg)*),
            Instruction::MStore8(i) => $fn(i$(,$arg)*),
            Instruction::SLoad(i) => $fn(i$(,$arg)*),
            Instruction::SStore(i) => $fn(i$(,$arg)*),
            Instruction::Jump(i) => $fn(i$(,$arg)*),
            Instruction::JumpI(i) => $fn(i$(,$arg)*),
            Instruction::Pc(i) => $fn(i$(,$arg)*),
            Instruction::MSize(i) => $fn(i$(,$arg)*),
            Instruction::Gas(i) => $fn(i$(,$arg)*),
            Instruction::JumpDest(i) => $fn(i$(,$arg)*),
            Instruction::TLoad(i) => $fn(i$(,$arg)*),
            Instruction::TStore(i) => $fn(i$(,$arg)*),
            Instruction::MCopy(i) => $fn(i$(,$arg)*),
            Instruction::Push0(i) => $fn(i$(,$arg)*),
            Instruction::Push1(i) => $fn(i$(,$arg)*),
            Instruction::Push2(i) => $fn(i$(,$arg)*),
            Instruction::Push3(i) => $fn(i$(,$arg)*),
            Instruction::Push4(i) => $fn(i$(,$arg)*),
            Instruction::Push5(i) => $fn(i$(,$arg)*),
            Instruction::Push6(i) => $fn(i$(,$arg)*),
            Instruction::Push7(i) => $fn(i$(,$arg)*),
            Instruction::Push8(i) => $fn(i$(,$arg)*),
            Instruction::Push9(i) => $fn(i$(,$arg)*),
            Instruction::Push10(i) => $fn(i$(,$arg)*),
            Instruction::Push11(i) => $fn(i$(,$arg)*),
            Instruction::Push12(i) => $fn(i$(,$arg)*),
            Instruction::Push13(i) => $fn(i$(,$arg)*),
            Instruction::Push14(i) => $fn(i$(,$arg)*),
            Instruction::Push15(i) => $fn(i$(,$arg)*),
            Instruction::Push16(i) => $fn(i$(,$arg)*),
            Instruction::Push17(i) => $fn(i$(,$arg)*),
            Instruction::Push18(i) => $fn(i$(,$arg)*),
            Instruction::Push19(i) => $fn(i$(,$arg)*),
            Instruction::Push20(i) => $fn(i$(,$arg)*),
            Instruction::Push21(i) => $fn(i$(,$arg)*),
            Instruction::Push22(i) => $fn(i$(,$arg)*),
            Instruction::Push23(i) => $fn(i$(,$arg)*),
            Instruction::Push24(i) => $fn(i$(,$arg)*),
            Instruction::Push25(i) => $fn(i$(,$arg)*),
            Instruction::Push26(i) => $fn(i$(,$arg)*),
            Instruction::Push27(i) => $fn(i$(,$arg)*),
            Instruction::Push28(i) => $fn(i$(,$arg)*),
            Instruction::Push29(i) => $fn(i$(,$arg)*),
            Instruction::Push30(i) => $fn(i$(,$arg)*),
            Instruction::Push31(i) => $fn(i$(,$arg)*),
            Instruction::Push32(i) => $fn(i$(,$arg)*),
            Instruction::Dup1(i) => $fn(i$(,$arg)*),
            Instruction::Dup2(i) => $fn(i$(,$arg)*),
            Instruction::Dup3(i) => $fn(i$(,$arg)*),
            Instruction::Dup4(i) => $fn(i$(,$arg)*),
            Instruction::Dup5(i) => $fn(i$(,$arg)*),
            Instruction::Dup6(i) => $fn(i$(,$arg)*),
            Instruction::Dup7(i) => $fn(i$(,$arg)*),
            Instruction::Dup8(i) => $fn(i$(,$arg)*),
            Instruction::Dup9(i) => $fn(i$(,$arg)*),
            Instruction::Dup10(i) => $fn(i$(,$arg)*),
            Instruction::Dup11(i) => $fn(i$(,$arg)*),
            Instruction::Dup12(i) => $fn(i$(,$arg)*),
            Instruction::Dup13(i) => $fn(i$(,$arg)*),
            Instruction::Dup14(i) => $fn(i$(,$arg)*),
            Instruction::Dup15(i) => $fn(i$(,$arg)*),
            Instruction::Dup16(i) => $fn(i$(,$arg)*),
            Instruction::Swap1(i) => $fn(i$(,$arg)*),
            Instruction::Swap2(i) => $fn(i$(,$arg)*),
            Instruction::Swap3(i) => $fn(i$(,$arg)*),
            Instruction::Swap4(i) => $fn(i$(,$arg)*),
            Instruction::Swap5(i) => $fn(i$(,$arg)*),
            Instruction::Swap6(i) => $fn(i$(,$arg)*),
            Instruction::Swap7(i) => $fn(i$(,$arg)*),
            Instruction::Swap8(i) => $fn(i$(,$arg)*),
            Instruction::Swap9(i) => $fn(i$(,$arg)*),
            Instruction::Swap10(i) => $fn(i$(,$arg)*),
            Instruction::Swap11(i) => $fn(i$(,$arg)*),
            Instruction::Swap12(i) => $fn(i$(,$arg)*),
            Instruction::Swap13(i) => $fn(i$(,$arg)*),
            Instruction::Swap14(i) => $fn(i$(,$arg)*),
            Instruction::Swap15(i) => $fn(i$(,$arg)*),
            Instruction::Swap16(i) => $fn(i$(,$arg)*),
            Instruction::Log0(i) => $fn(i$(,$arg)*),
            Instruction::Log1(i) => $fn(i$(,$arg)*),
            Instruction::Log2(i) => $fn(i$(,$arg)*),
            Instruction::Log3(i) => $fn(i$(,$arg)*),
            Instruction::Log4(i) => $fn(i$(,$arg)*),
            Instruction::Create(i) => $fn(i$(,$arg)*),
            Instruction::Call(i) => $fn(i$(,$arg)*),
            Instruction::CallCode(i) => $fn(i$(,$arg)*),
            Instruction::Return(i) => $fn(i$(,$arg)*),
            Instruction::DelegateCall(i) => $fn(i$(,$arg)*),
            Instruction::Create2(i) => $fn(i$(,$arg)*),
            Instruction::StaticCall(i) => $fn(i$(,$arg)*),
            Instruction::Revert(i) => $fn(i$(,$arg)*),
            Instruction::Invalid(i) => $fn(i$(,$arg)*),
            Instruction::SelfDestruct(i) => $fn(i$(,$arg)*),
            Instruction::Unknown(i) => $fn(i$(,$arg)*),
        }
    }};
}

impl InstructionMeta for Instruction {
    fn opcode(&self) -> OpCode {
        for_each_instruction!(self, InstructionMeta::opcode)
    }
}
