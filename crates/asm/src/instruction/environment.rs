//! Environmental information.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Get address of currently executing account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Address;

impl Instruction for Address {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ADDRESS)
    }
}

/// Get balance of the given account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Balance;

impl Instruction for Balance {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BALANCE)
    }
}

/// Get execution origination address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Origin;

impl Instruction for Origin {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ORIGIN)
    }
}

/// Get caller address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Caller;

impl Instruction for Caller {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLER)
    }
}

/// Get deposited value by the instruction/transaction responsible for this execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallValue;

impl Instruction for CallValue {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLVALUE)
    }
}

/// Get input data of current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallDataLoad;

impl Instruction for CallDataLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATALOAD)
    }
}

/// Get size of input data in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallDataSize;

impl Instruction for CallDataSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATASIZE)
    }
}

/// Copy input data in current environment to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallDataCopy;

impl Instruction for CallDataCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATACOPY)
    }
}

/// Get size of code running in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CodeSize;

impl Instruction for CodeSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CODESIZE)
    }
}

/// Copy code running in current environment to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CodeCopy;

impl Instruction for CodeCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CODECOPY)
    }
}

/// Get price of gas in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct GasPrice;

impl Instruction for GasPrice {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::GASPRICE)
    }
}

/// Get size of an account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ExtCodeSize;

impl Instruction for ExtCodeSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODESIZE)
    }
}

/// Copy an account’s code to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ExtCodeCopy;

impl Instruction for ExtCodeCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODECOPY)
    }
}

/// Get size of output data from the previous call from the current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ReturnDataSize;

impl Instruction for ReturnDataSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURNDATASIZE)
    }
}

/// Copy output data from the previous call to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ReturnDataCopy;

impl Instruction for ReturnDataCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURNDATACOPY)
    }
}

/// Get hash of an account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ExtCodeHash;

impl Instruction for ExtCodeHash {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODEHASH)
    }
}
