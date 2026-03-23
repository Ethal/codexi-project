// src/handler/history.rs

use anyhow::Result;

use codexi::{
    core::{DataPaths, parse_date, parse_decimal, parse_id, parse_text},
    file_management::FileManagement,
    logic::account::{AccountError, SearchParamsBuilder, search},
    logic::operation::Operation,
    logic::utils::resolve_id,
};

use crate::{
    command::{ArchiveCommand, HistoryCommand},
    msg_info,
    ui::{view_archive, view_search},
};

pub fn handle_history_command(command: HistoryCommand, paths: &DataPaths) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    let account = codexi.get_current_account_mut()?;
    match command {
        HistoryCommand::Init {
            date,
            initial_amount,
        } => {
            let date = parse_date(&date)?;
            let initial_amount_d = parse_decimal(&initial_amount, "initial_amount")?;
            account.initialize(date, initial_amount_d)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!(
                "codexi initialized with a balance of {:.2} on {}.",
                initial_amount_d,
                date
            );
        }
        HistoryCommand::Adjust {
            date,
            physical_amount,
        } => {
            let physical_amount_d = parse_decimal(&physical_amount, "physical_amount")?;
            let date = parse_date(&date)?;
            account.adjust_balance(date, physical_amount_d)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Adjust done: {} {}", date, physical_amount_d);
        }
        HistoryCommand::Close { date, description } => {
            let date = parse_date(&date)?;
            let description = parse_text(description);
            account.checkpoint(date, description, paths)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Close period of the {} completed", date);
        }
        HistoryCommand::Void { id } => {
            let mut codexi = FileManagement::load_current_state(paths)?;
            let op_id = {
                let account = codexi.get_current_account()?;
                resolve_id::<Operation, AccountError>(&id, &account.operations)?
            };
            codexi.void_from_current(op_id)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Operation {} voided.", op_id);
        }
        HistoryCommand::Archive(archive) => match archive.command {
            ArchiveCommand::List {} => {
                let results = FileManagement::list_archive(paths, account.id)?;
                view_archive(&results);
            }
            ArchiveCommand::View { id, date } => {
                let id = parse_id(&id)?;
                let date = parse_date(&date)?;
                let codexi_arch = FileManagement::load_archive(id, date, paths)?;
                let params = SearchParamsBuilder::default().build()?;

                let results = search(&codexi_arch, &params)?;
                view_search(&results);
            }
        },
    }
    Ok(())
}
