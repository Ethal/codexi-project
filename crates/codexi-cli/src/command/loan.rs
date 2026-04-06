// src/command/loan.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct LoanArgs {
    #[command(subcommand)]
    pub command: LoanCommand,
}

/// Loan policy and simulation commands
#[derive(Subcommand, Debug)]
pub enum LoanCommand {
    /// Manage the default loan policy
    Policy {
        #[command(subcommand)]
        action: LoanPolicyAction,
    },
    /// Simulate a loan and display the amount due
    Simulate {
        #[arg(
            long,
            value_name = "CAPITAL",
            required = true,
            allow_negative_numbers = false,
            help = "Capital amount (e.g. 1_000_000)"
        )]
        capital: String,

        #[arg(long, value_name = "DATE", required = true, help = "Loan start date (YYYY-MM-DD)")]
        start: String,

        #[arg(
            long,
            value_name = "DATE",
            required = true,
            help = "Expected refund date (YYYY-MM-DD)"
        )]
        refund: String,

        /// Override policy type for this simulation only
        #[arg(
            long = "type",
            value_name = "TYPE",
            help = "Interest type: 'linear' or 'compound' (overrides policy)"
        )]
        type_interest: Option<String>,

        /// Override interest rate for this simulation only
        #[arg(
            long,
            value_name = "RATE",
            allow_negative_numbers = false,
            help = "Daily interest rate in % (overrides policy)"
        )]
        rate: Option<String>,

        #[arg(
            long,
            value_name = "DAYS",
            allow_negative_numbers = false,
            help = "Free period in days before interest applies"
        )]
        free_days: Option<u32>,
    },
}

#[derive(Subcommand, Debug)]
pub enum LoanPolicyAction {
    /// Show the current loan policy
    Show,
    /// Set loan policy parameters (persisted to disk)
    Set {
        #[arg(
            long = "typeinterest",
            value_name = "TYPE",
            help = "Interest type: 'linear' or 'compound'"
        )]
        type_interest: Option<String>,

        #[arg(
            long,
            value_name = "RATE",
            allow_negative_numbers = false,
            help = "Daily interest rate in %"
        )]
        rate: Option<String>,

        #[arg(
            long,
            value_name = "DAYS",
            allow_negative_numbers = false,
            help = "Free period in days before interest applies"
        )]
        free_days: Option<u32>,

        #[arg(
            long,
            value_name = "PCT",
            allow_negative_numbers = false,
            help = "Max interest cap as % of capital (0-100)"
        )]
        max_cap: Option<String>,

        #[arg(
            long,
            value_name = "DAYS",
            allow_negative_numbers = false,
            help = "Max loan duration in days"
        )]
        max_days: Option<u32>,

        #[arg(
            long,
            value_name = "AMOUNT",
            allow_negative_numbers = false,
            help = "Minimum capital required"
        )]
        min_capital: Option<String>,

        #[arg(
            long,
            value_name = "PCT",
            allow_negative_numbers = false,
            help = "Max penalty as % of capital (0-100)"
        )]
        max_penalty: Option<String>,
    },
    /// Reset loan policy to default values
    Reset,
}
