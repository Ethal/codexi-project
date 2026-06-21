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
    /// Display technical metadata for the current ledger (version, paths, size, etc.).
    Infos,

    /// Create a backup of the ledger (`codexi.dat`) and its archive files.
    Backup {
        #[arg(
            long,
            value_name = "DIR or PATH",
            help = "Target directory or full path for the backup file. If a directory is provided, a timestamped filename will be generated."
        )]
        target_dir: Option<String>,
    },

    /// ⚠️  Restore the ledger (`codexi.dat` + archives) from a backup file.
    Restore {
        #[arg(value_name = "FILENAME", help = "The backup file to restore from.")]
        filename: String,
    },

    /// Run integrity checks on the ledger (balances, links, policies).
    Audit {
        #[arg(short, long, help = "Rebuild balances for the current account.")]
        rebuild: bool,
    },

    /// ⚠️  Move all ledger-related files from the active directory to the trash.
    ClearData,

    /// Manage the application trash (recover or purge deleted files).
    Trash(TrashArgs),

    /// Export all ledger data to a JSON file (full snapshot).
    ExportSpecial,

    /// ⚠️  Restore all ledger data from a JSON file (no validation performed).
    ImportSpecial,

    /// Generate a replayable shell script from the ledger's operations.
    ExportScript,
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct TrashArgs {
    #[command(subcommand)]
    pub command: TrashCommand,
}

/// Manage the application trash (recover or purge deleted files).
#[derive(Subcommand, Debug)]
pub enum TrashCommand {
    /// ⚠️  Restore a ledger from the trash to the active directory.
    Restore {
        /// Date of the trash folder to restore (format: `YYYYMMDD_HHMMSS`).
        #[arg(value_name = "DATE_TIME")]
        date: String,
    },

    /// ⚠️  Permanently delete all files in the trash.
    Purge,
}
