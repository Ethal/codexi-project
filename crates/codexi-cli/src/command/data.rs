// src/command/data.rs

use clap::{Args, Subcommand, ValueEnum};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct DataArgs {
    #[command(subcommand)]
    pub command: DataCommand,
}

/// Import/export data and manage snapshots
#[derive(Subcommand, Debug)]
pub enum DataCommand {
    /// Export ledger data to CSV, TOML, or JSON
    Export(ExchangeTypeArgs),

    /// Import ledger data from CSV, TOML, or JSON
    Import(ExchangeTypeArgs),

    /// Manage ledger snapshots (save/restore state before major changes)
    Snapshot(SnapshotArgs),
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct SnapshotArgs {
    #[command(subcommand)]
    pub command: SnapshotCommand,
}

/// Manage Codexi snapshots
#[derive(Subcommand, Debug)]
pub enum SnapshotCommand {
    /// Create a snapshot of the current ledger
    Create,

    /// List all available snapshots
    List,

    /// [WARN] Restore the ledger from a snapshot
    Restore {
        /// Snapshot file to restore
        #[arg(
            help = "Use 'data snapshot list' to see available snapshot files",
            value_name = "SNAPSHOT_FILE"
        )]
        snapshot_file: String,
    },

    /// Delete old snapshots (keeps 5 most recent by default)
    Clean {
        /// Number of most recent snapshots to keep
        #[arg(
            short,
            long,
            value_name = "N",
            default_value_t = 5,
            help = "Number of most recent snapshots to keep (default: 5)"
        )]
        keep: usize,
    },
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct ExchangeTypeArgs {
    #[command(subcommand)]
    pub command: ExchangeTypeCommand,
}

/// Exchange data types
#[derive(Subcommand, Debug)]
pub enum ExchangeTypeCommand {
    /// Account metadata (name, context, bank, currency, etc.)
    AccountHeader {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    /// Operations (transactions, transfers, etc.)
    Operation {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    /// Currency list
    Currency {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    /// Category list
    Category {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },

    /// Counterparty list
    Counterparty {
        #[arg(value_enum)]
        format: ExchangeFormat,
    },
}

/// Available export/import formats
#[derive(Debug, Clone, ValueEnum)]
pub enum ExchangeFormat {
    /// Comma-Separated Values
    Csv,
    /// Tom's Obvious, Minimal Language
    Toml,
    /// JavaScript Object Notation
    Json,
}
