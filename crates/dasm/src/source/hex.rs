//! Extraction of bytecode from a hex string.

use super::BytecodeSource;
use crate::bytecode::Bytecode;
use hex::FromHexError;

pub use hex;

impl BytecodeSource for String {
    type Error = FromHexError;

    fn extract(self) -> Result<Bytecode, Self::Error> {
        BytecodeSource::extract(self.as_str())
    }
}

impl BytecodeSource for &str {
    type Error = FromHexError;

    fn extract(self) -> Result<Bytecode, Self::Error> {
        let trimmed = self.trim().trim_start_matches("0x");
        Ok(Bytecode::from(hex::decode(trimmed)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_string_extraction() {
        let hex = String::from("102030");
        let bytecode = hex.extract().unwrap();
        assert_eq!(bytecode.as_ref(), &[0x10, 0x20, 0x30]);

        let hex = String::from("0x102030");
        let bytecode = hex.extract().unwrap();
        assert_eq!(bytecode.as_ref(), &[0x10, 0x20, 0x30]);

        let hex = String::from("0x10203");
        assert!(matches!(
            hex.extract().unwrap_err(),
            FromHexError::OddLength
        ));
    }
}
