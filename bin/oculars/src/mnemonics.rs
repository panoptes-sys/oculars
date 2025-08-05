use std::fmt::{Binary, Debug, Display, LowerHex, Octal};

use anyhow::Context as _;
use asm::Mnemonic;
use colored::Colorize as _;

/// A human readable description for a mnemonic.
pub trait MnemonicDescription {
    /// Return description of this mnemonic.
    fn description(&self) -> &'static str;

    /// Format mnemonic in a human way.
    fn format_human(&self) -> String
    where
        Self: LowerHex + Display + Octal + Binary + Debug,
    {
        format!(
            "* {name} - {}\n  - opcode: 0x{self:x}, 0b{self:b}, 0o{self:o}",
            self.description(),
            name = format!("{self}").yellow().bold(),
        )
    }
}

/// Try to get a mnemonic by its name.
pub fn get_mnemonic_by_name(name: &str) -> anyhow::Result<Mnemonic> {
    let uppercase_name = name.to_uppercase();

    Mnemonic::VARIANTS
        .iter()
        .find(|mnemonic| mnemonic.to_string() == uppercase_name)
        .copied()
        .context(format!("failed to find mnemonic by the name \"{name}\""))
}

#[expect(clippy::too_many_lines, reason = "can't split this up")]
impl MnemonicDescription for Mnemonic {
    fn description(&self) -> &'static str {
        match self {
            Mnemonic::STOP => "Halts execution.",
            Mnemonic::ADD => {
                "Adds the top two elements of the stack together, and pushes the result back on the stack."
            }
            Mnemonic::MUL => {
                "Multiply the top two elements of the stack, and pushes the result back on the stack."
            }
            Mnemonic::SUB => {
                "Subtracts the top two elements of the stack, and pushes the result back on the stack."
            }
            Mnemonic::DIV => {
                "Integer division of the top two elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::SDIV => {
                "Signed integer division of the top two elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::MOD => {
                "Modulo remainder of the top two elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::SMOD => {
                "Signed modulo remainder of the top two elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::ADDMOD => {
                "Modulo addition of the top 2 elements with the 3rd element. Pushes the result back on the stack."
            }
            Mnemonic::MULMOD => {
                "Modulo multiplication of the top 2 elements with the 3rd element. Pushes the result back on the stack."
            }
            Mnemonic::EXP => {
                "Exponential operation of the top 2 elements. Pushes the result back on the stack."
            }
            Mnemonic::SIGNEXTEND => {
                "Sign extend operation. In other words, extend a signed number which fits in N bytes to 32 bytes."
            }
            Mnemonic::LT => {
                "Checks if the top element is less than the next top element. Pushes the result back on the stack."
            }
            Mnemonic::GT => {
                "Checks if the top element is greater than the next top element. Pushes the result back on the stack."
            }
            Mnemonic::SLT => "Signed less-than comparison.",
            Mnemonic::SGT => "Signed greater-than comparison.",
            Mnemonic::EQ => {
                "Checks if the top element is equal to the next top element. Pushes the result back on the stack."
            }
            Mnemonic::ISZERO => {
                "Checks if the top element is equal to 0. Pushes the result back on the stack."
            }
            Mnemonic::AND => {
                "Bitwise AND operation of the top 2 elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::OR => {
                "Bitwise OR operation of the top 2 elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::XOR => {
                "Bitwise XOR operation of the top 2 elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::NOT => {
                "Bitwise NOT operation of the top element of the stack. Pushes the result back on the stack."
            }
            Mnemonic::BYTE => {
                "For a word (defined by next top element of the stack), retrieve the Nth byte (0-indexed and defined by top element of stack) from the left (most significant) to right (least significant)."
            }
            Mnemonic::SHL => {
                "Logical shift left operation of the top 2 elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::SHR => {
                "Logical shift right  operation of the top 2 elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::SAR => {
                "Arithmetic shift right operation of the top 2 elements of the stack. Pushes the result back on the stack."
            }
            Mnemonic::KECCAK256 => "Pushes to the stack the Keccak-256 hash of a region of memory.",
            Mnemonic::ADDRESS => {
                "Pushes the address of the current executing account to the stack."
            }
            Mnemonic::BALANCE => "Pushes the balance of the given account onto the stack.",
            Mnemonic::ORIGIN => {
                "Pushes the address of the original transaction sender to the stack. The origin address can only be an EOA."
            }
            Mnemonic::CALLER => "Pushes the address of the caller onto the stack.",
            Mnemonic::CALLVALUE => "Push the value (in wei) sent with the call onto the stack.",
            Mnemonic::CALLDATALOAD => {
                "Push a word (32 bytes) of the input data belonging to the current environment onto the stack."
            }
            Mnemonic::CALLDATASIZE => {
                "Push the size of input data in current environment onto the stack."
            }
            Mnemonic::CALLDATACOPY => {
                "Copy a portion of the input data in current environment to memory."
            }
            Mnemonic::CODESIZE => {
                "Push the size of code running in current environment onto the stack."
            }
            Mnemonic::CODECOPY => "Copy a portion of the code in current environment to memory.",
            Mnemonic::GASPRICE => "Push the gas price used in current environment onto the stack.",
            Mnemonic::EXTCODESIZE => "Push the code size of a given account onto the stack.",
            Mnemonic::EXTCODECOPY => "Copy a portion of an account's code to memory.",
            Mnemonic::RETURNDATASIZE => "Pushes the size of the return data buffer onto the stack.",
            Mnemonic::RETURNDATACOPY => "Copies data from the return data buffer code to memory.",
            Mnemonic::EXTCODEHASH => "Returns the Keccak-256 hash of a contract’s bytecode.",
            Mnemonic::BLOCKHASH => {
                "Push the hash of one of the 256 most recent complete blocks onto the stack. The block number to hash is present at the top of the stack."
            }
            Mnemonic::COINBASE => {
                "Push the current block's beneficiary address (address of the block miner) onto the stack."
            }
            Mnemonic::TIMESTAMP => {
                "Push the current block's timestamp onto the stack. Here the timestamp being referred is actually the unix timestamp in seconds."
            }
            Mnemonic::NUMBER => "Push the current block's number onto the stack.",
            Mnemonic::PREVRANDAO => {
                "Push the prev_randao value onto the stack. The prev_randao value is the random output of the beacon chain's randomness oracle for the previous block."
            }
            Mnemonic::GASLIMIT => "Push the current block's gas limit onto the stack.",
            Mnemonic::CHAINID => "Push the chain id onto the stack.",
            Mnemonic::SELFBALANCE => "Pushes the balance of the current address to the stack.",
            Mnemonic::BASEFEE => "Pushes the base fee of the current block on to the stack.",
            Mnemonic::BLOBHASH => {
                "Pushes the versioned hash at a particular index on to the stack."
            }
            Mnemonic::BLOBBASEFEE => "Pushes the blob base fee on to the stack.",
            Mnemonic::POP => "Remove item from stack.",
            Mnemonic::MLOAD => "Loads a word from memory.",
            Mnemonic::MSTORE => "Stores a word to memory.",
            Mnemonic::MSTORE8 => "Stores a byte to memory.",
            Mnemonic::SLOAD => "Loads a word from storage of the current account.",
            Mnemonic::SSTORE => "Stores a word to storage of the current account.",
            Mnemonic::JUMP => {
                "Alters the program counter to the location specified by the top of the stack."
            }
            Mnemonic::JUMPI => {
                "Alters the program counter to the specified location if and only if a condition is true. If the condition is not true, then the program counter would increase only by one."
            }
            Mnemonic::PC => {
                "Pushes onto the stack the value of the program counter after reaching the current instruction and without increasing it for the next instruction."
            }
            Mnemonic::MSIZE => "Pushes the size of active memory in bytes onto the stack.",
            Mnemonic::GAS => {
                "Pushes the amount of available gas (including the corresponding reduction for the cost of this instruction) onto the stack."
            }
            Mnemonic::JUMPDEST => "Marks a valid destination for jumps.",
            Mnemonic::TLOAD => "Loads a word from transient storage of the current acount",
            Mnemonic::TSTORE => "Stores a word to transient storage of the current acount",
            Mnemonic::MCOPY => "Copies the bytes in memory from one location to another.",
            Mnemonic::PUSH0 => "Pushes a zero onto the stack.",
            Mnemonic::PUSH1 => "Pushes a 1-byte immediate onto the stack.",
            Mnemonic::PUSH2 => "Pushes a 2-byte immediate onto the stack.",
            Mnemonic::PUSH3 => "Pushes a 3-byte immediate onto the stack.",
            Mnemonic::PUSH4 => "Pushes a 4-byte immediate onto the stack.",
            Mnemonic::PUSH5 => "Pushes a 5-byte immediate onto the stack.",
            Mnemonic::PUSH6 => "Pushes a 6-byte immediate onto the stack.",
            Mnemonic::PUSH7 => "Pushes a 7-byte immediate onto the stack.",
            Mnemonic::PUSH8 => "Pushes a 8-byte immediate onto the stack.",
            Mnemonic::PUSH9 => "Pushes a 9-byte immediate onto the stack.",
            Mnemonic::PUSH10 => "Pushes a 10-byte immediate onto the stack.",
            Mnemonic::PUSH11 => "Pushes a 11-byte immediate onto the stack.",
            Mnemonic::PUSH12 => "Pushes a 12-byte immediate onto the stack.",
            Mnemonic::PUSH13 => "Pushes a 13-byte immediate onto the stack.",
            Mnemonic::PUSH14 => "Pushes a 14-byte immediate onto the stack.",
            Mnemonic::PUSH15 => "Pushes a 15-byte immediate onto the stack.",
            Mnemonic::PUSH16 => "Pushes a 16-byte immediate onto the stack.",
            Mnemonic::PUSH17 => "Pushes a 17-byte immediate onto the stack.",
            Mnemonic::PUSH18 => "Pushes a 18-byte immediate onto the stack.",
            Mnemonic::PUSH19 => "Pushes a 19-byte immediate onto the stack.",
            Mnemonic::PUSH20 => "Pushes a 20-byte immediate onto the stack.",
            Mnemonic::PUSH21 => "Pushes a 21-byte immediate onto the stack.",
            Mnemonic::PUSH22 => "Pushes a 22-byte immediate onto the stack.",
            Mnemonic::PUSH23 => "Pushes a 23-byte immediate onto the stack.",
            Mnemonic::PUSH24 => "Pushes a 24-byte immediate onto the stack.",
            Mnemonic::PUSH25 => "Pushes a 25-byte immediate onto the stack.",
            Mnemonic::PUSH26 => "Pushes a 26-byte immediate onto the stack.",
            Mnemonic::PUSH27 => "Pushes a 27-byte immediate onto the stack.",
            Mnemonic::PUSH28 => "Pushes a 28-byte immediate onto the stack.",
            Mnemonic::PUSH29 => "Pushes a 29-byte immediate onto the stack.",
            Mnemonic::PUSH30 => "Pushes a 30-byte immediate onto the stack.",
            Mnemonic::PUSH31 => "Pushes a 31-byte immediate onto the stack.",
            Mnemonic::PUSH32 => "Pushes a 32-byte immediate onto the stack.",
            Mnemonic::DUP1 => "Duplicate the 1st stack item",
            Mnemonic::DUP2 => {
                "Duplicate the 2nd stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP3 => {
                "Duplicate the 3rd stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP4 => {
                "Duplicate the 4th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP5 => {
                "Duplicate the 5th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP6 => {
                "Duplicate the 6th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP7 => {
                "Duplicate the 7th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP8 => {
                "Duplicate the 8th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP9 => {
                "Duplicate the 9th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP10 => {
                "Duplicate the 10th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP11 => {
                "Duplicate the 11th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP12 => {
                "Duplicate the 12th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP13 => {
                "Duplicate the 13th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP14 => {
                "Duplicate the 14th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP15 => {
                "Duplicate the 15th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::DUP16 => {
                "Duplicate the 16th stack item (from top of the stack) to the top of stack."
            }
            Mnemonic::SWAP1 => {
                "Swap the top and the 1st element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP2 => {
                "Swap the top and the 2nd element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP3 => {
                "Swap the top and the 3rd element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP4 => {
                "Swap the top and the 4th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP5 => {
                "Swap the top and the 5th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP6 => {
                "Swap the top and the 6th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP7 => {
                "Swap the top and the 7th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP8 => {
                "Swap the top and the 8th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP9 => {
                "Swap the top and the 9th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP10 => {
                "Swap the top and the 10th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP11 => {
                "Swap the top and the 11th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP12 => {
                "Swap the top and the 12th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP13 => {
                "Swap the top and the 13th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP14 => {
                "Swap the top and the 14th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP15 => {
                "Swap the top and the 15th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::SWAP16 => {
                "Swap the top and the 16th element of the stack, where the top of the stack is position zero."
            }
            Mnemonic::LOG0 => "Append log record with no topics.",
            Mnemonic::LOG1 => "Append log record with one topic.",
            Mnemonic::LOG2 => "Append log record with two topics.",
            Mnemonic::LOG3 => "Append log record with three topics.",
            Mnemonic::LOG4 => "Append log record with four topics.",
            Mnemonic::CREATE => "Creates a new account with associated code.",
            Mnemonic::CALL => "Message-call into an account.",
            Mnemonic::CALLCODE => "Message-call into this account with alternative account’s code.",
            Mnemonic::RETURN => "Halts execution returning output data.",
            Mnemonic::DELEGATECALL => {
                "Message-call into this account with an alternative account’s code, but persisting the current values for sender and value."
            }
            Mnemonic::CREATE2 => {
                "Creates a new account with associated code. It's similar to CREATE opcode except that the address of new account depends on the init_code instead of the nonce of sender."
            }
            Mnemonic::STATICCALL => "Static message-call into an account.",
            Mnemonic::REVERT => {
                "Halt execution reverting state changes but returning data and remaining gas."
            }
            Mnemonic::INVALID => "Designated invalid instruction.",
            Mnemonic::SELFDESTRUCT => "Halt execution and register account for later deletion.",
            _ => "Unknown operation",
        }
    }
}
