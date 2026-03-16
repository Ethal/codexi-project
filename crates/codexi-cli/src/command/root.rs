// scr/command/root.rs

use clap::{Parser, Subcommand};

use crate::command::account::AccountArgs;
use crate::command::admin::AdminArgs;
use crate::command::bank::BankArgs;
use crate::command::category::CategoryArgs;
use crate::command::currency::CurrencyArgs;
use crate::command::data::DataArgs;
use crate::command::history::HistoryArgs;
use crate::command::report::ReportArgs;

#[derive(Parser, Debug)]
#[command(
    name = "codexi-cli",
    version,
    about = "Personal financial ledger & analytics",
    long_about = r#"
Account is an append-only financial ledger designed for clarity,
traceability and long-term analysis.

Typical workflow:
  - record operations (debit / credit)
  - search and filter data
  - generate reports
  - manage snapshots, archives, Backups
"#,
    arg_required_else_help = true
)]
pub struct Cli {
    #[arg(short, long, global = true, help = "Skip confirmation prompts")]
    pub yes: bool,
    #[command(subcommand)]
    pub command: RootCommand,
}

#[derive(Subcommand, Debug)]
pub enum RootCommand {
    /// Add a regular debit operation
    Debit {
        #[arg(
            index = 1,
            value_name = "DATE",
            required = true,
            help = "Date of the debit operation (YYYY-MM-DD)"
        )]
        date: String,

        #[arg(
            index = 2,
            value_name = "AMOUNT",
            required = true,
            allow_negative_numbers = true,
            help = "Amount of the debit operation"
        )]
        amount: String,

        #[arg(
            index = 3,
            value_name = "DESCRIPTION...",
            help = "Description of the debit operation",
            default_value = "no description"
        )]
        description: Vec<String>,
    },

    /// Add a regular credit operation
    Credit {
        #[arg(
            index = 1,
            value_name = "DATE",
            required = true,
            help = "Date of the credit operation (YYYY-MM-DD)"
        )]
        date: String,

        #[arg(
            index = 2,
            value_name = "AMOUNT",
            required = true,
            allow_negative_numbers = true,
            help = "Amount of the credit operation"
        )]
        amount: String,

        #[arg(
            index = 3,
            value_name = "DESCRIPTION...",
            help = "Description of the credit operation",
            default_value = "no description"
        )]
        description: Vec<String>,
    },

    /// Search and filter in current account operations.
    #[command(alias = "view")]
    Search {
        /// Arbitrary date range
        #[arg(
            long,
            help = "Start date for filtering operations",
            value_name = "FROM_DATE"
        )]
        from: Option<String>,

        /// Arbitrary date range
        #[arg(
            long,
            help = "End date for filtering operations",
            value_name = "TO_DATE"
        )]
        to: Option<String>,

        /// Filter by text contained in description
        #[arg(
            short = 't',
            long,
            help = "Filter by text in description",
            value_name = "TEXT"
        )]
        text: Option<String>,

        /// Filter by type of kind operation (Init, Adjust, Close, Transaction, ...)
        #[arg(
            short = 'k',
            long,
            help = "Filter by kind: 'init', 'adjust', 'void', 'close', 'transaction', 'fee', 'transfer', 'refund'",
            value_name = "KIND"
        )]
        kind: Option<String>,

        /// Filter by the flow of operation (debit, credit)
        #[arg(
            short = 'f',
            long,
            help = "Filter by flow: 'debit' or 'credit'",
            value_name = "FLOW"
        )]
        flow: Option<String>,

        /// Minimum amount
        #[arg(
            long = "a-min",
            help = "Minimum amount",
            value_name = "AMOUNT",
            allow_negative_numbers = true
        )]
        amount_min: Option<String>,

        /// Maximum amount
        #[arg(
            long = "a-max",
            help = "Maximum amount",
            value_name = "AMOUNT",
            allow_negative_numbers = true
        )]
        amount_max: Option<String>,

        /// The latest operations to display.
        #[arg(
            long,
            help = "The latest N operations to display",
            value_name = "NUMBER",
            allow_negative_numbers = false
        )]
        last: Option<usize>,

        /// The operation of today
        #[arg(long, help = "The operation  of today")]
        today: bool,
    },

    /// Manage codexi accounts
    Account(AccountArgs),

    /// Manage codexi banks
    Bank(BankArgs),

    /// Manage codexi currencies
    Currency(CurrencyArgs),

    /// Manage codexi categories
    Category(CategoryArgs),

    /// Manage accounting timeline (init, checkpoint, adjust, void, archives)
    History(HistoryArgs),

    /// Generate financial reports, statistics, and statements
    Report(ReportArgs),

    /// Manage data mobility (Import/Export) and local safety snapshots
    Data(DataArgs),

    /// Technical maintenance, disaster recovery, and low-level file management
    /// To be use carefully, performed a --help is recommended
    Admin(AdminArgs),
}
