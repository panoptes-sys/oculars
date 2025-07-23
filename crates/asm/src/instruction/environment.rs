//! Environmental information.

use super::KnownInstruction;
use crate::{instruction::InstructionMeta, opcode::Mnemonic};
use derive_more::Display;

/// Get address of currently executing account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Address;

impl KnownInstruction for Address {
    const MNEMONIC: Mnemonic = Mnemonic::ADDRESS;
}

/// Get balance of the given account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Balance;

impl KnownInstruction for Balance {
    const MNEMONIC: Mnemonic = Mnemonic::BALANCE;
}

/// Get execution origination address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Origin;

impl KnownInstruction for Origin {
    const MNEMONIC: Mnemonic = Mnemonic::ORIGIN;
}

/// Get caller address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Caller;

impl KnownInstruction for Caller {
    const MNEMONIC: Mnemonic = Mnemonic::CALLER;
}

/// Get deposited value by the instruction/transaction responsible for this execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallValue;

impl KnownInstruction for CallValue {
    const MNEMONIC: Mnemonic = Mnemonic::CALLVALUE;
}

/// Get input data of current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallDataLoad;

impl KnownInstruction for CallDataLoad {
    const MNEMONIC: Mnemonic = Mnemonic::CALLDATALOAD;
}

/// Get size of input data in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallDataSize;

impl KnownInstruction for CallDataSize {
    const MNEMONIC: Mnemonic = Mnemonic::CALLDATASIZE;
}

/// Copy input data in current environment to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CallDataCopy;

impl KnownInstruction for CallDataCopy {
    const MNEMONIC: Mnemonic = Mnemonic::CALLDATACOPY;
}

/// Get size of code running in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CodeSize;

impl KnownInstruction for CodeSize {
    const MNEMONIC: Mnemonic = Mnemonic::CODESIZE;
}

/// Copy code running in current environment to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CodeCopy;

impl KnownInstruction for CodeCopy {
    const MNEMONIC: Mnemonic = Mnemonic::CODECOPY;
}

/// Get price of gas in current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct GasPrice;

impl KnownInstruction for GasPrice {
    const MNEMONIC: Mnemonic = Mnemonic::GASPRICE;
}

/// Get size of an account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ExtCodeSize;

impl KnownInstruction for ExtCodeSize {
    const MNEMONIC: Mnemonic = Mnemonic::EXTCODESIZE;
}

/// Copy an account’s code to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ExtCodeCopy;

impl KnownInstruction for ExtCodeCopy {
    const MNEMONIC: Mnemonic = Mnemonic::EXTCODECOPY;
}

/// Get size of output data from the previous call from the current environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ReturnDataSize;

impl KnownInstruction for ReturnDataSize {
    const MNEMONIC: Mnemonic = Mnemonic::RETURNDATASIZE;
}

/// Copy output data from the previous call to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ReturnDataCopy;

impl KnownInstruction for ReturnDataCopy {
    const MNEMONIC: Mnemonic = Mnemonic::RETURNDATACOPY;
}

/// Get hash of an account’s code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ExtCodeHash;

impl KnownInstruction for ExtCodeHash {
    const MNEMONIC: Mnemonic = Mnemonic::EXTCODEHASH;
}
