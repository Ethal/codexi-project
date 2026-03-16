// src/handler/data.rs

use anyhow::Result;
use std::path::Path;

use codexi::{core::DataPaths, file_management::FileManagement};

use crate::prompts::Prompt;
use crate::ui::{view_snapshot, view_warning};
use crate::{
    command::{DataCommand, ExchangeFormat, SnapshotCommand},
    msg_info, msg_warn,
};

pub fn handle_data_command(
    command: DataCommand,
    cwd: &Path,
    paths: &DataPaths,
    skip_confirm: bool,
) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    let account = codexi.get_current_account_mut()?;
    match command {
        DataCommand::Export { format } => match format {
            ExchangeFormat::Csv => {
                msg_warn!("Export CSV not yet supported");
            }
            ExchangeFormat::Toml => {
                FileManagement::export_toml(account, cwd)?;
                msg_info!("Export TOML completed");
            }
            ExchangeFormat::Json => {
                FileManagement::export_json(account, cwd)?;
                msg_info!("Export JSON completed");
            }
        },
        DataCommand::Import { format } => match format {
            ExchangeFormat::Csv => {
                msg_info!("Import CSV not yet supported");
            }
            ExchangeFormat::Toml => {
                if !skip_confirm && !Prompt::confirm("Import the data?", false)? {
                    msg_info!("Operation cancelled.");
                    return Ok(());
                }
                let (new_account, warnings) = FileManagement::import_toml(cwd)?;
                let summary = codexi.import_account(new_account)?;
                FileManagement::save_current_state(&codexi, paths)?;
                msg_info!(
                    "Import in {}: {} created, {} updated.",
                    summary.account_name,
                    summary.created,
                    summary.updated
                );
                if !warnings.is_empty() {
                    view_warning(&warnings);
                    msg_warn!("Import TOML completed, {} warnings", warnings.len());
                } else {
                    msg_info!("Import TOML completed");
                }
            }
            ExchangeFormat::Json => {
                if !skip_confirm && !Prompt::confirm("Import the data?", false)? {
                    msg_info!("Operation cancelled.");
                    return Ok(());
                }
                let (new_account, warnings) = FileManagement::import_json(cwd)?;
                let summary = codexi.import_account(new_account)?;
                FileManagement::save_current_state(&codexi, paths)?;
                msg_info!(
                    "Import in {}: {} created, {} updated.",
                    summary.account_name,
                    summary.created,
                    summary.updated
                );
                if !warnings.is_empty() {
                    view_warning(&warnings);
                    msg_warn!("Import JSON completed, {} warnings", warnings.len());
                } else {
                    msg_info!("Import JSON completed");
                }
            }
        },
        DataCommand::Snapshot(snapshot) => match snapshot.command {
            SnapshotCommand::Create {} => {
                FileManagement::create_snapshot(&codexi, paths)?;
                msg_info!("Snapshot done");
            }
            SnapshotCommand::List {} => {
                let snapshot = FileManagement::list_snapshot(paths)?;
                view_snapshot(&snapshot);
            }
            SnapshotCommand::Restore { snapshot_file } => {
                if !skip_confirm && !Prompt::confirm("Restore the snapshot ?", false)? {
                    msg_info!("Operation cancelled.");
                    return Ok(());
                }
                let codexi = FileManagement::restore_snapshot(paths, &snapshot_file)?;
                FileManagement::save_current_state(&codexi, paths)?;
                msg_info!("Restore of the snapshot ({}) completed", snapshot_file);
            }

            SnapshotCommand::Clean { keep } => {
                FileManagement::clean_snapshot(paths, keep)?;
                msg_info!("Cleaning of snapshot done");
            }
        },
    }
    Ok(())
}
