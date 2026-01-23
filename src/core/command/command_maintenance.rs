// src/core/command/command_maintenance.rs

use clap::{Args, Subcommand};

/// structure Maintenance
#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct MaintenanceArgs {
    #[command(subcommand)]
    pub command: MaintenanceCommand,
}

#[derive(Subcommand, Debug)]
pub enum MaintenanceCommand {

    /// DANGER: Deleted all file related to ledger(codexi) in app directory.
    Clear,

    /// DANGER: Migration a old version of the codexi.dat, including archived file if any.
    Migrate {
        /// Version to migrate.
        #[arg(value_name = "VERSION",  help = "Version of the file to migrate.")]
        version: usize,
    },

    /// Get informations related the current ledger(codexi).
    LedgerInfos,
}
