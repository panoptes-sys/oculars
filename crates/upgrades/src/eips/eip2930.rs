//! EIP-2930: Optional access lists.
//!
//! ## Simple Summary
//!
//! Adds a transaction type which contains an access list, a list of addresses and storage keys that the transaction plans to access. Accesses outside the list are possible, but become more expensive.
//!
//! ## Abstract
//!
//! We introduce a new [EIP-2718](./eip-2718.md) transaction type, with the format `0x01 || rlp([chainId, nonce, gasPrice, gasLimit, to, value, data, accessList, signatureYParity, signatureR, signatureS])`.
//!
//! The `accessList` specifies a list of addresses and storage keys; these addresses and storage keys are added into the `accessed_addresses` and `accessed_storage_keys` global sets (introduced in [EIP-2929](./eip-2929.md)). A gas cost is charged, though at a discount relative to the cost of accessing outside the list.
//!
//! ## Motivation
//!
//! This EIP serves two functions:
//!
//! 1. Mitigates contract breakage risks introduced by [EIP-2929](./eip-2929.md), as transactions could pre-specify and pre-pay for the accounts and storage slots that the transaction plans to access; as a result, in the actual execution, the SLOAD and EXT* opcodes would only cost 100 gas: low enough that it would not only prevent breakage due to that EIP but also "unstuck" any contracts that became stuck due to EIP 1884.
//! 2. Introduces the access list format and the logic for handling the format. This logic can later be repurposed for many other purposes, including block-wide witnesses, use in `ReGenesis`, moving toward static state access over time, and more.
//!
//! ## Specification
//!
//! ### Definitions
//!
//! **`TransactionType`** `1`.  See [EIP-2718](./eip-2718.md)
//!
//! **`ChainId`** The transaction only valid on networks with this `chainID`.
//!
//! **`YParity`** The parity (0 for even, 1 for odd) of the y-value of a secp256k1 signature.
//!
//!
//! ### Parameters
//!
//! | Constant | Value |
//! | - | - |
//! | `FORK_BLOCK` | 12244000 |
//! | `ACCESS_LIST_STORAGE_KEY_COST` | 1900 |
//! | `ACCESS_LIST_ADDRESS_COST` | 2400 |
//!
//! As of `FORK_BLOCK_NUMBER`, a new [EIP-2718](./eip-2718.md) transaction is introduced with `TransactionType` `1`.
//!
//! The [EIP-2718](./eip-2718.md) `TransactionPayload` for this transaction is `rlp([chainId, nonce, gasPrice, gasLimit, to, value, data, accessList, signatureYParity, signatureR, signatureS])`.
//!
//! The `signatureYParity, signatureR, signatureS` elements of this transaction represent a secp256k1 signature over `keccak256(0x01 || rlp([chainId, nonce, gasPrice, gasLimit, to, value, data, accessList]))`.
//!
//! The [EIP-2718](./eip-2718.md) `ReceiptPayload` for this transaction is `rlp([status, cumulativeGasUsed, logsBloom, logs])`.
//!
//! For the transaction to be valid, `accessList` must be of type `[[{20 bytes}, [{32 bytes}...]]...]`, where `...` means "zero or more of the thing to the left". For example, the following is a valid access list (all hex strings would in reality be in byte representation):
//!
//! ```python
//! [
//!     [
//!         "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae",
//!         [
//!             "0x0000000000000000000000000000000000000000000000000000000000000003",
//!             "0x0000000000000000000000000000000000000000000000000000000000000007"
//!         ]
//!     ],
//!     [
//!         "0xbb9bc244d798123fde783fcc1c72d3bb8c189413",
//!         []
//!     ]
//! ]
//! ```
//!
//! At the beginning of execution (ie. at the same time as the `21000 + 4 * zeroes + 16 * nonzeroes` start gas is charged according to [EIP-2028](./eip-2028.md) rules), we charge additional gas for the access list: `ACCESS_LIST_ADDRESS_COST` gas per address and `ACCESS_LIST_STORAGE_KEY_COST` gas per storage key. For example, the above example would be charged `ACCESS_LIST_ADDRESS_COST * 2 + ACCESS_LIST_STORAGE_KEY_COST * 2` gas.
//!
//! Note that non-unique addresses and storage keys are not disallowed, though they will be charged for multiple times, and aside from the higher gas cost there is no other difference in execution flow or outcome from multiple-inclusion of a value as opposed to the recommended single-inclusion.
//!
//! The address and storage keys would be immediately loaded into the `accessed_addresses` and `accessed_storage_keys` global sets; this can be done using the following logic (which doubles as a specification-in-code of validation of the RLP-decoded access list)
//!
//! ```python
//! def process_access_list(access_list) -> Tuple[List[Set[Address], Set[Pair[Address, Bytes32]]], int]:
//!     accessed_addresses = set()
//!     accessed_storage_keys = set()
//!     gas_cost = 0
//!     assert isinstance(access_list, list)
//!     for item in access_list:
//!         assert isinstance(item, list) and len(item) == 2
//!         # Validate and add the address
//!         address = item[0]
//!         assert isinstance(address, bytes) and len(address) == 20
//!         accessed_addresses.add(address)
//!         gas_cost += ACCESS_LIST_ADDRESS_COST
//!         # Validate and add the storage keys
//!         assert isinstance(item[1], list)
//!         for key in item[1]:
//!             assert isinstance(key, bytes) and len(key) == 32
//!             accessed_storage_keys.add((address, key))
//!             gas_cost += ACCESS_LIST_STORAGE_KEY_COST
//!     return (
//!         accessed_addresses,
//!         accessed_storage_keys,
//!         gas_cost
//!     )
//! ```
//!
//! The access list is NOT charged per-byte fees like tx data is; the per-item costs described above are meant to cover the bandwidth costs of the access list data in addition to the costs of accessing those accounts and storage keys when evaluating the transaction.
//!
//! ## Rationale
//!
//! ### Charging less for accesses in the access list
//!
//! This is done to encourage transactions to use the access list as much as possible, and because processing transactions is easier when their storage reads are predictable (because clients can pre-load the data from databases and/or ask for witnesses at the time the transaction is received, or at least load the data in parallel).
//!
//! ### Allowing duplicates
//!
//! This is done because it maximizes simplicity, avoiding questions of what to prevent duplication against: just between two addresses/keys in the access list, between the access list and the tx sender/recipient/newly created contract, other restrictions? Because gas is charged per item, there is no gain and only cost in including a value in the access list twice, so this should not lead to extra chain bloat in practice.
//!
//! ### Signature signs over the transaction type as well as the transaction data
//!
//! This is done to ensure that the transaction cannot be "re-interpreted" as a transaction of a different type.
//!
//! ## Backwards Compatibility
//!
//! This EIP does make it more gas-expensive to perform "unexpected" SLOADs and account accesses. Because gas is prepaid and so does not affect fixed-gas local calls, it does not break contracts in the way that previous gas cost increases would risk. However, it does make applications that heavily rely on storage access much less economically viable.
//!
//! ## Security Considerations
//!
//! ### Access list generation
//!
//! Access lists are difficult to construct in real-time in many situations, and this is exacerbated in environments where there is a high time lag between transaction generation and signing or simplicity of the transaction generator is highly valued (eg. either or both may apply in hardware wallets).
//!
//! However, this EIP proposes only a 10% initial discount to access lists, so there is almost no cost to not bothering with access list generation and only making a simple transaction. The cost of accessing state outside the access list is expected to be ramped up in future hard forks over time as tools are developed and access list generation becomes more mature.
//!
//! ### Transaction size bloating
//!
//! Average block size will increase as a result of access lists being used. However, the per-byte cost of access lists is `1900 / 32 = 59.375` for storage keys and `2400 / 20 = 120` for addresses, making it much more expensive than calldata; hence, worst-case block size will not increase. Additionally, increases in average block size will be partially compensated for by the ability to pre-fetch storage at time of receiving a transaction and/or load storage in parallel upon receiving a block.
//!
//! Vitalik Buterin (@vbuterin), Martin Swende (@holiman), "EIP-2930: Optional access lists," Ethereum Improvement Proposals, no. 2930, August 2020. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-2930>.

use crate::eip::Eip;

/// EIP-2930: Optional access lists.
pub struct Eip2930;

impl Eip for Eip2930 {
    const NUMBER: u32 = 2930;
}
