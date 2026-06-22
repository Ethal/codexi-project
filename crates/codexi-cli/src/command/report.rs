// src/command/report.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct ReportArgs {
    #[command(subcommand)]
    pub command: ReportCommand,
}

/// Generate financial reports (dashboard, balance, monthly, etc.)
#[derive(Subcommand, Debug)]
pub enum ReportCommand {
    /// Dashboard report with key metrics
    Dashboard {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Summary of the current account
    Summary,

    /// Balance and debit/credit totals
    Balance {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Monthly report with breakdowns
    Monthly {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Financial statistics and analytics for active operations
    /// NOTE: Stats are time-based. Voided operations are excluded by default,
    /// even if voided outside the period.
    Financial {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
        #[arg(long, help = "Open report in the default browser")]
        open: bool,
    },

    /// Counterparty report with transaction history
    Counterparty {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Category report with spending breakdown
    Category {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Exchange rate report
    Rate {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Tree view: Counterparty → Category → Operations
    Tree {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Export account statement as HTML
    Statement {
        #[arg(long, value_name = "FROM_DATE", help = "Start date (YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date (YYYY-MM-DD)")]
        to: Option<String>,
        #[arg(long, help = "Open statement in the default browser")]
        open: bool,
    },
}
