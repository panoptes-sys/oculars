//! CLI utility for getting information about EVM instructions.

/// CLI arguments.
mod args;
/// Mnemonic utilities.
mod mnemonics;

use anyhow::Context;
use clap::Parser as _;
use eva_asm::opcode::Mnemonic;
use minus::Pager;
use std::fmt::Write;

use crate::{
    args::{Cli, Command},
    mnemonics::{MnemonicDescription, get_mnemonic_by_name},
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Mnemonics => list_mnemonics()?,
        Command::Mnemonic { name } => display_mnemonic_info_by_name(&name)?,
    }

    Ok(())
}

/// List a list of all mnemonics.
fn list_mnemonics() -> anyhow::Result<()> {
    let mut pager = Pager::new();

    for mnemonic in Mnemonic::iter() {
        writeln!(pager, "{}\n", mnemonic.format_human())
            .context("failed to write mnemonic info")?;
    }

    minus::page_all(pager).context("failed to display mnemonic info")?;

    Ok(())
}

/// Display information about a mnemonic by its name.
fn display_mnemonic_info_by_name(name: &str) -> anyhow::Result<()> {
    let mnemonic = get_mnemonic_by_name(name)?;
    println!("{}", mnemonic.format_human());

    Ok(())
}
