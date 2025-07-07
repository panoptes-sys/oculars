//! Environmental information.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Get address of currently executing account.
pub struct Address;

impl InstructionMeta for Address {
    const OPCODE: OpCode = OpCode::ADDRESS;
}

/// Get balance of the given account.
pub struct Balance;

impl InstructionMeta for Balance {
    const OPCODE: OpCode = OpCode::BALANCE;
}

/// Get execution origination address.
pub struct Origin;

impl InstructionMeta for Origin {
    const OPCODE: OpCode = OpCode::ORIGIN;
}

/// Get caller address.
pub struct Caller;

impl InstructionMeta for Caller {
    const OPCODE: OpCode = OpCode::CALLER;
}

/// Get deposited value by the instruction/transaction responsible for this execution.
pub struct CallValue;

impl InstructionMeta for CallValue {
    const OPCODE: OpCode = OpCode::CALLVALUE;
}

/// Get input data of current environment.
pub struct CallDataLoad;

impl InstructionMeta for CallDataLoad {
    const OPCODE: OpCode = OpCode::CALLDATALOAD;
}

/// Get size of input data in current environment.
pub struct CallDataSize;

impl InstructionMeta for CallDataSize {
    const OPCODE: OpCode = OpCode::CALLDATASIZE;
}

/// Copy input data in current environment to memory.
pub struct CallDataCopy;

impl InstructionMeta for CallDataCopy {
    const OPCODE: OpCode = OpCode::CALLDATACOPY;
}

/// Get size of code running in current environment.
pub struct CodeSize;

impl InstructionMeta for CodeSize {
    const OPCODE: OpCode = OpCode::CODESIZE;
}

/// Copy code running in current environment to memory.
pub struct CodeCopy;

impl InstructionMeta for CodeCopy {
    const OPCODE: OpCode = OpCode::CODECOPY;
}

/// Get price of gas in current environment.
pub struct GasPrice;

impl InstructionMeta for GasPrice {
    const OPCODE: OpCode = OpCode::GASPRICE;
}

/// Get size of an account’s code.
pub struct ExtCodeSize;

impl InstructionMeta for ExtCodeSize {
    const OPCODE: OpCode = OpCode::EXTCODESIZE;
}

/// Copy an account’s code to memory.
pub struct ExtCodeCopy;

impl InstructionMeta for ExtCodeCopy {
    const OPCODE: OpCode = OpCode::EXTCODECOPY;
}

/// Get size of output data from the previous call from the current environment.
pub struct ReturnDataSize;

impl InstructionMeta for ReturnDataSize {
    const OPCODE: OpCode = OpCode::RETURNDATASIZE;
}

/// Copy output data from the previous call to memory.
pub struct ReturnDataCopy;

impl InstructionMeta for ReturnDataCopy {
    const OPCODE: OpCode = OpCode::RETURNDATACOPY;
}

/// Get hash of an account’s code.
pub struct ExtCodeHash;

impl InstructionMeta for ExtCodeHash {
    const OPCODE: OpCode = OpCode::EXTCODEHASH;
}
