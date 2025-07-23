//! Extraction of bytecode from a file.

use super::BytecodeSource;
use crate::bytecode::Bytecode;
use std::{
    fs::File,
    io::{self, Read},
};

impl BytecodeSource for File {
    type Error = io::Error;

    fn extract(mut self) -> Result<Bytecode, Self::Error> {
        let mut bytes = vec![];
        self.read_to_end(&mut bytes)?;

        match String::try_from(bytes) {
            Ok(hex_string) => {
                if let Ok(bytecode) = BytecodeSource::extract(hex_string.as_str()) {
                    Ok(bytecode)
                } else {
                    Ok(Bytecode::from(hex_string.into_bytes()))
                }
            }
            Err(err) => Ok(Bytecode::from(err.into_bytes())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek as _, SeekFrom, Write};
    use tempfile::tempfile;

    #[test]
    fn byte_file_extraction() {
        let mut file = tempfile().unwrap();
        file.write_all(&[10, 20, 30]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let bytecode = file.extract().unwrap();
        assert_eq!(bytecode.as_ref(), &[10, 20, 30]);
    }

    #[test]
    fn utf8_file_extraction() {
        let mut file = tempfile().unwrap();
        write!(file, "0x102030").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let bytecode = file.extract().unwrap();
        assert_eq!(bytecode.as_ref(), &[0x10, 0x20, 0x30]);
    }

    #[test]
    fn invalid_utf8_file_extraction() {
        let mut file = tempfile().unwrap();
        file.write_all(&[0xC0]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let bytecode = file.extract().unwrap();
        assert_eq!(bytecode.as_ref(), &[0xC0]);
    }
}
