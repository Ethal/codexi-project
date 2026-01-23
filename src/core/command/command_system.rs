// src/core/command/command_system.rs

use clap::{Args, Subcommand};

/// structure System
#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct SystemArgs {
    #[command(subcommand)]
    pub command: SystemCommand,
}

#[derive(Subcommand, Debug)]
pub enum SystemCommand {

    /// Initializes the ledger with a starting balance.
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
        #[arg(index = 2, value_name = "PHYSICAL_BALANCE", allow_negative_numbers = true, help = "The actual physical balance to adjust the codexi to this amount.")]
        physical_amount: String,

    },

    ///Void an existing operation without deleting it. A compensating operation is created to preserve history.
    Void {
        #[arg(value_name = "INDEX", help = "Index of the operation to void", allow_negative_numbers = false)]
        index: usize
    },

    /// Closes operations up to the specified date, replacing them with a carried-over balance.
    Close {
        /// The closing date (YYYY-MM-DD). All transactions prior to this date will be archived and deleted from the codexi.
        #[arg(value_name = "DATE", required = true, help = "The closing date (YYYY-MM-DD). All transactions prior to this date will be archived and deleted from the codexi.")]
        date: String,

        /// Description of the balance carried forward (ex: 'Closing Year 2025').
        #[arg(value_name = "DESCRIPTION...", help = "Description of the closing operation")]
        description: Vec<String>,
    },

    /// List the archive file(Close operations).
    List {},

    /// View the content of an archive file.
    View {
        /// Load an archieve file (view only)
        #[arg(value_name = "FILENAME", help = "The archive filename to view")]
        filename: String,
    },

    /// Backup(Zip file) the ledger(codexi.dat) include the archive file.
    Backup {
        #[arg(long, value_name = "DIR or PATH", help = "Target directory or full path for the backup ZIP file. If a directory is provided, a default filename with timestamp will be used.")]
        target_dir: Option<String>,
    },

    /// Restore a ledger(codexi.dat) include the archive file from a backup file(Zip File).
    Restore {
        #[arg(value_name = "FILENAME", help = "The backup ZIP filename to restore from")]
        filename: String,
    },

}
