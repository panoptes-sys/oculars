use clap::{Parser, Subcommand};

/// Utility for getting EVM instruction information.
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// List all EVM mnemonics.
    Mnemonics,
    /// Get information about a specific mnemonic.
    Mnemonic {
        /// Name of the mnemonic.
        name: String,
    },
}
