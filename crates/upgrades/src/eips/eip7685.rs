//! EIP-7685: General purpose execution layer requests.
//!
//! ## Abstract
//!
//! This proposal defines a general purpose framework for storing contract-triggered
//! requests. It extends the execution header with a single field to store the
//! request information. Requests are later on exposed to the consensus layer, which
//! then processes each one.
//!
//! ## Motivation
//!
//! The proliferation of smart contract controlled validators has caused there to be
//! a demand for additional EL triggered behaviors. By allowing these systems to
//! delegate administrative operations to their governing smart contracts, they can
//! avoid intermediaries needing to step in and ensure certain operations occur.
//! This creates a safer system for end users. By abstracting each individual request
//! details from the EL, adding new request types is simpler and does not require an
//! update on the execution block structure.
//!
//! ## Specification
//!
//! ### Execution Layer
//!
//! #### Requests
//!
//! A `requests` object consists of a `request_type` byte prepended to an opaque byte array
//! `request_data`. The `request_data` contains zero or more encoded request objects.
//!
//! ```python
//! requests = request_type ++ request_data
//! ```
//!
//! Each request type will define its own `requests` object with its own `request_data` format.
//!
//! #### Block Header
//!
//! Extend the header with a new 32 byte commitment value `requests_hash`.
//!
//! While processing a block, multiple `requests` objects with different `request_type`s will
//! be produced by the system, and accumulated in the block requests list.
//!
//! In order to compute the commitment, an intermediate hash list is first built by hashing
//! all non-empty requests elements of the block requests list. Items with empty
//! `request_data` are excluded, i.e. the intermediate list skips `requests` items which
//! contain only the `request_type` (1 byte) and nothing else.
//!
//! Within the intermediate list, `requests` items must be ordered by `request_type` ascending.
//!
//! The final commitment is computed as the sha256 hash of the intermediate element hashes.
//!
//! ```python
//! def compute_requests_hash(block_requests: Sequence[bytes]):
//!     m = sha256()
//!     for r in block_requests:
//!         if len(r) > 1:
//!             m.update(sha256(r).digest())
//!     return m.digest()
//!
//! block.header.requests_hash = compute_requests_hash(requests)
//! ```
//!
//! ### Consensus Layer
//!
//! Each proposal may choose how to extend the beacon chain types to include new EL request
//! types.
//!
//! ## Rationale
//!
//! ### Opaque byte array rather than an RLP array
//!
//! By having the bytes of `request_data` array from second byte on be opaque bytes, rather
//! than an RLP (or other encoding) list, we can support different encoding formats for the
//! request payload in the future such as SSZ, LEB128, or a fixed width format.
//!
//! ### Request source and validity
//!
//! This EIP makes no strict requirement where a request may come from nor when/how
//! a request must be validated. This is to provide future protocol designers
//! maximum flexibility.
//!
//! The authors' recommendations on source and validity of requests are:
//!
//! * The source of requests should be from the execution of transactions. More
//!   specifically, transactions which make calls to designated system contracts
//!   that store the request in account. The storage would later be retrieved by a
//!   post-block system call to the contract. Alternatively, if the system call does
//!   not need to be inherently concerned with rate limiting, it could rely simply
//!   on emitting an event which is later parsed post-block by the system and
//!   converted into a request.
//! * A request's validity can often not be fully verified at the execution layer.
//!   This is why they are referred to merely as "requests"; they do not carry the
//!   authority on their own to unilaterally catalyze an action. We expect the system
//!   contracts to perform whatever validation is possible by the EL and then pass
//!   it on to the CL for further validation.
//!
//! ### Ordering
//!
//! The ordering across types is ascending by type. This is to simplify the process
//! of verifying that all requests which were committed to in `requests_hash` match.
//!
//! An alternative could be to order by when the request was generated within the
//! block. Since it's expected that many requests will be accumulated at the end of
//! the block via system calls, this would be difficult to enforce. Therefore,
//! ordering by type is the most straightforward ordering which ensures integrity.
//!
//! #### Intra-type
//!
//! Within the same type, order is not defined. This is because the data of the
//! request is opaque as far as this EIP is concerned. Therefore, it is to be
//! determined by each request type individually.
//!
//! ### Removing empty requests in commitment
//!
//! We exclude empty requests elements from the `requests_hash` commitment in order to get a
//! stable 'empty' hash value that is independent of the blockchain fork. For a block with no
//! requests data, the `requests_hash` is simply `sha256("")`.
//!
//! ## Backwards Compatibility
//!
//! No backward compatibility issues found.
//!
//! lightclient (@lightclient), Felix Lange (@fjl), "EIP-7685: General purpose execution layer requests," Ethereum Improvement Proposals, no. 7685, April 2024. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-7685>.

use crate::eip::Eip;

/// EIP-7685: General purpose execution layer requests.
pub struct Eip7685;

impl Eip for Eip7685 {
    const NUMBER: u32 = 7685;
}
