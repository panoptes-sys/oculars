//! Sources from which bytecode can be extracted.

pub mod bytes;
pub mod file;
pub mod hex;

use crate::bytecode::Bytecode;

/// A source of bytecode out of which the actual bytecode can be extracted.
pub trait BytecodeSource {
    /// An error that can happen during extraction.
    type Error;

    /// Extracts bytecode from this source.
    ///
    /// # Errors
    /// Returns an error if extraction fails (source-specific, see [`BytecodeSource::Error`]).
    fn extract(self) -> Result<Bytecode, Self::Error>;
}
