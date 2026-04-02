// src/command/admin.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommand,
}

/// Technical maintenance, disaster recovery, and low-level file management.
/// To be use carefully, performed a --help is recommended
#[derive(Subcommand, Debug)]
pub enum AdminCommand {
    /// Get technical information about the current codexi file
    Infos,

    /// Backup(Zip file) the ledger(codexi.dat) include the archive file.
    Backup {
        #[arg(
            long,
            value_name = "DIR or PATH",
            help = "Target directory or full path for the backup ZIP file. If a directory is provided, a default filename with timestamp will be used."
        )]
        target_dir: Option<String>,
    },

    ///⚠️  Restore a ledger(codexi.dat) include the archive file from a backup file(Zip File).
    Restore {
        #[arg(
            value_name = "FILENAME",
            help = "The backup ZIP filename to restore from"
        )]
        filename: String,
    },

    ///⚠️  Migration an old version of the loger(codexi.dat), including archived file if any.
    Migrate {
        /// Version to migrate.
        #[arg(
            value_name = "VERSION",
            help = "Version of the file to migrate(1->2, 2->3)."
        )]
        version: usize,
    },

    /// Audit the Codexi.
    Audit {
        #[arg(short, long, help = "Balance rebuild of the current account")]
        rebuild: bool,
    },

    ///⚠️  Move all files related to the current ledger(Codexi) in the app active directory to the app trash directory.
    ClearData,

    /// Manage the application trash (Recover or purge deleted files).
    Trash(TrashArgs),

    /// Export all the data of the current codexi to json file
    ExportSpecial,

    ///⚠️  Restore all the data of the current codexi from json file, no validations is perfom
    ImportSpecial,

    /// Export operations in a script for a replay
    ExportScript,
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct TrashArgs {
    #[command(subcommand)]
    pub command: TrashCommand,
}

/// Manage the application trash (Recover or purge deleted files).
#[derive(Subcommand, Debug)]
pub enum TrashCommand {
    ///⚠️  Restore a Codexi from the app trash directory to the app active directory.
    Restore {
        /// date of the folder to restore.
        #[arg(value_name = "DATE_TIME", help = "Date format YYYYMMDD_HHMMSS.")]
        date: String,
    },

    ///⚠️  Emptying the trash in app directory.
    Purge,
}
