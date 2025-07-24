//! Extraction of bytecode from an array of bytes.

use super::BytecodeSource;
use crate::bytecode::Bytecode;
use std::convert::Infallible;

impl BytecodeSource for Vec<u8> {
    type Error = Infallible;

    fn extract(self) -> Result<Bytecode, Self::Error> {
        Ok(Bytecode::from(self))
    }
}

impl BytecodeSource for &[u8] {
    type Error = Infallible;

    fn extract(self) -> Result<Bytecode, Self::Error> {
        Ok(Bytecode::from(self))
    }
}

impl<const N: usize> BytecodeSource for [u8; N] {
    type Error = Infallible;

    fn extract(self) -> Result<Bytecode, Self::Error> {
        Ok(Bytecode::from(self.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_vec_extraction() {
        let bytes = vec![1, 2, 3];
        let bytecode = bytes.clone().extract().unwrap();
        assert_eq!(bytecode.as_ref(), &bytes);
    }

    #[test]
    fn byte_slice_extraction() {
        let bytes = [1, 2, 3].as_slice();
        let bytecode = bytes.extract().unwrap();
        assert_eq!(bytecode.as_ref(), bytes);
    }

    #[test]
    fn byte_array_extraction() {
        let bytes = [1, 2, 3];
        let bytecode = bytes.extract().unwrap();
        assert_eq!(bytecode.as_ref(), bytes);
    }
}
