//! Environmental information.

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Get address of currently executing account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address;

impl InstructionMeta for Address {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ADDRESS)
    }
}

/// Get balance of the given account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Balance;

impl InstructionMeta for Balance {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BALANCE)
    }
}

/// Get execution origination address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Origin;

impl InstructionMeta for Origin {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ORIGIN)
    }
}

/// Get caller address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Caller;

impl InstructionMeta for Caller {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLER)
    }
}

/// Get deposited value by the instruction/transaction responsible for this execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallValue;

impl InstructionMeta for CallValue {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLVALUE)
    }
}

/// Get input data of current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallDataLoad;

impl InstructionMeta for CallDataLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATALOAD)
    }
}

/// Get size of input data in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallDataSize;

impl InstructionMeta for CallDataSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATASIZE)
    }
}

/// Copy input data in current environment to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallDataCopy;

impl InstructionMeta for CallDataCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATACOPY)
    }
}

/// Get size of code running in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeSize;

impl InstructionMeta for CodeSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CODESIZE)
    }
}

/// Copy code running in current environment to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeCopy;

impl InstructionMeta for CodeCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CODECOPY)
    }
}

/// Get price of gas in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GasPrice;

impl InstructionMeta for GasPrice {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::GASPRICE)
    }
}

/// Get size of an account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtCodeSize;

impl InstructionMeta for ExtCodeSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODESIZE)
    }
}

/// Copy an account’s code to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtCodeCopy;

impl InstructionMeta for ExtCodeCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODECOPY)
    }
}

/// Get size of output data from the previous call from the current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReturnDataSize;

impl InstructionMeta for ReturnDataSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURNDATASIZE)
    }
}

/// Copy output data from the previous call to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReturnDataCopy;

impl InstructionMeta for ReturnDataCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURNDATACOPY)
    }
}

/// Get hash of an account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtCodeHash;

impl InstructionMeta for ExtCodeHash {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODEHASH)
    }
}
