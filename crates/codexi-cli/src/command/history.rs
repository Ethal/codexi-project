// src/command/history.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct HistoryArgs {
    #[command(subcommand)]
    pub command: HistoryCommand,
}

#[derive(Subcommand, Debug)]
pub enum HistoryCommand {
    /// Initialize the ledger with a starting balance
    Init {
        /// Date (YYYY-MM-DD)
        #[arg(index = 1, value_name = "DATE", help = "Initialization date (YYYY-MM-DD)")]
        date: String,

        /// Starting balance
        #[arg(
            index = 2,
            value_name = "INITIAL_BALANCE",
            help = "Starting balance (can be negative)"
        )]
        initial_amount: String,
    },

    /// Adjust the balance to match a physical amount
    Adjust {
        /// Date (YYYY-MM-DD)
        #[arg(index = 1, value_name = "DATE", help = "Adjustment date (YYYY-MM-DD)")]
        date: String,

        /// Physical balance
        #[arg(index = 2, value_name = "PHYSICAL_BALANCE", help = "Physical balance to adjust to")]
        physical_amount: String,
    },

    /// Void an existing operation (creates a compensating operation to preserve history)
    Void {
        /// Operation ID (full or short)
        #[arg(value_name = "ID", help = "Operation ID. Accepts full ID or short ID")]
        id: String,
    },

    /// Close operations up to a date and carry over the balance
    Close {
        /// Closing date (YYYY-MM-DD)
        #[arg(
            value_name = "DATE",
            required = true,
            help = "Closing date (YYYY-MM-DD). All operations prior to this date will be archived"
        )]
        date: String,

        /// Description
        #[arg(
            value_name = "DESCRIPTION",
            num_args = 0..,
            help = "Description of the closing operation"
        )]
        description: Vec<String>,
    },

    /// Browse and view closed period archives
    Archive(ArchiveArgs),
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct ArchiveArgs {
    #[command(subcommand)]
    pub command: ArchiveCommand,
}

#[derive(Subcommand, Debug)]
pub enum ArchiveCommand {
    /// 📂 List closed period archives
    List,

    /// 📂 View archive file content
    View {
        /// Account ID
        #[arg(
            index = 1,
            value_name = "ID",
            required = true,
            help = "Account ID. Accepts full ID, short ID, or name"
        )]
        id: String,

        /// Closing date
        #[arg(index = 2, value_name = "DATE", required = true, help = "Closing date (YYYY-MM-DD)")]
        date: String,
    },
}
