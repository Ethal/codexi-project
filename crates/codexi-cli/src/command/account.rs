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
    /// List all accounts, prefix (*) for Current account, (c) for Close account
    List,

    /// Show the context of the current account
    Context,

    /// Create a new account
    Create {
        /// Account openning date
        #[arg(
            value_name = "DATE",
            required = true,
            help = "Opening date of the account (YYYY-MM-DD)"
        )]
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
            help = "Account type (Current, Joint, Saving, Deposit, Loan, Business, Student)"
        )]
        account_type: Option<String>,
    },

    /// Switch active account
    Use {
        /// Account id
        #[arg(
            value_name = "ID",
            required = true,
            help = "Account ID. Accept full ID, short ID or name of the account"
        )]
        id: String,
    },

    /// !!! Close an account (no future action will be available)
    Close {
        /// Account id
        #[arg(
            index = 1,
            value_name = "ID",
            required = true,
            help = "Account ID. Accept full ID, short ID or name of the account"
        )]
        id: String,
        /// Closing date
        #[arg(
            index = 2,
            value_name = "DATE",
            required = true,
            help = "Closing date of the account (YYYY-MM-DD)"
        )]
        date: String,
    },

    /// Rename an account
    Rename {
        /// Account id
        #[arg(
            index = 1,
            value_name = "ID",
            required = true,
            help = "Account ID. Accept full ID, short ID or name of the account"
        )]
        id: String,
        /// New cccount name
        #[arg(index = 2, value_name = "NAME", required = true, help = "new account name")]
        name: Vec<String>,
    },
    /// Set bank to current account
    SetBank {
        /// Bank id
        #[arg(
            value_name = "ID",
            required = true,
            help = "Bank ID. Accept full ID, short ID or name of the bank"
        )]
        id: String,
    },
    /// Set currency to current account
    SetCurrency {
        /// Currency id
        #[arg(
            value_name = "ID",
            required = true,
            help = "Currency ID. Accept full ID, short ID or code of the currency"
        )]
        id: String,
        /// Update all the operations with the account currency
        #[arg(short, long, help = "Update all the operations with the account currency")]
        update_operation: bool,
    },
    /// Set context to current account
    SetContext {
        /// Overdraft limit
        #[arg(
            short = 'o',
            long,
            value_name = "OVERDRAFT",
            allow_negative_numbers = false,
            help = "Overdraft limeit, shall be positive (e.g. 500.00)"
        )]
        overdraft: Option<String>,
        /// Minimiun balance
        #[arg(
            short = 'b',
            long,
            value_name = "BALANCE_MIN",
            allow_negative_numbers = false,
            help = "Minimiun balance reequired, shall be positive (e.g. 100.00)"
        )]
        balance_min: Option<String>,
        /// Max monthly transactions
        #[arg(
            short = 'm',
            long,
            value_name = "COUNT",
            allow_negative_numbers = false,
            help = "Max number of transactions per month, shall be positive"
        )]
        max_monthly_transactions: Option<String>,
        /// Deposit locked until date
        #[arg(
            short = 'd',
            long,
            value_name = "DEPOSIT_LOCKED_DATE",
            help = "Deposit locked until date (YYYY-MM-DD)"
        )]
        deposit_locked_until: Option<String>,
        /// Allow interest
        #[arg(short = 'i', long, help = "Allow interest")]
        interest: Option<bool>,
        /// Allow signers
        #[arg(short = 's', long, help = "Allow signers")]
        signers: Option<bool>,
    },
}
