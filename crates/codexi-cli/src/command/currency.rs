// src/command/currency.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct CurrencyArgs {
    #[command(subcommand)]
    pub command: CurrencyCommand,
}

#[derive(Subcommand, Debug)]
pub enum CurrencyCommand {
    /// List the currencies
    List,
}
