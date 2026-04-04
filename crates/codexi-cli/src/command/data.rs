// src/command/data.rs

use clap::{Args, Subcommand, ValueEnum};

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
    Export(ExchangeTypeArgs),

    /// Import data from an external format (CSV, TOML, JSON)
    Import(ExchangeTypeArgs),

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

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct ExchangeTypeArgs {
    #[command(subcommand)]
    pub command: ExchangeTypeCommand,
}

///Manage exchange type
#[derive(Subcommand, Debug)]
pub enum ExchangeTypeCommand {
    /// Exchange account header
    AccountHeader {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    /// Exchange operation
    Operation {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    /// Exchange currency
    Currency {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },
    /// Exchange counterparty
    Counterparty {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },
}

/// Available exchange format
#[derive(Debug, Clone, ValueEnum)]
pub enum ExchangeFormat {
    Csv,
    Toml,
    Json,
}
