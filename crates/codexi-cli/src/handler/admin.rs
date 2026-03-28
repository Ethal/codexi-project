// src/handler/admin.rs

use anyhow::{Result, bail};
use chrono::Local;
use rust_decimal::Decimal;
use std::path::{Path, PathBuf};

use codexi::{
    core::{DataPaths, format_max_monthly_transactions, format_optional_date, format_optional_id},
    file_management::FileManagement,
    logic::{
        codexi::{Codexi, migrate_v1, migrate_v2},
        operation::{OperationFlow, OperationKind, RegularKind, SystemKind},
    },
};

use crate::{
    command::{AdminCommand, TrashCommand},
    msg_info, msg_warn,
    prompts::Prompt,
    ui::{view_codexi_infos, view_warning},
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
                msg_info!("Command cancelled.");
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
                msg_info!("Command cancelled.");
                return Ok(());
            }
            FileManagement::clear_data(paths)?;
            msg_warn!(
                "Codexi cleared (all data files move to the trash under `data_dir/trash/` include snapshots and archives)"
            );
        }
        AdminCommand::Trash(trash) => match trash.command {
            TrashCommand::Restore { date } => {
                if !skip_confirm
                    && !Prompt::critical_confirm("Restore all the current data ?", "RESTORE")?
                {
                    msg_info!("Command cancelled.");
                    return Ok(());
                }
                FileManagement::restore_trash(paths, date)?;
                msg_warn!("Restore a codexi from the trash ");
            }
            TrashCommand::Purge => {
                if !skip_confirm && !Prompt::critical_confirm("Purge the trash", "PURGE")? {
                    msg_info!("command cancelled.");
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
                msg_info!("Command cancelled.");
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
    let mut lines_codexi = Vec::new();
    lines_codexi.push("#!/bin/bash".to_string());
    lines_codexi.push("set -e".to_string());
    lines_codexi.push(String::new());
    lines_codexi.push(format!("# Script generated for codexi: {}", codexi.name));
    lines_codexi.push(format!("# Codexi ID: {}", codexi.id));
    lines_codexi.push(format!("# Generated at: {}", Local::now().date_naive()));
    lines_codexi.push(String::new());
    lines_codexi.push("codexi-cli report balance-all".to_string());
    lines_codexi.push("codexi-cli admin backup".to_string());
    lines_codexi.push("codexi-cli admin clear-data".to_string());

    for account in &codexi.accounts {
        lines_codexi.push(format!(
            "codexi-cli account create {} {} --type {}",
            account.open_date,
            account.name,
            account.context.account_type.as_str().to_lowercase(),
        ));
        lines_codexi.push(format!(
            "# codexi-cli account set-context -o {} -b {} -m {} -d {} -i {} -s {}",
            account.context.overdraft_limit,
            account.context.min_balance,
            format_max_monthly_transactions(account.context.max_monthly_transactions),
            format_optional_date(account.context.deposit_locked_until).unwrap_or("".into()),
            account.context.allows_interest,
            account.context.allows_joint_signers
        ));
        let code = account
            .currency_id
            .and_then(|cur_id| codexi.currencies.currency_code_by_id(&cur_id)) // retourne Option<String>
            .unwrap_or_else(|| codexi.settings.default_currency.clone()); // fallback si None
        lines_codexi.push(format!("# codexi-cli account set-currency {}", code));

        let bk_name = account
            .bank_id
            .and_then(|bk_id| codexi.banks.bank_name_by_id(&bk_id)) // retourne Option<String>
            .unwrap_or("".to_string()); // fallback si None
        lines_codexi.push(format!("# codexi-cli account set-bank {}", bk_name));

        let mut lines = Vec::new();
        lines.push("#!/bin/bash".to_string());
        lines.push("set -e".to_string());
        lines.push(String::new());
        lines.push(format!("# Script generated for account: {}", account.name));
        lines.push(format!("# Account ID: {}", account.id));
        lines.push(format!("# Generated at: {}", Local::now().date_naive()));
        lines.push(String::new());

        lines.push(format!("codexi-cli account use \"{}\"", account.name));
        for op in &account.operations {
            let line = match op.kind {
                OperationKind::System(SystemKind::Init) => {
                    let amount = match op.flow {
                        OperationFlow::Credit => op.amount,
                        OperationFlow::Debit => -op.amount,
                        OperationFlow::None => Decimal::ZERO,
                    };

                    lines_codexi.push(format!("codexi-cli account use \"{}\"", account.name));
                    lines_codexi.push(format!("codexi-cli history init {} {}", op.date, amount));
                    lines_codexi.push(String::new());

                    format!(
                        "#FYI - INIT in main codexi script: codexi-cli history init {} {}",
                        op.date, amount
                    )
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
                OperationKind::Regular(RegularKind::Transaction) => {
                    if op.links.void_by.is_some() {
                        format!(
                            "#VOIDED: codexi-cli {} {} {} \"{}\"",
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
                OperationKind::Regular(RegularKind::Fee) => {
                    format!(
                        "#FEE: codexi-cli fee {} {} \"{}\"",
                        op.date, op.amount, op.description
                    )
                }
                OperationKind::Regular(RegularKind::Refund) => {
                    format!(
                        "#REFUND: codexi-cli refund {} {} \"{}\"",
                        op.date, op.amount, op.description
                    )
                }
                OperationKind::Regular(RegularKind::Interest) => {
                    format!(
                        "codexi-cli interest {} {} \"{}\"",
                        op.date, op.amount, op.description
                    )
                }
                OperationKind::Regular(RegularKind::Transfer) => match op.flow {
                    OperationFlow::Debit => {
                        format!(
                            "#TRANSFER codexi-cli transfer {} {} {:?} {:?} \"{}\"",
                            op.date,
                            op.amount,
                            op.amount * op.context.exchange_rate,
                            format_optional_id(op.links.transfer_account_id),
                            op.description
                        )
                    }
                    OperationFlow::Credit => {
                        format!(
                            "#FYI - TRANSFER: check other account {} {} \"{}\"",
                            op.date, op.amount, op.description
                        )
                    }
                    OperationFlow::None => {
                        format!(
                            "#NONE: Kind {} {} {} \"{}\"",
                            op.kind.as_str(),
                            op.date,
                            op.amount,
                            op.description
                        )
                    }
                },
            };
            lines.push(line);
        }

        // one file per account
        let filename = format!(
            "script_{}_{}.sh",
            account.name.to_lowercase().replace(' ', "_"),
            account.id
        );
        let file_path = cwd.join(&filename);
        std::fs::write(&file_path, lines.join("\n"))?;

        msg_info!("Script exported: {}", file_path.display());
    }

    lines_codexi.push("codexi-cli account list".to_string());
    // one file for codexi
    let filename = format!(
        "script_{}_{}.sh",
        codexi.name.to_lowercase().replace(' ', "_"),
        codexi.id
    );
    let file_path = cwd.join(&filename);
    std::fs::write(&file_path, lines_codexi.join("\n"))?;
    msg_info!("Script exported: {}", file_path.display());
    msg_warn!("-----------------------------------------------------------------");
    msg_warn!("Review all the script before launch");
    msg_warn!("Paid attention of the overldraft, recommenand to increase,");
    msg_warn!("due to the fact that the rebuild is done account per account,");
    msg_warn!("order to execute the scripts:");
    msg_warn!(" -1 the main script_codexi");
    msg_warn!(" -2 script related to current account type");
    msg_warn!(" -3 script related to others account type");
    msg_warn!("-----------------------------------------------------------------");
    msg_info!("Job Export completed");

    Ok(())
}
