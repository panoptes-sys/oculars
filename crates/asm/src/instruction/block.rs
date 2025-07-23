//! Block information.

use super::KnownInstruction;
use crate::{instruction::InstructionMeta, opcode::Mnemonic};
use derive_more::Display;

/// Get the hash of one of the 256 most recent complete blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BlockHash;

impl KnownInstruction for BlockHash {
    const MNEMONIC: Mnemonic = Mnemonic::BLOCKHASH;
}

/// Get the block’s beneficiary address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct CoinBase;

impl KnownInstruction for CoinBase {
    const MNEMONIC: Mnemonic = Mnemonic::COINBASE;
}

/// Get the block’s timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Timestamp;

impl KnownInstruction for Timestamp {
    const MNEMONIC: Mnemonic = Mnemonic::TIMESTAMP;
}

/// Get the block’s number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct Number;

impl KnownInstruction for Number {
    const MNEMONIC: Mnemonic = Mnemonic::NUMBER;
}

/// Get the block’s difficulty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct PrevRandao;

impl KnownInstruction for PrevRandao {
    const MNEMONIC: Mnemonic = Mnemonic::PREVRANDAO;
}

/// Get the block’s gas limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct GasLimit;

impl KnownInstruction for GasLimit {
    const MNEMONIC: Mnemonic = Mnemonic::GASLIMIT;
}

/// Get the chain ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct ChainId;

impl KnownInstruction for ChainId {
    const MNEMONIC: Mnemonic = Mnemonic::CHAINID;
}

/// Get balance of currently executing account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct SelfBalance;

impl KnownInstruction for SelfBalance {
    const MNEMONIC: Mnemonic = Mnemonic::SELFBALANCE;
}

/// Get the base fee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BaseFee;

impl KnownInstruction for BaseFee {
    const MNEMONIC: Mnemonic = Mnemonic::BASEFEE;
}

/// Get versioned hashes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BlobHash;

impl KnownInstruction for BlobHash {
    const MNEMONIC: Mnemonic = Mnemonic::BLOBHASH;
}

/// Returns the value of the blob base-fee of the current block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{}", self.opcode())]
pub struct BlobBaseFee;

impl KnownInstruction for BlobBaseFee {
    const MNEMONIC: Mnemonic = Mnemonic::BLOBBASEFEE;
}
