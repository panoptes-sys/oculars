//! Instruction assembly information.

use crate::OpCode;

/// An EVM assembly instruction.
pub trait AssemblyInstruction {
    /// Returns the size of this instruction in bytes.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{instruction::{Stop, Push}, AssemblyInstruction};
    /// assert_eq!(Stop.size(), 1);
    /// assert_eq!(Push::<4>::new([0; 4]).size(), 5);
    /// ```
    #[must_use]
    #[inline]
    fn size(&self) -> u8 {
        self.immediate_size() + 1
    }

    /// Returns the size of this instruction's immediate value.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{instruction::{Stop, Push}, AssemblyInstruction};
    /// assert_eq!(Stop.immediate_size(), 0);
    /// assert_eq!(Push::<0>::new([]).immediate_size(), 0);
    /// assert_eq!(Push::<32>::new([0; 32]).immediate_size(), 32);
    /// ```
    #[must_use]
    #[inline]
    fn immediate_size(&self) -> u8 {
        0
    }

    /// Returns the instruction's [`OpCode`].
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{OpCode, Mnemonic, instruction::{Stop, Unknown}, AssemblyInstruction};
    /// assert_eq!(Stop.opcode(), OpCode::Known(Mnemonic::STOP));
    /// assert_eq!(Unknown::new(0xF).opcode(), OpCode::Unknown(0xF));
    /// ```
    #[must_use]
    fn opcode(&self) -> OpCode;

    /// Returns [`true`] if this instruction is of the type `PUSHx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{AssemblyInstruction, instruction::{Push, Gas}};
    /// assert_eq!(Push::new([0; 10]).is_push(), true);
    /// assert_eq!(Gas.is_push(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_push(&self) -> bool {
        self.opcode().is_push()
    }

    /// Returns [`true`] if this instruction is of the type `DUPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{AssemblyInstruction, instruction::{Dup, Gas}};
    /// assert_eq!(Dup::<10>::new().is_dup(), true);
    /// assert_eq!(Gas.is_dup(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_dup(&self) -> bool {
        self.opcode().is_dup()
    }

    /// Returns [`true`] if this instruction is of the type `SWAPx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{AssemblyInstruction, instruction::{Swap, Gas}};
    /// assert_eq!(Swap::<10>::new().is_swap(), true);
    /// assert_eq!(Gas.is_swap(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_swap(&self) -> bool {
        self.opcode().is_swap()
    }

    /// Returns [`true`] if this instruction is of the type `LOGx`.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{AssemblyInstruction, instruction::{Log, Gas}};
    /// assert_eq!(Log::<3>::new().is_log(), true);
    /// assert_eq!(Gas.is_log(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_log(&self) -> bool {
        self.opcode().is_log()
    }

    /// Returns [`true`] for instructions that terminate execution of the smart contract.
    ///
    /// # Example
    /// ```
    /// # use oculars_asm2::{AssemblyInstruction, instruction::{Return, Unknown, Gas}};
    /// assert_eq!(Return.is_terminator(), true);
    /// assert_eq!(Unknown::new(0xF).is_terminator(), true);
    /// assert_eq!(Gas.is_terminator(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_terminator(&self) -> bool {
        self.opcode().is_terminator()
    }

    /// Returns [`true`] if this instruction is a `JUMP`, `JUMPI` or a `JUMPDEST`.
    /// # Example
    /// ```
    /// # use oculars_asm2::{AssemblyInstruction, instruction::{Jump, JumpDest, Gas}};
    /// assert_eq!(Jump.is_control_flow(), true);
    /// assert_eq!(JumpDest.is_control_flow(), true);
    /// assert_eq!(Gas.is_control_flow(), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_control_flow(&self) -> bool {
        self.opcode().is_control_flow()
    }
}
