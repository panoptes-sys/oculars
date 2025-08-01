//! EIP-7702: Set Code for EOAs.
//!
//! ## Abstract
//!
//! Add a new [EIP-2718](./eip-2718.md) transaction type that allows Externally
//! Owned Accounts (EOAs) to set the code in their account. This is done by
//! attaching a list of authorization tuples -- individually formatted as `[chain_id,
//! address, nonce, y_parity, r, s]` -- to the transaction. For each tuple, a
//! delegation indicator `(0xef0100 || address)` is written to the authorizing
//! account's code. All code executing operations must load and execute the code
//! pointed to by the delegation.
//!
//! ## Motivation
//!
//! Despite great advances in the smart contract wallet ecosystem, EOAs have held
//! back broad adoption of UX improvements across applications. This EIP therefore
//! focuses on adding short-term functionality improvements to EOAs which will allow
//! UX improvements to permeate through the entire application stack. Three
//! particular features this EIP is designed around are:
//!
//! * **Batching**: allowing multiple operations from the same user in one atomic transaction. One common example is an [ERC-20](./eip-20.md) approval followed by spending that approval. This is a common workflow in DEXes that requires two transactions today. Advanced use cases of batching occasionally involve dependencies: the output of the first operation is part of the input to the second operation.
//! * **Sponsorship**: account X pays for a transaction on behalf of account Y. Account X could be paid in some other ERC-20 for this service, or it could be an application operator including the transactions of its users for free.
//! * **Privilege de-escalation**: users can sign sub-keys and give them specific permissions that are much weaker than global access to the account. For example, a permission to spend ERC-20 tokens but not ETH, or to spend up to 1% of the total balance per day, or to interact only with a specific application.
//!
//! ## Specification
//!
//! ### Parameters
//!
//! |     Parameter            | Value   |
//! | ------------------------ | ------- |
//! | `SET_CODE_TX_TYPE`       | `0x04`  |
//! | `MAGIC`                  | `0x05`  |
//! | `PER_AUTH_BASE_COST`     | `12500` |
//! | `PER_EMPTY_ACCOUNT_COST` | `25000` |
//!
//! ### Set code transaction
//!
//! A new [EIP-2718](./eip-2718.md) transaction known as the "set code transaction"
//! is introduced, where the `TransactionType` is `SET_CODE_TX_TYPE` and the
//! `TransactionPayload` is the RLP serialization of the following:
//!
//! ```python
//! rlp([chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas, gas_limit,
//! destination, value, data, access_list, authorization_list, signature_y_parity,
//! signature_r, signature_s])
//!
//! authorization_list = [[chain_id, address, nonce, y_parity, r, s], ...]
//! ```
//!
//! The fields `chain_id`, `nonce`, `max_priority_fee_per_gas`, `max_fee_per_gas`,
//! `gas_limit`, `destination`, `value`, `data`, and `access_list` of the outer
//! transaction follow the same semantics as [EIP-4844](./eip-4844.md). *Note, this
//! implies a null destination is not valid.*
//!
//! The `signature_y_parity, signature_r, signature_s` elements of this transaction
//! represent a secp256k1 signature over `keccak256(SET_CODE_TX_TYPE ||
//! TransactionPayload)`.
//!
//! The `authorization_list` is a list of tuples that indicate what code the signer
//! of each tuple desires to execute in the context of their EOA. The transaction is
//! considered invalid if the length of `authorization_list` is zero.
//!
//! The transaction is also considered invalid when any field in an authorization
//! tuple cannot fit within the following bounds:
//!
//! ```python
//! assert auth.chain_id < 2**256
//! assert auth.nonce < 2**64
//! assert len(auth.address) == 20
//! assert auth.y_parity < 2**8
//! assert auth.r < 2**256
//! assert auth.s < 2**256
//! ```
//!
//! The [EIP-2718](./eip-2718.md) `ReceiptPayload` for this transaction is
//! `rlp([status, cumulative_transaction_gas_used, logs_bloom, logs])`.
//!
//! #### Behavior
//!
//! The authorization list is processed before the execution portion of the
//! transaction begins, but after the sender's nonce is incremented.
//!
//! For each `[chain_id, address, nonce, y_parity, r, s]` tuple, perform the
//! following:
//!
//! 1. Verify the chain ID is 0 or the ID of the current chain.
//! 2. Verify the `nonce` is less than `2**64 - 1`.
//! 3. Let `authority = ecrecover(msg, y_parity, r, s)`.
//!     * Where `msg = keccak(MAGIC || rlp([chain_id, address, nonce]))`.
//!     * Verify `s` is less than or equal to `secp256k1n/2`, as specified in EIP-2.
//! 4. Add `authority` to `accessed_addresses`, as defined in [EIP-2929](./eip-2929.md).
//! 5. Verify the code of `authority` is empty or already delegated.
//! 6. Verify the nonce of `authority` is equal to `nonce`.
//! 7. Add `PER_EMPTY_ACCOUNT_COST - PER_AUTH_BASE_COST` gas to the global refund
//!    counter if `authority` is not empty.
//! 8. Set the code of `authority` to be `0xef0100 || address`. This is a delegation
//!    indicator.
//!     * If `address` is `0x0000000000000000000000000000000000000000`, do not write the delegation indicator. Clear the account's code by resetting the account's code hash to the empty code hash `0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470`.
//! 9. Increase the nonce of `authority` by one.
//!
//! If any step above fails, immediately stop processing the tuple and continue to
//! the next tuple in the list. When multiple tuples from the same authority are
//! present, set the code using the address in the last valid occurrence.
//!
//! Note, if transaction execution results in failure (e.g. any exceptional
//! condition or code reverting), the processed delegation indicators is *not rolled
//! back*.
//!
//! ##### Delegation indicator
//!
//! Delegation indicators use the banned opcode `0xef`, defined in
//! [EIP-3541](./eip-3541.md), to indicate that the code must be handled differently
//! than regular code. The delegation forces all code executing operations to follow
//! the address pointer to obtain the code to execute. For example, `CALL` loads the
//! code at `address` and executes it in the context of `authority`.
//!
//! The affected executing operations are:
//!
//! * `CALL`
//! * `CALLCODE`
//! * `DELEGATECALL`
//! * `STATICCALL`
//! * any transaction where `destination` points to an address with a delegation indicator present
//!
//! For code reading, only `CODESIZE` and `CODECOPY` instructions are affected. They
//! operate directly on the executing code instead of the delegation. For example,
//! when executing a delegated account `EXTCODESIZE` returns `23` (the size of
//! `0xef0100 || address`) whereas `CODESIZE` returns the size of the code residing
//! at `address`.
//!
//! *Note, this means during delegated execution `CODESIZE` and `CODECOPY` produce a
//! different result compared to calling `EXTCODESIZE` and `EXTCODECOPY` on the
//! authority.*
//!
//! ###### Precompiles
//!
//! When a precompile address is the target of a delegation, the retrieved code is
//! considered empty and `CALL`, `CALLCODE`, `STATICCALL`, `DELEGATECALL`
//! instructions targeting this account will execute empty code, and therefore
//! succeed with no execution when given enough gas to initiate the call.
//!
//! ###### Loops
//!
//! In case a delegation indicator points to another delegation, creating a
//! potential chain or loop of delegations, clients must retrieve only the first
//! code and then stop following the delegation chain.
//!
//! #### Gas Costs
//!
//! The intrinsic cost of the new transaction is inherited from
//! [EIP-2930](./eip-2930.md), specifically `21000 + 16 * non-zero calldata bytes +
//! 4 * zero calldata bytes + 1900 * access list storage key count + 2400 * access
//! list address count`. Additionally, add a cost of `PER_EMPTY_ACCOUNT_COST *
//! authorization list length`.
//!
//! The transaction sender will pay for all authorization tuples, regardless of
//! validity or duplication.
//!
//! If a code executing instruction accesses a cold account during the resolution of
//! delegated code, add an additional [EIP-2929](eip-2929.md)
//! `COLD_ACCOUNT_READ_COST` cost of `2600` gas to the normal cost and add the
//! account to `accessed_addresses`. Otherwise, assess a `WARM_STORAGE_READ_COST`
//! cost of `100`.
//!
//! #### Transaction origination
//!
//! Modify the restriction put in place by [EIP-3607](./eip-3607.md) to allow EOAs
//! whose code is a valid delegation indicator, i.e. `0xef0100 || address`, to
//! originate transactions. Accounts with any other code values may not originate
//! transactions.
//!
//! Additionally, if a transaction's `destination` has a delegation indicator, add
//! the target of the delegation to `accessed_addresses`.
//!
//! ## Rationale
//!
//! Below is the rationale for both general design directions of the EIP, as well as
//! specific technical choices.
//!
//! ### General design philosophy
//!
//! #### Persistence of code delegation
//!
//! The first draft of this proposal had a clever idea to avoid disagreement on
//! whether in-protocol revocation was needed or not. The idea was to temporarily
//! set code in the account with the authorization. After the transaction finished,
//! the code would be completely cleared. This was a new design space for enriching
//! EOA functionality.
//!
//! Even this approach was not without its flaws. Fundamentally, there was not much
//! friction for users including set code authorizations. This meant that some users
//! and applications would opt to treat the extension as more of a scripting
//! facility, rather than a full-fledged upgrade to a smart contract wallet. The
//! outcome of this would be two somewhat competing workstreams for UX improvements:
//! smart contract wallets and EOA scripts.
//!
//! Previous proposals had been met with similar criticisms. To counteract this,
//! persistent delegations were introduced. They create enough friction in
//! deployment that users will not deploy new, unique ones regularly. This will
//! hopefully unify the workstreams and minimize fragmentation in UX developments.
//!
//! #### No initcode
//!
//! Running initcode is not desirable for many reasons. It creates a new mode of
//! execution that needs extensive testing, and may be used for purposes not
//! possible with standard smart contract wallets. It also forces developers to
//! perform initialization as a standard call to the EOA after delegation. The lack
//! of atomicity in these operations is another factor that will push users to
//! complete smart contract wallet solutions, instead of EOA scripts.
//!
//! Additionally, initcode tends to be propagated inside the transaction calldata.
//! This means it would need to be included in the authorization tuple and signed
//! over. The minimum initcode is around 15 bytes -- it would simply copy the
//! contract code from an external address. The total cost would be `16 * 15 = 240`
//! calldata cost, plus the [EIP-3860](./eip-3860.md) cost of `2 * 15 = 30`, plus
//! the runtime costs of around `150`. So nearly `500` additional gas would be spent
//! preparing the account. Even more likely, 1200+ gas if not copying from an
//! external account.
//!
//! #### Creation by template
//!
//! Initcode or not, there is a question of how users should specify the code they
//! intend to run in their account. The two main options are to specify the bytecode
//! directly in the transaction or to specify a pointer to the code. The simplest
//! pointer would just be the address of code deployed on-chain.
//!
//! The cost analysis makes the answer clear. The smallest proxy would be around 50
//! bytes and an address is 20 bytes. The 30 byte difference provides no useful
//! additional functionality and will be inefficiently replicated billions of times.
//!
//! Furthermore, specifying code directly would again make it possible for EOAs to
//! have a new, unique ability to execute arbitrary code specified in the
//! transaction calldata. It is for these reasons that creation by template is
//! chosen.
//!
//! #### Interaction with applications and wallets
//!
//! While this EIP provides a lot of flexibility to applications and EOAs, there are
//! incorrect ways of using it. Applications **must not** expect that they can
//! suggest the user sign an authorization, and therefore it is the duty of the
//! wallet to not provide an interface to do so.
//!
//! **There is no safe way to provide this interface**. The code specified by an
//! authorization has unrestricted access to the account and must always be closely
//! audited by the wallet. Few users have the level of sophistication to reasonably
//! verify the code they are delegating to.
//!
//! It is also not possible to implement a system of permissions at this level to
//! minimize the risk. If applications require custom wallet functionality, they
//! must use standardized extension / module systems built on top of the delegated
//! code that correctly implements permissions.
//!
//! #### Forward-compatibility with future account abstraction
//!
//! This EIP is designed to be forward-compatible with endgame account abstraction,
//! without over-enshrining any fine-grained details of [ERC-4337](./eip-4337.md) or
//! RIP-7560.
//!
//! To start, the `address` that users sign could directly point to existing
//! ERC-4337 wallet code. This essentially requires the "code pathways" that are
//! used are code pathways that would, in most cases, continue to make sense in a
//! pure-smart-contract-wallet world. Hence, it avoids the problem of creating two
//! separate UX workstreams because, to a large extent, they would be the same
//! ecosystem.
//!
//! There will be some workflows that require kludges under this solution that would
//! be better done in some different "more native" under "endgame AA", but this is
//! relatively a small subset. The EIP does not require adding any opcodes, that
//! would become dangling and useless in a post-EOA world, and it allows EOAs to
//! masquerade as contracts to be included in ERC-4337 bundles, in a way that's
//! compatible with the existing `EntryPoint`.
//!
//! #### Self-sponsoring: allowing `tx.origin` to set code
//!
//! Allowing `tx.origin` to set code and execute its own delegated code enables what
//! is called self-sponsoring. It allows users to take advantage of EIP-7702 without
//! relying on any third party infrastructure.
//!
//! However, that means the EIP breaks the invariant that `msg.sender == tx.origin`
//! only happens in the topmost execution frame of a transaction. This will affect
//! smart contracts containing `require(msg.sender == tx.origin)` style checks. This
//! check is used for at least three purposes:
//!
//! 1. Ensuring that `msg.sender` is an EOA (given that `tx.origin` always has to be
//!    an EOA). This invariant does not depend on the execution layer depth and,
//!    therefore, is not affected.
//! 2. Protecting against atomic sandwich attacks like flash loans, which rely on
//!    the ability to modify state before and after the execution of the target
//!    contract as part of the same atomic transaction. This protection would be broken
//!    by this EIP. However, relying on `tx.origin` in this way is considered bad
//!    practice, and can already be circumvented by miners conditionally including
//!    transactions in a block.
//! 3. Preventing reentrancy.
//!
//! Examples of (1) and (2) can be found in contracts deployed on Ethereum mainnet,
//! with (1) being more common (and unaffected by this proposal). On the other hand,
//! use case (3) is more severely affected by this proposal, but the authors of this
//! EIP did not find any examples of this form of reentrancy protection, though the
//! search was non-exhaustive.
//!
//! This distribution of occurrences—many (1), some (2), and no (3)—is exactly what
//! the authors of this EIP expect because:
//!
//! * Determining if `msg.sender` is an EOA without `tx.origin` is difficult, if not impossible.
//! * The only execution context which is safe from atomic sandwich attacks is the topmost context, and `tx.origin == msg.sender` is the only way to detect that context.
//! * In contrast, there are many direct and flexible ways of preventing reentrancy (e.g., using a transient storage variable). Since `msg.sender == tx.origin` is only true in the topmost context, it would make an obscure tool for preventing reentrancy, rather than other more common approaches.
//!
//! There are other approaches to mitigate this restriction which do not break the
//! invariant:
//!
//! * Set `tx.origin` to a constant `ENTRY_POINT` address when using the `CALL*` instruction in the context of an EOA.
//! * Set `tx.origin` to a special address derived from the sender or signer addresses.
//! * Disallow `tx.origin` from setting code. This would make the simple batching use cases impossible, but could be relaxed in the future.
//!
//! ### Rationale for technical details
//!
//! #### Cost of delegation
//!
//! The `PER_AUTH_BASE_COST` is the cost to process the authorization tuple and set
//! the delegation destination. To compute a fair cost for this operation, the
//! authors review its impact on the system:
//!
//! * ferry 101 bytes of calldata = `101 * non-zero cost (16) = 1616`
//! * recovering the `authority` address = `3000`
//! * reading the nonce and code of `authority` = `2600`
//! * storing values in already warm account = `200`
//! * cost to deploy code = `200 * 23 = 4600`
//!
//! The impact-based assessment identifies `12016` gas of comparable computation for
//! the operation. It is rounded up to `12500` to account for miscellaneous costs
//! associated with shuttling data around the state transition.
//!
//! #### Clearing delegation indicators
//!
//! A general design goal in state transition changes is to minimize the number of
//! special cases an EIP has. In early iterations, this EIP resisted a special case
//! for clearing an account's delegation indicator.
//!
//! For most intents and purposes, an account delegated to `0x0` is
//! indistinguishable from a true EOA. However, one particular unfortunate case is
//! unavoidable. Even if a user has a zeroed out delegation indicator, most
//! operations that interact with that account will incur an additional
//! `COLD_ACCOUNT_READ_COST` upon the first touch caused by attempting to load the
//! code at `0x0`.
//!
//! For this reason, the authors have opted to include a special case which allow
//! users to restore their EOA to its original purity.
//!
//! #### Lack of instruction prohibition
//!
//! Consistency is a valuable property in the EVM, both from an implementation
//! perspective and a user-understanding-perspective. Despite considering bans on
//! several families of instructions in the context of EOAs, the authors feel there
//! is not a compelling reason to do so, as it would cause smart contract wallets
//! and EOA smart contract wallets to proceed down distinct UX workstreams.
//!
//! The main instruction families where a ban was considered were storage related
//! and contract creation related. The decision to not ban storage instructions
//! hinged mostly on their importance to smart contract wallets. Although it's
//! possible to have an external storage contract that the smart contract wallet
//! calls into, it is unnecessarily complicated and inefficient. In the future, new
//! state schemes may allow substantially cheaper access to certain storage slots
//! within an account. This is something smart contract wallets will want to take
//! advantage of that a storage contract wouldn't support.
//!
//! Creation instructions were considered for a ban as well on other similar EIPs,
//! however because this EIP allows EOAs to spend value intra-transaction, the
//! concern with bumping the nonce intra-transaction and invalidating pending
//! transactions is not significant.
//!
//! #### Protection from malleability cross-chain
//!
//! One consideration when signing a code pointer is what code that address points
//! to on another chain. While it is possible to create a deterministic deployment,
//! i.e. via Nick's method, verifying such a deployment may not always be desirable.
//! In such situations, the chain ID can be set to reduce the scope of the
//! authorization. When universal deployment is preferred, simply set chain ID to 0.
//!
//! An alternative to adding chain ID could be to substitute in the actual code for
//! the address in the signature. This seems to have the benefit of both minimizing
//! the on-chain size of auth tuples, by continuing to serialize only the address,
//! while retaining specificity of the actual code running in the account, by
//! pulling in the code for the signature. One unfortunate issue of this format,
//! though, is that it imposes a database lookup to determine the signer of each
//! auth tuple. This imposition itself seems to create enough complexity in
//! transaction propagation that it is decided to avoid and simply sign over the
//! address directly.
//!
//! #### Delegation of code execution only
//!
//! Other code retrieving operations like `EXTCODEHASH` do not automatically follow
//! delegations, they operate on the delegation indicator itself. If instead
//! delegations were followed, an account would be able to temporarily masquerade as
//! having a particular codehash, which would break contracts that rely on
//! codehashes as a definition of possible account behavior. A change of behavior in
//! a contract is currently only possible if its code explicitly allows it (in
//! particular via `DELEGATECALL`), and a change of codehash is only possible in the
//! presence of `SELFDESTRUCT` (which, as of Cancun, only applies in the same
//! transaction as contract creation), so choosing to follow delegations in
//! `EXTCODE*` opcodes would have created a new type of account breaking prior
//! assumptions.
//!
//! #### Charge maximum cost upfront
//!
//! While computing the intrinsic gas cost, the transaction is charged the
//! worst-case cost for each delegation. Later, while processing the authorization
//! list, a refund is issued if the account already exists in state. This mechanism
//! is designed to avoid state lookups for each authorization when computing the
//! intrinsic gas and can quickly determine the validity of the transaction with
//! only a state lookup on the sender's account.
//!
//! #### No blobs, no contract creation
//!
//! Transactions should be thought of as specialized tools and not necessarily a
//! one-type-does-all solution. EIP-4844 is treated differently at the p2p level due
//! to burden blobs place on a node's bandwidth. EIP-7702 has different implications
//! on transaction gossiping and there is no need to complicate those rules
//! unnecessarily by making it a superset of all possible functionality. The authors
//! ultimately do not expect there to be much demand for atomic delegation and blob
//! submission.
//!
//! Contract creation is another specialized use case that has been grandfathered
//! into several transaction types. It adds complexity to testing, because it is a
//! new distinct branch of execution that needs to be tested when any change to the
//! EVM occurs and verify the change works as expected in that context.
//!
//! For these reasons, the authors have chosen to keep the scope of the EIP focused
//! on improving UX.
//!
//! #### Disallow delegation to precompiles
//!
//! Precompiles are themselves edge cases, so allowing delegations to precompiles or
//! not requires some focus in implementation. Considering the fact that precompiles
//! technically do not have code associated with their accounts, the authors decided
//! it would be marginally simpler to not execute the precompile logic when a user
//! delegates to one. This is somewhat unintuitive.
//!
//! #### Non-empty authorization list required
//!
//! Set code transactions are required to have at least one authorization to be
//! considered valid. This is to disincentivize senders from using type 4
//! transactions as a generic transaction format, because this transaction has
//! different implications on the transaction pool than, say,
//! [EIP-1559](./eip-1559.md) transactions.
//!
//! ## Backwards Compatibility
//!
//! This EIP breaks a few invariants:
//!
//! * An account balance can only decrease as a result of a transaction originating from that account.
//!   * Once an account has been delegated, any call to the account may also cause the balance to decrease.
//! * An EOA nonce may not increase after transaction execution has begun.
//!   * Once an account has been delegated, the account may call a create operation during execution, causing the nonce to increase.
//! * `tx.origin == msg.sender` can only be true in the topmost frame of execution.
//!   * Once an account has been delegated, it can invoke multiple calls per transaction.
//!
//! ## Security Considerations
//!
//! ### Implementation of secure delegate contracts
//!
//! The following is a non-exhaustive list of pitfalls that delegate contracts
//! *should* be wary of and require a signature over from the account's authority:
//!
//! * Replay protection (e.g., a nonce) should be implemented by the delegate and signed over. Without it, a malicious actor can reuse a signature, repeating its effects.
//! * `value` -- without it, a malicious sponsor could cause unexpected effects in the callee.
//! * `gas` -- without it, a malicious sponsor could cause the callee to run out of gas and fail, griefing the sponsee.
//! * `target` / `calldata` -- without them, a malicious actor may call arbitrary functions in arbitrary contracts.
//!
//! A poorly implemented delegate can *allow a malicious actor to take near complete
//! control over a signer's EOA*.
//!
//! ### Front running initialization
//!
//! Smart contract wallet developers must consider the implications of setting code
//! in an account without execution. Contracts are normally deployed by executing
//! initcode to determine the exact code to be placed in the account. This gives
//! developers the opportunity to initialize storage slots at the same time. The
//! initial values of the account cannot be replaced by an observer, because they
//! are either signed over by an EOA in the case of a creation transaction or they
//! are committed to by computing the contract's address deterministically from the
//! hash of the initcode.
//!
//! This EIP does not provide developers the opportunity to run initcode and set
//! storage slots during delegation. To secure the account from an observer
//! front-running the initialization of the delegation with an account they control,
//! smart contract wallet developers must verify the initial calldata to the account
//! for setup purposes be signed by the EOA's key using ecrecover. This ensures the
//! account can only be initialized with desirable values.
//!
//! ### Storage management
//!
//! Changing an account's delegation is a security-critical operation that should
//! not be done lightly, especially if the newly delegated code is not purposely
//! designed and tested as an upgrade to the old one.
//!
//! In particular, in order to ensure a safe migration of an account from one
//! delegate contract to another, it's important for these contracts to use storage
//! in a way that avoids accidental collisions among them. For example, using
//! [ERC-7201](./eip-7201.md) a contract may root its storage layout at a slot
//! dependent on a unique identifier. To simplify this, smart contract languages may
//! provide a way of re-rooting the entire storage layout of existing contract
//! source code.
//!
//! If all contracts previously delegated to by the account used the approach
//! described above, a migration should not cause any issues. However, if there is
//! any doubt, it is recommended to first clear all account storage, an operation
//! that is not natively offered by the protocol but that a special-purpose delegate
//! contract can be designed to implement.
//!
//! ### Setting code as `tx.origin`
//!
//! Allowing the sender of an EIP-7702 to also set code has the possibility to:
//!
//! * Break atomic sandwich protections which rely on `tx.origin`;
//! * Break reentrancy guards of the style `require(tx.origin == msg.sender)`.
//!
//! The authors of this EIP believe the risks of allowing this are acceptable for
//! the reasons outlined in the Rationale section.
//!
//! ### Sponsored transaction relayers
//!
//! It is possible for the `authorized` account to cause sponsored transaction
//! relayers to spend gas without being reimbursed by either invalidating the
//! authorization (i.e., increasing the account's nonce) or by sweeping the relevant
//! assets out of the account. Relayers should be designed with these cases in mind,
//! possibly by requiring a bond to be deposited or by implementing a reputation
//! system.
//!
//! ### Transaction propagation
//!
//! Allowing EOAs to behave as smart contracts via the delegation indicator poses
//! some challenges for transaction propagation. Traditionally, EOAs have only been
//! able to send value via a transaction. This invariant allows nodes to statically
//! determine the validity of transactions for that account. In other words, a
//! single transaction has only been able to invalidate transactions pending from
//! the sender's account.
//!
//! With this EIP, it becomes possible to cause transactions from other accounts to
//! become stale. This is due to the fact that once an EOA has delegated to code,
//! that code can be called by anyone at any point in a transaction. It becomes
//! impossible to know if the balance of the account has been swept in a static
//! manner.
//!
//! While there are a few mitigations for this, the authors recommend that clients
//! do not accept more than one pending transaction for any EOA with a non-zero
//! delegation indicator. This minimizes the number of transactions that can be
//! invalidated by a single transaction.
//!
//! An alternative would be to expand the EIP-7702 transaction with a list of
//! accounts the caller wishes to "hydrate" during the transaction. Those accounts
//! behave as the delegated code *only* for EIP-7702 transactions which include them
//! in such a list, thus returning to clients the ability to statically analyze and
//! reason about pending transactions.
//!
//! A related issue is that an EOA's nonce may be incremented more than once per
//! transaction. Because clients already need to be robust in a worse scenario
//! (described above), it isn't a major concern. However, clients should be aware
//! this behavior is possible and design their transaction propagation accordingly.
//!
//! Vitalik Buterin (@vbuterin), Sam Wilson (@`SamWilsn`), Ansgar Dietrichs (@adietrichs), lightclient (@lightclient), "EIP-7702: Set Code for EOAs," Ethereum Improvement Proposals, no. 7702, May 2024. [Online serial]. Available: <https://eips.ethereum.org/EIPS/eip-7702>.

use crate::eip::Eip;

/// EIP-7702: Set Code for EOAs.
pub struct Eip7702;

impl Eip for Eip7702 {
    const NUMBER: u32 = 7702;
}
