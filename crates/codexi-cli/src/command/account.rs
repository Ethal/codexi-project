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

    /// Create a new account
    Create {
        /// Account openning date
        #[arg(
            index = 1,
            value_name = "DATE",
            required = true,
            help = "Account openning date"
        )]
        date: String,
        /// Account name
        #[arg(index = 2, value_name = "NAME", required = true, help = "Account name")]
        name: Vec<String>,
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
}
