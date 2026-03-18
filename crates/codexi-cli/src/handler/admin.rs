// src/handler/admin.rs

use anyhow::{Result, bail};
use chrono::Local;
use rust_decimal::Decimal;
use std::path::{Path, PathBuf};

use codexi::{
    core::DataPaths,
    file_management::FileManagement,
    logic::{
        codexi::{Codexi, migrate_v1, migrate_v2},
        operation::{OperationFlow, OperationKind, SystemKind},
    },
};

use crate::prompts::Prompt;
use crate::ui::{view_codexi_infos, view_warning};
use crate::{
    command::{AdminCommand, TrashCommand},
    msg_info, msg_warn,
};

pub fn handle_admin_command(
    command: AdminCommand,
    cwd: &Path,
    paths: &DataPaths,
    skip_confirm: bool,
) -> Result<()> {
    match command {
        AdminCommand::Backup { target_dir } => {
            let backup_file = FileManagement::create_backup(paths, target_dir.as_deref())?;
            msg_info!("Backup completed to: {}", backup_file.display());
        }
        AdminCommand::Restore { filename } => {
            if !skip_confirm && !Prompt::confirm("Restore the backup file ?", false)? {
                msg_info!("Operation cancelled.");
                return Ok(());
            }
            let full_path = PathBuf::from(filename);
            FileManagement::restore_backup(paths, &full_path)?;
            msg_info!("Restore backup completed");
        }

        AdminCommand::Migrate { version } => match version {
            2 => {
                migrate_v1(paths)?;
                msg_info!("Migrate done");
            }
            3 => {
                let warnings = migrate_v2(paths)?;
                if !warnings.is_empty() {
                    view_warning(&warnings);
                    msg_warn!("Migration ompleted, {} warnings", warnings.len());
                } else {
                    msg_info!("Migration completed");
                }
            }
            _ => bail!("Migration not supported"),
        },
        AdminCommand::Audit { rebuild } => {
            let mut codexi = FileManagement::load_current_state(paths)?;
            let (warnings, name) = {
                let account = codexi.get_current_account_mut()?;
                if rebuild {
                    (account.audit_and_rebuild()?, account.name.clone())
                } else {
                    (account.audit()?, account.name.clone())
                }
            }; // borrow account released here

            if rebuild {
                FileManagement::save_current_state(&codexi, paths)?;
            }

            if !warnings.is_empty() {
                msg_warn!(
                    "Account: {} Audit completed, {} warnings",
                    name,
                    warnings.len()
                );
                view_warning(&warnings);
            } else {
                msg_info!("Account: {} Audit completed, no warnings", name);
            }
        }
        AdminCommand::ClearData => {
            if !skip_confirm && !Prompt::critical_confirm("Clear all the current data ?", "CLEAR")?
            {
                msg_info!("Operation cancelled.");
                return Ok(());
            }
            FileManagement::clear_data(paths)?;
            msg_warn!(
                "Codexi cleared (all data files move to the trash under `data_dir/trash/` include snapshots and archives)"
            );
        }
        AdminCommand::Trash(trash) => match trash.command {
            TrashCommand::RestoreTrash { date } => {
                if !skip_confirm
                    && !Prompt::critical_confirm("Restore all the current data ?", "RESTORE")?
                {
                    msg_info!("Operation cancelled.");
                    return Ok(());
                }
                FileManagement::restore_trash(paths, date)?;
                msg_warn!("Restore a codexi from the trash ");
            }
            TrashCommand::PurgeTrash => {
                if !skip_confirm && !Prompt::critical_confirm("Purge the trash", "PURGE")? {
                    msg_info!("Operation cancelled.");
                    return Ok(());
                }
                FileManagement::clean_trash(paths)?;
                msg_warn!("Emptying the trash");
            }
        },
        AdminCommand::Infos => {
            let codexi = FileManagement::load_current_state(paths)?;
            let infos = FileManagement::codexi_infos(paths, &codexi)?;
            view_codexi_infos(&infos);
        }
        AdminCommand::ExportSpecial => {
            let codexi = FileManagement::load_current_state(paths)?;
            FileManagement::export_special_json(&codexi, cwd)?;
            msg_info!("Export special JSON completed");
        }
        AdminCommand::ImportSpecial => {
            if !skip_confirm && !Prompt::confirm("Import (no validation)?", false)? {
                msg_info!("Operation cancelled.");
                return Ok(());
            }
            let codexi = FileManagement::import_special_json(cwd)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Import special JSON completed");
        }
        AdminCommand::ExportScript => {
            let codexi = FileManagement::load_current_state(paths)?;
            handle_export_script(&codexi, cwd)?;
        }
    }
    Ok(())
}

fn handle_export_script(codexi: &Codexi, cwd: &Path) -> Result<()> {
    for account in &codexi.accounts {
        let mut lines = Vec::new();

        lines.push("#!/bin/bash".to_string());
        lines.push("set -e".to_string());
        lines.push(String::new());
        lines.push(format!("# Script generated for account: {}", account.name));
        lines.push(format!("# Account ID: {}", account.id));
        lines.push(format!("# Generated at: {}", Local::now().date_naive()));
        lines.push(String::new());

        lines.push(format!(
            "codexi-cli account create {} {} --type {}",
            account.open_date,
            account.name,
            account.context.account_type.as_str()
        ));
        lines.push(format!(
            "codexi-cli account set-context -o {} -b {}",
            account.context.overdraft_limit, account.context.min_balance,
        ));
        for op in &account.operations {
            let line = match op.kind {
                OperationKind::System(SystemKind::Init) => {
                    let amount = match op.flow {
                        OperationFlow::Credit => op.amount,
                        OperationFlow::Debit => -op.amount,
                        OperationFlow::None => Decimal::ZERO,
                    };
                    format!("codexi-cli history init {} {}", op.date, amount)
                }
                OperationKind::System(SystemKind::Adjust) => {
                    format!(
                        "# codexi-cli history adjust {} {} # {}",
                        op.date, op.balance, op.description
                    )
                }
                OperationKind::System(SystemKind::Checkpoint) => {
                    format!(
                        "# codexi-cli history checkpoint {} \"{}\"",
                        op.date, op.description
                    )
                }
                OperationKind::System(SystemKind::Void) => {
                    "# VOID op — skip (handled by void_of link on target)".to_string()
                }
                OperationKind::Regular(_) => {
                    if op.links.void_by.is_some() {
                        format!(
                            "# VOIDED: codexi-cli {} {} {} \"{}\"",
                            op.flow.as_str().to_lowercase(),
                            op.date,
                            op.amount,
                            op.description
                        )
                    } else {
                        format!(
                            "codexi-cli {} {} {} \"{}\"",
                            op.flow.as_str().to_lowercase(),
                            op.date,
                            op.amount,
                            op.description
                        )
                    }
                }
            };
            lines.push(line);
        }

        // un fichier par account
        let filename = format!(
            "script_{}_{}.sh",
            account.name.to_lowercase().replace(' ', "_"),
            account.id
        );
        let file_path = cwd.join(&filename);
        std::fs::write(&file_path, lines.join("\n"))?;

        msg_info!("Script exported: {}", file_path.display());
    }

    Ok(())
}
