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
    ///  List all counterparties
    List,

    /// Add a new counterparty
    Add {
        /// Counterparty name (1+ words)
        #[arg(
            value_name = "NAME",
            required = true,
            num_args = 1..,
            help = "Counterparty name (accepts multiple words)"
        )]
        name: Vec<String>,

        /// Counterparty kind
        #[arg(value_name = "KIND", required = true, help = "Kind: 'personal' or 'organization'")]
        kind: String,

        /// Optional note
        #[arg(long, value_name = "NOTE", help = "Note about the counterparty")]
        note: Option<String>,
    },

    /// [WARN] Terminate a counterparty
    Terminate {
        /// Counterparty ID or name
        #[arg(value_name = "ID", required = true, help = "Counterparty ID, short ID, or name")]
        id: String,
    },
}
