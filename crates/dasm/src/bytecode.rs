//! EVM bytecode.

/// EVM bytecode.
#[derive(Debug)]
pub struct Bytecode(Vec<u8>);

impl From<Vec<u8>> for Bytecode {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<&[u8]> for Bytecode {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

impl AsRef<[u8]> for Bytecode {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
