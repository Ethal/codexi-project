// src/command/account.rs

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
    /// List all accounts
    /// prefix * Current account
    /// prefix c Close account
    List,

    /// Context of the current account
    Context,

    /// Create a new account
    Create {
        /// Account openning date
        #[arg(value_name = "DATE", required = true, help = "Account opening date")]
        date: String,
        /// Account name
        #[arg(value_name = "NAME", required = true, num_args = 1.., help = "Account name")]
        name: Vec<String>,
        /// Account Type (Current, Joint, Saving, Deposit, Businness, Student)
        #[arg(
            value_name = "ACCOUNT_TYPE",
            short = 't',
            long = "type",
            default_value = "Current",
            help = "Account type (Current, Joint, Saving, Deposit, Business, Student)"
        )]
        account_type: Option<String>,
    },

    /// Switch active account
    Use {
        /// Account id
        #[arg(value_name = "ID", required = true, help = "Account ID")]
        id: String,
    },

    /// !!! Close an account (no future action will be available)
    Close {
        /// Account id
        #[arg(index = 1, value_name = "ID", required = true, help = "Account ID")]
        id: String,
        /// Closing date
        #[arg(
            index = 2,
            value_name = "DATE",
            required = true,
            help = "Date of the close account (YYYY-MM-DD)"
        )]
        date: String,
    },

    /// Rename an account
    Rename {
        /// Account id
        #[arg(index = 1, value_name = "ID", required = true, help = "Account ID")]
        id: String,
        /// New cccount name
        #[arg(
            index = 2,
            value_name = "NAME",
            required = true,
            help = "new account name"
        )]
        name: Vec<String>,
    },
    /// Set bank to current account
    SetBank {
        /// Bank id
        #[arg(value_name = "ID", required = true, help = "Bank ID")]
        id: String,
    },
    /// Set currency to current account
    SetCurrency {
        /// Currency id
        #[arg(value_name = "ID", required = true, help = "Currency ID")]
        id: String,
    },
    /// Set context to current account
    SetContext {
        /// Overdraft limit
        #[arg(
            short,
            long,
            value_name = "OVERDRAFT",
            help = "Overdraft limeit (e.g. 500.00)"
        )]
        overdraft: Option<String>,
        /// Minimiun balance
        #[arg(
            short,
            long,
            value_name = "BALANCE_MIN",
            help = "Minimiun balance reequired (e.g. 100.00)"
        )]
        balance_min: Option<String>,
        /// Max monthly transactions
        #[arg(
            short,
            long,
            value_name = "COUNT",
            help = "Max number of transactions per month"
        )]
        max_monthly_transactions: Option<String>,
        /// Deposit locked until date
        #[arg(
            short,
            long,
            value_name = "DEPOSIT_LOCKED_DATE (YYYY-MM-DD)",
            help = "deposit locked until date"
        )]
        deposit_locked_until: Option<String>,
        /// Allow interest
        #[arg(short, long, help = "Allow interest")]
        interest: Option<bool>,
        /// Allow signers
        #[arg(short, long, help = "Allow signers")]
        signers: Option<bool>,
    },
}
