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
    /// View operation details
    View {
        /// Operation ID (full or short)
        #[arg(
            value_name = "ID",
            required = true,
            help = "Operation ID. Accepts full ID or short ID"
        )]
        id: String,

        /// View raw data
        #[arg(short, long, help = "Display raw operation data")]
        raw: bool,
    },

    /// Update operation metadata (description, counterparty, category, or exchange rate)
    Update {
        /// Operation ID (full or short)
        #[arg(
            value_name = "ID",
            required = true,
            help = "Operation ID. Accepts full ID or short ID"
        )]
        id: String,

        /// Description
        #[arg(
            short = 'd',
            long,
            value_name = "DESCRIPTION",
            help = "New description for the operation"
        )]
        description: Option<String>,

        /// Counterparty
        #[arg(
            short = 'c',
            long,
            value_name = "COUNTERPARTY",
            help = "Counterparty (ID, short ID, or name)"
        )]
        counterparty: Option<String>,

        /// Category
        #[arg(
            short = 'g',
            long,
            value_name = "CATEGORY",
            help = "Category (ID, short ID, or name)"
        )]
        category: Option<String>,

        /// Exchange rate
        #[arg(
            short = 'r',
            long,
            value_name = "FROM TO",
            num_args = 2,
            help = "Exchange rate: units of <TO> per unit of <FROM> (e.g., --rate EUR USD)"
        )]
        rate: Option<Vec<String>>,
    },
}
