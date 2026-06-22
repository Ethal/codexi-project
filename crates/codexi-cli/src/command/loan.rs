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

    /// Simulate a loan and display amount due, interest, and repayment schedule
    Simulate {
        /// Capital amount (e.g., 1_000_000)
        #[arg(
            long,
            value_name = "CAPITAL",
            required = true,
            help = "Loan capital amount (positive, e.g., 1_000_000)"
        )]
        capital: String,

        /// Loan start date
        #[arg(
            long,
            value_name = "START_DATE",
            required = true,
            help = "Loan start date (YYYY-MM-DD)"
        )]
        start: String,

        /// Expected refund date
        #[arg(
            long,
            value_name = "REFUND_DATE",
            required = true,
            help = "Expected refund date (YYYY-MM-DD)"
        )]
        refund: String,

        /// Override policy type for this simulation only
        #[arg(
            long = "type",
            value_name = "INTEREST_TYPE",
            help = "Interest type: 'linear' or 'compound' (overrides policy)"
        )]
        interest_type: Option<String>,

        /// Override daily interest rate for this simulation only
        #[arg(
            long,
            value_name = "RATE",
            help = "Daily interest rate in % (0-100, overrides policy)"
        )]
        rate: Option<String>,

        /// Override free period for this simulation only
        #[arg(
            long,
            value_name = "DAYS",
            help = "Free period in days before interest applies (overrides policy)"
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
        /// Interest type: 'linear' or 'compound'
        #[arg(
            long = "type",
            value_name = "INTEREST_TYPE",
            help = "Interest type: 'linear' or 'compound'"
        )]
        interest_type: Option<String>,

        /// Daily interest rate in %
        #[arg(long, value_name = "RATE", help = "Daily interest rate in % (0-100)")]
        rate: Option<String>,

        /// Free period in days before interest applies
        #[arg(long, value_name = "DAYS", help = "Free period in days before interest applies")]
        free_days: Option<u32>,

        /// Max interest cap as % of capital
        #[arg(long, value_name = "MAX_CAP_PCT", help = "Max interest cap as % of capital (0-100)")]
        max_cap: Option<String>,

        /// Max loan duration in days
        #[arg(long, value_name = "MAX_DAYS", help = "Max loan duration in days")]
        max_days: Option<u32>,

        /// Minimum capital required
        #[arg(
            long,
            value_name = "MIN_CAPITAL",
            help = "Minimum capital required (positive amount)"
        )]
        min_capital: Option<String>,

        /// Max penalty as % of capital
        #[arg(long, value_name = "MAX_PENALTY_PCT", help = "Max penalty as % of capital (0-100)")]
        max_penalty: Option<String>,
    },

    /// Reset loan policy to default values
    Reset,
}
