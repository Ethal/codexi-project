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

    /// Add a counterparty
    Add {
        /// Counterparty name
        #[arg(value_name = "NAME", required = true, num_args = 1.., help = "Counterparty name")]
        name: Vec<String>,

        /// Counterparty kind
        #[arg(value_name = "KIND", required = true, help = "Kind: 'personal' or 'organization'")]
        kind: String,

        /// Counterparty note
        #[arg(long, value_name = "NOTE", num_args = 1.., help = "note of the counterparty")]
        note: Option<Vec<String>>,
    },
    /// Teminate a counterparty
    Terminate {
        /// Id of the counterparty
        #[arg(value_name = "ID", required = true, help = "id\name of the counterparty")]
        id: String,
    },
}
