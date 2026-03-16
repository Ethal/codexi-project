// src/command/bank.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct BankArgs {
    #[command(subcommand)]
    pub command: BankCommand,
}

#[derive(Subcommand, Debug)]
pub enum BankCommand {
    /// List the banks
    List,
}
