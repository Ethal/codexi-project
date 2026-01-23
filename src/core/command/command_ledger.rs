// scr/core/command/command_ledger.rs
use clap::{Parser, Subcommand };

use crate::core::command::command_data::DataArgs;
use crate::core::command::command_system::SystemArgs;
use crate::core::command::command_report::ReportArgs;
use crate::core::command::command_maintenance::MaintenanceArgs;


#[derive(Parser, Debug)]
#[command(
    name = "codexi",
    version,
    about = "Personal financial ledger & analytics",
    long_about = r#"
Codexi is an append-only financial ledger designed for clarity,
traceability and long-term analysis.

Typical workflow:
  - record operations (debit / credit)
  - search and filter data
  - generate reports
  - manage snapshots and archives
"#,
    arg_required_else_help = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: LedgerCommand,
}

#[derive(Subcommand, Debug)]
pub enum LedgerCommand {

    /// Add a regular debit operation
    Debit {
        #[arg(index = 1, value_name = "DATE", required = true, help = "Date of the debit operation (YYYY-MM-DD)")]
        date: String,

        #[arg(index = 2, value_name = "AMOUNT", required = true, allow_negative_numbers = true, help = "Amount of the debit operation")]
        amount: String,

        #[arg(index = 3, value_name = "DESCRIPTION...", help = "Description of the debit operation", default_value = "no description")]
        description: Vec<String>,
    },

    /// Add a regular credit operation
    Credit {
        #[arg(index = 1, value_name = "DATE", required = true, help = "Date of the credit operation (YYYY-MM-DD)")]
        date: String,

        #[arg(index = 2, value_name = "AMOUNT", required = true, allow_negative_numbers = true, help = "Amount of the credit operation")]
        amount: String,

        #[arg(index = 3, value_name = "DESCRIPTION...", help = "Description of the credit operation", default_value = "no description")]
        description: Vec<String>,
    },

    /// Search in operation.
    Search {
        // Filtres granulaire (Plage de dates arbitraire)
        #[arg(long, help = "Start date for filtering operations", value_name = "FROM_DATE")]
        from: Option<String>,

        #[arg(long, help = "End date for filtering operations", value_name = "TO_DATE")]
        to: Option<String>,

        /// Filter by text contained in description
        #[arg(short = 't', long, help = "Filter by text in description", value_name = "TEXT")]
        text: Option<String>,

        /// Filter by type of kind operation (Init, Adjust, Close, Transaction, ...)
        #[arg(short = 'k', long, help = "Filter by kind: 'init', 'adjust', 'void', 'close', 'transaction', 'fee', 'transfer', 'refund'", value_name = "KIND")]
        kind: Option<String>,

        /// Filter by the flow of operation (debit, credit)
        #[arg(short = 'f', long, help = "Filter by flow: 'debit' or 'credit'", value_name = "FLOW")]
        flow: Option<String>,

        /// Minimum amount
        #[arg(long = "a-min", help = "Minimum amount", value_name = "AMOUNT", allow_negative_numbers = true)]
        amount_min: Option<String>,

        /// Maximum amount
        #[arg(long = "a-max", help = "Maximum amount", value_name = "AMOUNT", allow_negative_numbers = true)]
        amount_max: Option<String>,

        /// The latest operations to display.
        #[arg(long, help = "The latest N operations to display", value_name = "NUMBER", allow_negative_numbers = false)]
        latest: Option<usize>,
    },

    /// Report.
    Report(ReportArgs),

    /// Export/Import/Snapshot
    Data(DataArgs),

    /// Manages accounting anchors (Initial Balance, Adjustment, Closing), Void, Archives and Backup/Restore
    System(SystemArgs),

    /// Manages maintenance of the file (clear, clear all, migration, file infos)
    Maintenance(MaintenanceArgs),

}
