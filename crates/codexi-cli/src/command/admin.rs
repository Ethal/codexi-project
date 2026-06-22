// src/command/admin.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommand,
}

/// Technical maintenance, disaster recovery, and low-level file management.
/// Use with caution; run `--help` for details.
#[derive(Subcommand, Debug)]
pub enum AdminCommand {
    /// Show technical metadata (version, paths, size, etc.)
    Infos,

    /// Create a backup of the ledger and archive files
    Backup {
        #[arg(
            long,
            value_name = "DIR or PATH",
            help = "Backup destination directory or file path. If a directory is provided, a timestamped file is created."
        )]
        target_dir: Option<String>,
    },

    /// Restore the ledger from a backup file
    Restore {
        #[arg(value_name = "FILENAME", help = "Backup file to restore from")]
        filename: String,
    },

    /// Run integrity checks on the ledger
    Audit {
        #[arg(short, long, help = "Rebuild balances for the current account")]
        rebuild: bool,
    },

    /// Move all ledger files to trash
    ClearData,

    /// Manage trash (restore or permanently delete files)
    Trash(TrashArgs),

    /// Export full ledger data as JSON
    ExportSpecial,

    /// Import full ledger data from JSON (no validation)
    ImportSpecial,

    /// Generate a replayable shell script from ledger operations
    ExportScript,
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct TrashArgs {
    #[command(subcommand)]
    pub command: TrashCommand,
}

/// Manage trash (restore or purge deleted files)
#[derive(Subcommand, Debug)]
pub enum TrashCommand {
    /// [WARN] Restore a ledger from trash
    Restore {
        /// Trash timestamp (YYYYMMDD_HHMMSS)
        #[arg(value_name = "DATE_TIME")]
        date: String,
    },

    /// [WARN] Permanently delete all trash files
    Purge,
}
