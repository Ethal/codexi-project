// src/command/report.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct ReportArgs {
    #[command(subcommand)]
    pub command: ReportCommand,
}

/// Generate financial reports, statistics, and statements
#[derive(Subcommand, Debug)]
pub enum ReportCommand {
    /// Dashboard report.
    Dashboard {
        #[arg(long, value_name = "FROM_DATE", help = "Start date of the report(YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date of the report(YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// View a summary of the current account.
    Summary {},

    /// View balance and debit/credit.
    Balance {
        #[arg(long, value_name = "FROM_DATE", help = "Start date of the report(YYYY-MM-DD)")]
        from: Option<String>,

        #[arg(long, value_name = "TO_DATE", help = "End date of the report(YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Monthly report.
    Monthly {
        #[arg(long, value_name = "FROM_DATE", help = "Start date of the report(YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date of the report(YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// View financial statistics and analytics for active operations
    /// NOTE:
    /// Stats are time-based.
    /// By default, voided operations are excluded, even if voided outside the period.
    Financial {
        #[arg(long, value_name = "FROM_DATE", help = "Start date of the report(YYYY-MM-DD)")]
        from: Option<String>,

        #[arg(long, value_name = "TO_DATE", help = "End date of report(YYYY-MM-DD)")]
        to: Option<String>,

        #[arg(long, help = "open the stats with defaut browser")]
        open: bool,
    },

    /// Counterparty report.
    Counterparty {
        #[arg(long, value_name = "FROM_DATE", help = "Start date of the report(YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date of the report(YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// Category report.
    Category {
        #[arg(long, value_name = "FROM_DATE", help = "Start date of the report(YYYY-MM-DD)")]
        from: Option<String>,
        #[arg(long, value_name = "TO_DATE", help = "End date of the report(YYYY-MM-DD)")]
        to: Option<String>,
    },

    /// export (html file) a statement of the current account. Available criteria, --from --to.  .
    Statement {
        /// Arbitrary date range
        #[arg(long, help = "Start date of the report(YYYY-MM-DD)", value_name = "FROM_DATE")]
        from: Option<String>,

        /// Arbitrary date range
        #[arg(long, help = "End date of the report(YYYY-MM-DD)", value_name = "TO_DATE")]
        to: Option<String>,

        /// open in the defaut browser
        #[arg(long, help = "open the statemnt with defaut browser")]
        open: bool,
    },
}
