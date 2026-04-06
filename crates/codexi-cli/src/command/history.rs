// src/command/system.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct HistoryArgs {
    #[command(subcommand)]
    pub command: HistoryCommand,
}

#[derive(Subcommand, Debug)]
pub enum HistoryCommand {
    /// Initializes the ledger(codexi) with a starting balance.
    Init {
        /// The start date of the initialization (YYYY-MM-DD).
        #[arg(index = 1, value_name = "DATE", help = "The date of the adjustment (YYYY-MM-DD).")]
        date: String,

        /// The initial account balance.
        #[arg(index = 2, value_name = "INITIAL_BALANCE", allow_negative_numbers = true)]
        initial_amount: String,
    },

    /// Adjusts the balance to a given physical amount.
    Adjust {
        /// The start date of the initialization (YYYY-MM-DD).
        #[arg(index = 1, value_name = "DATE", help = "The date of the adjustment (YYYY-MM-DD).")]
        date: String,

        /// The actual physical balance.
        #[arg(
            index = 2,
            value_name = "PHYSICAL_BALANCE",
            allow_negative_numbers = true,
            help = "The actual physical balance to adjust the codexi to this amount."
        )]
        physical_amount: String,
    },

    ///Void an existing operation without deleting it. A compensating operation is created to preserve history.
    Void {
        #[arg(value_name = "ID", help = "Id of the operation to void")]
        id: String,
    },

    /// Closes operations up to the specified date, replacing them with a carried-over balance.
    Close {
        /// The closing date (YYYY-MM-DD). All transactions prior to this date will be archived and deleted from the codexi.
        #[arg(
            value_name = "DATE",
            required = true,
            help = "The closing date (YYYY-MM-DD). All transactions prior to this date will be archived and deleted from the codexi."
        )]
        date: String,

        /// Description of the balance carried forward (ex: 'Closing Year 2025').
        #[arg(value_name = "DESCRIPTION...", help = "Description of the closing operation")]
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
    /// List the archive file(Close operations).
    List {},

    /// View the content of an archive file.
    View {
        /// Account id
        #[arg(index = 1, value_name = "ID", required = true, help = "Account ID")]
        id: String,
        /// Close period date (Checkpoint)
        #[arg(index = 2, value_name = "DATE", required = true, help = "The close period date")]
        date: String,
    },
}
