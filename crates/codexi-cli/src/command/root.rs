// scr/command/root.rs

use clap::{Parser, Subcommand};

use crate::command::account::AccountArgs;
use crate::command::admin::AdminArgs;
use crate::command::bank::BankArgs;
use crate::command::category::CategoryArgs;
use crate::command::countparty::CounterpartyArgs;
use crate::command::currency::CurrencyArgs;
use crate::command::data::DataArgs;
use crate::command::history::HistoryArgs;
use crate::command::operation::OperationArgs;
use crate::command::report::ReportArgs;

#[derive(Parser, Debug)]
#[command(
    name = "codexi-cli",
    version,
    about = "Codexi - Personal financial ledger & analytics",
    long_about = r#"
Codexi is an append-only financial ledger designed for clarity,
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
    /// Show an overview of the accounts (id,name,type,currency,debit,credit,balance)
    Overview {},

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
            allow_negative_numbers = false,
            help = "Amount of the debit operation, shall be positive"
        )]
        amount: String,

        #[arg(
            index = 3,
            value_name = "DESCRIPTION",
            required = false,
            num_args = 0..,
            help = "Description of the debit operation"
        )]
        description: Vec<String>,

        #[arg(
            short = 'c',
            long = "counterparty",
            value_name = "COUNTERPARTY",
            required = false,
            help = "Counterparty of the debit"
        )]
        counterparty: Option<String>,

        #[arg(
            short = 'g',
            long = "category",
            value_name = "CATEGORY",
            required = false,
            help = "category of the debit"
        )]
        category: Option<String>,
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
            allow_negative_numbers = false,
            help = "Amount of the credit operation, shall be positive"
        )]
        amount: String,

        #[arg(
            index = 3,
            value_name = "DESCRIPTION",
            required = false,
            num_args = 0..,
            help = "Description of the credit operation"
        )]
        description: Vec<String>,

        #[arg(
            short = 'c',
            long = "counterparty",
            value_name = "COUNTERPARTY",
            required = false,
            help = "Counterparty of the credit"
        )]
        counterparty: Option<String>,

        #[arg(
            short = 'g',
            long = "category",
            value_name = "CATEGORY",
            required = false,
            help = "category of the credit"
        )]
        category: Option<String>,
    },
    /// Add a regular interest operation
    Interest {
        #[arg(
            index = 1,
            value_name = "DATE",
            required = true,
            help = "Date of the interest operation (YYYY-MM-DD)"
        )]
        date: String,

        #[arg(
            index = 2,
            value_name = "AMOUNT",
            required = true,
            allow_negative_numbers = false,
            help = "Amount of the interest operation, shall be positive"
        )]
        amount: String,

        #[arg(
            index = 3,
            value_name = "DESCRIPTION",
            required = false,
            num_args = 0..,
            help = "Description of the interest operation"
        )]
        description: Vec<String>,

        #[arg(
            short = 'c',
            long = "counterparty",
            value_name = "COUNTERPARTY",
            required = false,
            help = "Counterparty of the interest"
        )]
        counterparty: Option<String>,

        #[arg(
            short = 'g',
            long = "category",
            value_name = "CATEGORY",
            required = false,
            help = "category of the interest"
        )]
        category: Option<String>,
    },
    /// Add a transfer operation between current account and other account in the ledger.
    /// <ACCOUNT_ID_TO> accept full ID, short ID, name of the account
    Transfer {
        /// date of the transfer
        #[arg(
            index = 1,
            value_name = "DATE",
            required = true,
            help = "Date of the operation (YYYY-MM-DD)"
        )]
        date: String,
        /// Amount from the current account to be sent
        #[arg(
            index = 2,
            value_name = "AMOUNT_FROM",
            required = true,
            allow_negative_numbers = false,
            help = "Amount, in currency, from the current account to be sent, shall be positive"
        )]
        amount_from: String,
        /// Amount of the destination account to be received
        #[arg(
            index = 3,
            value_name = "AMOUNT_TO",
            required = true,
            allow_negative_numbers = false,
            help = "Amount, in currency, of the destination account to be received, shall be positive"
        )]
        amount_to: String,
        /// Account id of the destination
        #[arg(
            index = 4,
            value_name = "ACCOUNT_ID_TO",
            required = true,
            allow_negative_numbers = false,
            help = "Account id of the destination. Accept full ID, short ID or name of the account"
        )]
        account_id_to: String,
        /// Description of the operation
        #[arg(
            index = 5,
            value_name = "DESCRIPTION...",
            num_args = 1..,
            help = "Description of the operation",
            default_value = "no description"
        )]
        description: Vec<String>,

        #[arg(
            short = 'g',
            long = "category",
            value_name = "CATEGORY",
            required = false,
            help = "category of the transfer"
        )]
        category: Option<String>,
    },

    /// Search and filter on operations in the current account, alias: view.
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
            help = "Filter by kind: 'init', 'adjust', 'void', 'close/checkpoint', 'transaction', 'fee', 'transfer', 'interest', 'refund'",
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
            help = "The latest N operations to show",
            value_name = "NUMBER",
            allow_negative_numbers = false
        )]
        last: Option<usize>,

        /// The operation of today
        #[arg(long, help = "The operation(s) of 'today'")]
        today: bool,

        /// open in the defaut browser
        #[arg(long, help = "Open result in the default browser")]
        open: bool,
    },

    /// Manage Operations
    Operation(OperationArgs),

    /// Manage Accounts
    Account(AccountArgs),

    /// Manage Banks
    Bank(BankArgs),

    /// Manage Currencies
    Currency(CurrencyArgs),

    /// Manage Counterparties
    Counterparty(CounterpartyArgs),

    /// Manage Categories
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
