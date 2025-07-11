//! Block information.

use derive_more::Display;

use crate::{
    instruction::Instruction,
    opcode::{Mnemonic, OpCode},
};

/// Get the hash of one of the 256 most recent complete blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BlockHash;

impl Instruction for BlockHash {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BLOCKHASH)
    }
}

/// Get the block’s beneficiary address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CoinBase;

impl Instruction for CoinBase {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::COINBASE)
    }
}

/// Get the block’s timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Timestamp;

impl Instruction for Timestamp {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::TIMESTAMP)
    }
}

/// Get the block’s number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Number;

impl Instruction for Number {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::NUMBER)
    }
}

/// Get the block’s difficulty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct PrevRandao;

impl Instruction for PrevRandao {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::PREVRANDAO)
    }
}

/// Get the block’s gas limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct GasLimit;

impl Instruction for GasLimit {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::GASLIMIT)
    }
}

/// Get the chain ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ChainId;

impl Instruction for ChainId {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::CHAINID)
    }
}

/// Get balance of currently executing account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SelfBalance;

impl Instruction for SelfBalance {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::SELFBALANCE)
    }
}

/// Get the base fee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BaseFee;

impl Instruction for BaseFee {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BASEFEE)
    }
}

/// Get versioned hashes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BlobHash;

impl Instruction for BlobHash {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BLOBHASH)
    }
}

/// Returns the value of the blob base-fee of the current block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BlobBaseFee;

impl Instruction for BlobBaseFee {
    fn opcode(&self) -> OpCode {
        OpCode::Known(Mnemonic::BLOBBASEFEE)
    }
}
