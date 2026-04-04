// src/command/counterparty.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct CounterpartyArgs {
    #[command(subcommand)]
    pub command: CounterpartyCommand,
}

#[derive(Subcommand, Debug)]
pub enum CounterpartyCommand {
    /// List the counterparty
    List,
}
