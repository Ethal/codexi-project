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
        #[arg(
            value_name = "ID",
            required = true,
            help = "Account id of the operation. Accept full ID, short ID"
        )]
        id: String,

        /// View the raw data
        #[arg(short, long)]
        raw: bool,
    },
}
