// src/command/operation.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct OperationArgs {
    #[command(subcommand)]
    pub command: OperationCommand,
}

/// Manage operations
#[derive(Subcommand, Debug)]
pub enum OperationCommand {
    /// View an operation
    View {
        /// Operation id
        #[arg(value_name = "ID", required = true, help = "Operation id, accept full ID, short ID")]
        id: String,

        /// View the raw data
        #[arg(short, long)]
        raw: bool,
    },
    /// Update description, counterparty or category
    Update {
        /// Operation id
        #[arg(value_name = "ID", required = true, help = "Operation id, accept full ID, short ID")]
        id: String,
        /// Description
        #[arg(short = 'd', long, value_name = "DESCRIPTION", help = "Description of the operation")]
        description: Option<String>,
        /// Counterparty
        #[arg(
            short = 'c',
            long,
            value_name = "COUNTERPARTY",
            help = "Counterparty of the operation, accept full ID, short ID or name"
        )]
        counterparty: Option<String>,
        /// Category
        #[arg(
            short = 'g',
            long,
            value_name = "CATEGORY",
            help = "Category of the operation, accept full ID, short ID or name"
        )]
        category: Option<String>,
        // Exchange rate
        #[arg(
            short = 'r',
            long,
            value_name = "FROM TO",
            num_args = 2,
            help = "Exchange rate: --rate <from> <to>"
        )]
        rate: Option<Vec<String>>,
    },
}
