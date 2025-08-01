//! EIP-2718: Typed Transaction Envelope.
//!
//! ## Abstract
//! `TransactionType || TransactionPayload` is a valid transaction and `TransactionType || ReceiptPayload` is a valid transaction receipt where `TransactionType` identifies the format of the transaction and `*Payload` is the transaction/receipt contents, which are defined in future EIPs.
//!
//! ## Motivation
//! In the past, when we have wanted to add new transaction types we have had to ensure they were backward compatible with all other transactions, meaning that you could differentiate them based only on the encoded payload, and it was not possible to have a transaction that matched both types.
//! This was seen in [EIP-155](./eip-155.md) where the new value was bit-packed into one of the encoded fields.
//! There are multiple proposals in discussion that define new transaction types such as one that allows EOA accounts to execute code directly within their context, one that enables someone besides `msg.sender` to pay for gas, and proposals related to layer 1 multi-sig transactions.
//! These all need to be defined in a way that is mutually compatible, which quickly becomes burdensome to EIP authors and to clients who now have to follow complex rules for differentiating transaction type.
//!
//! By introducing an envelope transaction type, we only need to ensure backward compatibility with existing transactions and from then on we just need to solve the much simpler problem of ensuring there is no numbering conflict between `TransactionType`s.
//!
//! ## Specification
//! ### Definitions
//! * `||` is the byte/byte-array concatenation operator.
//!
//! ### Transactions
//! As of `FORK_BLOCK_NUMBER`, the transaction root in the block header **MUST** be the root hash of `patriciaTrie(rlp(Index) => Transaction)` where:
//! * `Index` is the index in the block of this transaction
//! * `Transaction` is either `TransactionType || TransactionPayload` or `LegacyTransaction`
//! * `TransactionType` is a positive unsigned 8-bit number between `0` and `0x7f` that represents the type of the transaction
//! * `TransactionPayload` is an opaque byte array whose interpretation is dependent on the `TransactionType` and defined in future EIPs
//! * `LegacyTransaction` is `rlp([nonce, gasPrice, gasLimit, to, value, data, v, r, s])`
//!
//! All signatures for future transaction types **SHOULD** include the `TransactionType` as the first byte of the signed data.
//! This makes it so we do not have to worry about signatures for one transaction type being used as signatures for a different transaction type.
//!
//! ### Receipts
//! As of `FORK_BLOCK_NUMBER`, the receipt root in the block header **MUST** be the root hash of `patriciaTrie(rlp(Index) => Receipt)` where:
//! * `Index` is the index in the block of the transaction this receipt is for
//! * `Receipt` is either `TransactionType || ReceiptPayload` or `LegacyReceipt`
//! * `TransactionType` is a positive unsigned 8-bit number between `0` and `0x7f` that represents the type of the transaction
//! * `ReceiptPayload` is an opaque byte array whose interpretation is dependent on the `TransactionType` and defined in future EIPs
//! * `LegacyReceipt` is `rlp([status, cumulativeGasUsed, logsBloom, logs])`
//!
//! The `TransactionType` of the receipt **MUST** match the `TransactionType` of the transaction with a matching `Index`.
//!
//! ## Rationale
//! ### `TransactionType` only goes up to 0x7f
//! For the forseable future, 0x7f is plenty and it leaves open a number of options for extending the range such as using the high bit as a continuation bit.
//! This also prevents us from colliding with legacy transaction types, which always start with a byte `>= 0xc0`.
//! ### **SHOULD** instead of **MUST** for the `TransactionType` being first byte of signed data
//! While it is strongly recommended that all future transactions sign the first byte to ensure that there is no problem with signature reuse, the authors acknowledge that this may not always make sense or be possible.
//! One example where this isn't possible is wrapped legacy transactions that are signature compatible with the legacy signing scheme.
//! Another potential situation is one where transactions don't have a signature in the traditional sense and instead have some other mechanism for determining validity.
//! ### `TransactionType` selection algorithm
//! There was discussion about defining the `TransactionType` identifier assignment/selection algorithm in this standard.
//! While it would be nice to have a standardized mechanism for assignment, at the time of writing of this standard there is not a strong need for it so it was deemed out of scope.
//! A future EIP may introduce a standard for `TransactionType` identifier assignment if it is deemed necessary.
//! ### Opaque byte array rather than an RLP array
//! By having the second byte on be opaque bytes, rather than an RLP (or other encoding) list, we can support different encoding formats for the transaction payload in the future such as SSZ, LEB128, or a fixed width format.
//! ### ORIGIN and CALLER
//! There was discussion about having ORIGIN and CALLER opcodes become dependent on the transaction type, so that each transaction type could define what those opcodes returned.
//! However, there is a desire to make transaction type opaque to the contracts to discourage contracts treating different types of transactions differently.
//! There also were concerns over backward compatibility with existing contracts which make assumptions about ORIGIN and CALLER opcodes.
//! Going forward, we will assume that all transaction types will have an address that reasonably represents a `CALLER` of the first EVM frame and `ORIGIN` will be the same address in all cases.
//! If a transaction type needs to supply additional information to contracts, they will need a new opcode.
//!
//! Micah Zoltu (@`MicahZoltu`), "EIP-2718: Typed Transaction Envelope," Ethereum Improvement Proposals, no. 2718, June 2020. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-2718>.

use crate::eip::Eip;

/// EIP-2718: Typed Transaction Envelope.
pub struct Eip2718;

impl Eip for Eip2718 {
    const NUMBER: u32 = 2718;
}
