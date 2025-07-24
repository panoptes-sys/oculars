//! EVM disassembler.

use std::marker::PhantomData;

use asm::{instruction::Stop, opcode::Mnemonic};
use thiserror::Error;
use upgrades::execution::ExecutionUpgrade;

use crate::{assembly::Assembly, bytecode::Bytecode, source::BytecodeSource};

/// EVM disassembler.
#[derive(Default)]
pub struct Disassembler<E: ExecutionUpgrade> {
    /// Marker for storing the `ExecutionUpgrade` generic.
    _marker: PhantomData<E>,
}

/// Errors that can happen when disassembling bytecode.
#[derive(Debug, Error)]
pub enum DasmError {}

/// An error that can happen when disassembling from source.
#[derive(Debug, Error)]
pub enum SourceDasmError<E> {
    /// Failed to extract bytecode from source.
    #[error("source extraction failed: {0}")]
    Extraction(E),

    /// A failure to disassemble the bytecode.
    #[error("disassembly failed: {0}")]
    Dasm(#[from] DasmError),
}

impl<E: ExecutionUpgrade> Disassembler<E> {
    /// Disassemble EVM bytecode into an instruction list.
    ///
    /// # Errors
    /// TODO
    pub fn disassemble(&self, bytecode: &Bytecode) -> Result<Assembly, DasmError> {
        for byte in bytecode.as_ref() {
            match *byte {
                byte if byte == Mnemonic::STOP as u8 && E::supports_instruction::<Stop>() => {
                    todo!()
                }
                _ => todo!(),
            }
            //
        }

        todo!()
    }

    /// Disassembles any source that provides [`Bytecode`] into EVM assembly.
    ///
    /// # Errors
    /// Returns an error if bytecode could not be extracted from the source or if disassembly failed (see [`Disassembler::disassemble`]).
    pub fn disassemble_from_source<T: BytecodeSource>(
        &self,
        source: T,
    ) -> Result<Assembly, SourceDasmError<T::Error>> {
        let bytecode = source.extract().map_err(SourceDasmError::Extraction)?;
        Ok(self.disassemble(&bytecode)?)
    }

    /// Disassembles a hex string into EVM assembly.
    ///
    /// # Errors
    /// Returns an error if the hex string could not be parsed or if disassembly failed (see [`Disassembler::disassemble`]).
    pub fn disassemble_hex<T: AsRef<str>>(
        &self,
        hex: T,
    ) -> Result<Assembly, SourceDasmError<<&str as BytecodeSource>::Error>> {
        self.disassemble_from_source(hex.as_ref())
    }

    /// Disassembles raw bytes into EVM assembly.
    ///
    /// # Errors
    /// Returns an error if disassembly failed (see [`Disassembler::disassemble`]).
    pub fn disassemble_bytes<T: AsRef<[u8]>>(
        &self,
        bytes: T,
    ) -> Result<Assembly, SourceDasmError<<&[u8] as BytecodeSource>::Error>> {
        self.disassemble_from_source(bytes.as_ref())
    }
}
