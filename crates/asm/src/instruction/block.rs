//! Block information.

use crate::{instruction::InstructionMeta, opcode::OpCode};

/// Get the hash of one of the 256 most recent complete blocks.
pub struct BlockHash;

impl InstructionMeta for BlockHash {
    const OPCODE: OpCode = OpCode::BLOCKHASH;
}

/// Get the block’s beneficiary address.
pub struct CoinBase;

impl InstructionMeta for CoinBase {
    const OPCODE: OpCode = OpCode::COINBASE;
}

/// Get the block’s timestamp.
pub struct Timestamp;

impl InstructionMeta for Timestamp {
    const OPCODE: OpCode = OpCode::TIMESTAMP;
}

/// Get the block’s number.
pub struct Number;

impl InstructionMeta for Number {
    const OPCODE: OpCode = OpCode::NUMBER;
}

/// Get the block’s difficulty.
pub struct PrevRandao;

impl InstructionMeta for PrevRandao {
    const OPCODE: OpCode = OpCode::PREVRANDAO;
}

/// Get the block’s gas limit.
pub struct GasLimit;

impl InstructionMeta for GasLimit {
    const OPCODE: OpCode = OpCode::GASLIMIT;
}

/// Get the chain ID.
pub struct ChainId;

impl InstructionMeta for ChainId {
    const OPCODE: OpCode = OpCode::CHAINID;
}

/// Get balance of currently executing account.
pub struct SelfBalance;

impl InstructionMeta for SelfBalance {
    const OPCODE: OpCode = OpCode::SELFBALANCE;
}

/// Get the base fee.
pub struct BaseFee;

impl InstructionMeta for BaseFee {
    const OPCODE: OpCode = OpCode::BASEFEE;
}

/// Get versioned hashes.
pub struct BlobHash;

impl InstructionMeta for BlobHash {
    const OPCODE: OpCode = OpCode::BLOBHASH;
}

/// Returns the value of the blob base-fee of the current block.
pub struct BlobBaseFee;

impl InstructionMeta for BlobBaseFee {
    const OPCODE: OpCode = OpCode::BLOBBASEFEE;
}
