//! Environmental information.

use crate::{
    instruction::InstructionMeta,
    opcode::{Mnemonic, OpCode},
};

/// Get address of currently executing account.
pub struct Address;

impl InstructionMeta for Address {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ADDRESS)
    }
}

/// Get balance of the given account.
pub struct Balance;

impl InstructionMeta for Balance {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BALANCE)
    }
}

/// Get execution origination address.
pub struct Origin;

impl InstructionMeta for Origin {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::ORIGIN)
    }
}

/// Get caller address.
pub struct Caller;

impl InstructionMeta for Caller {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLER)
    }
}

/// Get deposited value by the instruction/transaction responsible for this execution.
pub struct CallValue;

impl InstructionMeta for CallValue {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLVALUE)
    }
}

/// Get input data of current environment.
pub struct CallDataLoad;

impl InstructionMeta for CallDataLoad {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATALOAD)
    }
}

/// Get size of input data in current environment.
pub struct CallDataSize;

impl InstructionMeta for CallDataSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATASIZE)
    }
}

/// Copy input data in current environment to memory.
pub struct CallDataCopy;

impl InstructionMeta for CallDataCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CALLDATACOPY)
    }
}

/// Get size of code running in current environment.
pub struct CodeSize;

impl InstructionMeta for CodeSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CODESIZE)
    }
}

/// Copy code running in current environment to memory.
pub struct CodeCopy;

impl InstructionMeta for CodeCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CODECOPY)
    }
}

/// Get price of gas in current environment.
pub struct GasPrice;

impl InstructionMeta for GasPrice {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::GASPRICE)
    }
}

/// Get size of an account’s code.
pub struct ExtCodeSize;

impl InstructionMeta for ExtCodeSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODESIZE)
    }
}

/// Copy an account’s code to memory.
pub struct ExtCodeCopy;

impl InstructionMeta for ExtCodeCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODECOPY)
    }
}

/// Get size of output data from the previous call from the current environment.
pub struct ReturnDataSize;

impl InstructionMeta for ReturnDataSize {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURNDATASIZE)
    }
}

/// Copy output data from the previous call to memory.
pub struct ReturnDataCopy;

impl InstructionMeta for ReturnDataCopy {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::RETURNDATACOPY)
    }
}

/// Get hash of an account’s code.
pub struct ExtCodeHash;

impl InstructionMeta for ExtCodeHash {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::EXTCODEHASH)
    }
}
