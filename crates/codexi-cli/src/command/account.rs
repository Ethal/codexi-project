use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub command: AccountCommand,
}

/// Manage account identity and lifecycle
#[derive(Subcommand, Debug)]
pub enum AccountCommand {
    /// List all accounts (current account marked with *, closed with c)
    List,

    /// Show the current account context
    Context,

    /// Create a new account
    Create {
        /// Account opening date
        #[arg(value_name = "DATE", required = true, help = "Opening date (YYYY-MM-DD)")]
        date: String,

        /// Account name
        #[arg(
            value_name = "NAME",
            required = true,
            num_args = 1..,
            help = "Account name"
        )]
        name: Vec<String>,

        /// Account type
        #[arg(
            short = 't',
            long = "type",
            value_name = "ACCOUNT_TYPE",
            default_value = "Current",
            help = "Account type (Current, Joint, Saving, Deposit, Loan, Business, Student)"
        )]
        account_type: Option<String>,
    },

    /// Switch active account
    Use {
        /// Account ID, short ID, or name
        #[arg(value_name = "ID", required = true, help = "Account ID, short ID, or name")]
        id: String,
    },

    /// Close an account (irreversible operation)
    Close {
        /// Account ID, short ID, or name
        #[arg(
            index = 1,
            value_name = "ID",
            required = true,
            help = "Account ID, short ID, or name"
        )]
        id: String,

        /// Closing date
        #[arg(index = 2, value_name = "DATE", required = true, help = "Closing date (YYYY-MM-DD)")]
        date: String,
    },

    /// Rename an account
    Rename {
        /// Account ID, short ID, or name
        #[arg(
            index = 1,
            value_name = "ID",
            required = true,
            help = "Account ID, short ID, or name"
        )]
        id: String,

        /// New account name
        #[arg(index = 2, value_name = "NAME", required = true, help = "New account name")]
        name: Vec<String>,
    },

    /// Set bank for the current account
    SetBank {
        /// Bank ID, short ID, or name
        #[arg(value_name = "ID", required = true, help = "Bank ID, short ID, or name")]
        id: String,
    },

    /// Set currency for the current account
    SetCurrency {
        /// Currency ID, short ID, or code
        #[arg(value_name = "ID", required = true, help = "Currency ID, short ID, or code")]
        id: String,

        /// Update all existing operations with this currency
        #[arg(short = 'u', long, help = "Update existing operations")]
        update_operation: bool,
    },

    /// Configure account context (limits, rules, constraints)
    SetContext {
        /// Overdraft limit
        #[arg(
            short = 'o',
            long,
            value_name = "AMOUNT",
            allow_negative_numbers = false,
            help = "Overdraft limit (e.g. 500.00)"
        )]
        overdraft: Option<String>,

        /// Minimum balance
        #[arg(
            short = 'b',
            long,
            value_name = "AMOUNT",
            allow_negative_numbers = false,
            help = "Minimum balance (e.g. 100.00)"
        )]
        balance_min: Option<String>,

        /// Maximum monthly transactions
        #[arg(
            short = 'm',
            long,
            value_name = "N",
            allow_negative_numbers = false,
            help = "Max monthly transactions"
        )]
        max_monthly_transactions: Option<String>,

        /// Lock deposits until a date
        #[arg(short = 'd', long, value_name = "DATE", help = "Lock deposits until (YYYY-MM-DD)")]
        deposit_locked_until: Option<String>,

        /// Allow interest
        #[arg(short = 'i', long, help = "Allow interest")]
        interest: Option<bool>,

        /// Allow joint signers
        #[arg(short = 's', long, help = "Allow joint signers")]
        signers: Option<bool>,
    },
}
