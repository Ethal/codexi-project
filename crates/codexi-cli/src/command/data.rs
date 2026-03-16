// src/command/data.rs

use clap::{Args, Subcommand, ValueEnum};

/// Available exchange format
#[derive(Debug, Clone, ValueEnum)]
pub enum ExchangeFormat {
    Csv,
    Toml,
    Json,
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct DataArgs {
    #[command(subcommand)]
    pub command: DataCommand,
}

/// Manage data mobility (Import/Export) and local safety snapshots
#[derive(Subcommand, Debug)]
pub enum DataCommand {
    /// Export the data to an external format (CSV, TOML, JSON)
    Export {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    /// Import data from an external format (CSV, TOML, JSON)
    Import {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    ///Manage local snapshots (Quick-save points before major changes)
    Snapshot(SnapshotArgs),
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct SnapshotArgs {
    #[command(subcommand)]
    pub command: SnapshotCommand,
}

///Manage local snapshots (Quick-save points before major changes)
#[derive(Subcommand, Debug)]
pub enum SnapshotCommand {
    /// Create a snapshot
    Create {},

    /// list the available snapshot
    List {},

    /// Restore a snapshot
    Restore {
        #[arg(
            help = "Used 'ListSnapShot' for the available snapshot files",
            value_name = "SNAPSHOT_FILE"
        )]
        snapshot_file: String,
    },

    /// Remove old snapshot files, keeping only the 5 most recent ones by default
    Clean {
        #[arg(
            short,
            long,
            help = "Number of most recent snapshots to keep (default: 5)",
            value_name = "KEEP_FILE"
        )]
        keep: Option<usize>,
    },
}
