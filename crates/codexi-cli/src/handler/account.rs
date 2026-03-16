// src/handler/account.rs

use anyhow::Result;

use codexi::{
    core::{DataPaths, parse_date, parse_id, parse_optional_id, parse_text},
    file_management::FileManagement,
    logic::account::Account,
};

use crate::ui::view_account;
use crate::{command::AccountCommand, msg_info};

pub fn handle_account_command(command: AccountCommand, paths: &DataPaths) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    match command {
        AccountCommand::List => {
            let items = codexi.account_entry();
            view_account(&items);
        }
        AccountCommand::Create { date, name } => {
            let date = parse_date(&date)?;
            let name = parse_text(name);
            let bank_id = parse_optional_id(Some(""))?;
            let currency_id = parse_optional_id(Some(""))?;
            let new_account = Account::new(date, name, bank_id, currency_id)?;
            codexi.add_account(new_account);
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Account created and activated.");
        }

        AccountCommand::Use { id } => {
            let id_n = parse_id(&id)?;
            codexi.set_current_account(&id_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Switched to account {}", id);
        }

        AccountCommand::Close { id, date } => {
            let id_n = parse_id(&id)?;
            let date = parse_date(&date)?;
            codexi.close_account(id_n, date)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Account closed.");
        }

        AccountCommand::Rename { id, name } => {
            let id_n = parse_id(&id)?;
            let account = codexi.get_account_by_id_mut(&id_n)?;
            account.name = parse_text(name);
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Account renamed.");
        }
        AccountCommand::SetBank { id } => {
            let id_n = parse_id(&id)?;
            codexi.set_account_bank(&id_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Bank set.");
        }
        AccountCommand::SetCurrency { id } => {
            let id_n = parse_id(&id)?;
            codexi.set_account_currency(&id_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Currency set.");
        }
    }
    Ok(())
}
