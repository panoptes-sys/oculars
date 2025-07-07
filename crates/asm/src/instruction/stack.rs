//! Stack Operations.

/// Remove item from stack.
pub struct Pop;

// TODO: figure out how to represent these instructions the best way.
/// Place item on stack.
pub struct Push<const N: u8>;
/// Duplicate stack items.
pub struct Dup;
/// Exchange stack items.
pub struct Swap;
