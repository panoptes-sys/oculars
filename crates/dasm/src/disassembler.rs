//! EVM disassembler.

use std::marker::PhantomData;

use upgrades::execution::ExecutionUpgrade;

/// EVM disassembler.
pub struct Disassembler<F: ExecutionUpgrade> {
    /// Marker for storing the `ExecutionUpgrade` generic.
    _marker: PhantomData<F>,
}
