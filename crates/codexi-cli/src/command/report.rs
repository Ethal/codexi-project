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
    /// View a summary of the current account.
    Summary {},

    /// View balance and debit/credit. Available criteria, --from --to.
    Balance {
        // Filtres granulaire (Plage de dates arbitraire)
        #[arg(long, value_name = "FROM_DATE", help = "Start date for filtering operations")]
        from: Option<String>,

        #[arg(long, value_name = "TO_DATE", help = "End date for filtering operations")]
        to: Option<String>,
    },

    /// Monthly report
    Monthly {
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },

    /// View financial statistics and analytics for active operations
    /// NOTE:
    /// Stats are time-based.
    /// By default, voided operations are excluded, even if voided outside the period.
    Financial {
        #[arg(long, value_name = "YYYY-MM-DD", help = "Start date for stats")]
        from: Option<String>,

        #[arg(long, value_name = "YYYY-MM-DD", help = "End date for stats")]
        to: Option<String>,

        #[arg(long, help = "open the stats with defaut browser")]
        open: bool,
    },

    /// Counter report
    Counterparty {
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },

    /// export (html file) a statement of the current account. Available criteria, --from --to.  .
    Statement {
        /// Arbitrary date range
        #[arg(long, help = "Start date for filtering operations", value_name = "FROM_DATE")]
        from: Option<String>,

        /// Arbitrary date range
        #[arg(long, help = "End date for filtering operations", value_name = "TO_DATE")]
        to: Option<String>,

        /// open in the defaut browser
        #[arg(long, help = "open the statemnt with defaut browser")]
        open: bool,
    },
}
