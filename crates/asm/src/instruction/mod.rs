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

pub use arithmetic::*;
pub use bitwise::*;
pub use block::*;
pub use environment::*;
pub use flow::*;
pub use logging::*;
pub use memory::*;
pub use stack::*;
pub use storage::*;
pub use system::*;

use crate::opcode::OpCode;

/// General instruction information.
pub trait InstructionMeta {
    /// Operation code associated with this instruction.
    const OPCODE: OpCode;

    /// Return the operation code associated with this instruction.
    #[inline]
    fn opcode(&self) -> OpCode {
        Self::OPCODE
    }
}

/// EVM instruction.
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
