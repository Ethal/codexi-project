// src/handler/account.rs

use anyhow::Result;

use codexi::{
    core::{
        DataPaths, parse_date, parse_id, parse_optional_date, parse_optional_decimal,
        parse_optional_id, parse_optional_u32, parse_text,
    },
    file_management::FileManagement,
    logic::{account::{Account, AccountType}, codexi::CodexiError, utils::resolve_id,}
};

use crate::ui::{view_account, view_account_context, view_warning};
use crate::{command::AccountCommand, msg_info, msg_warn};

pub fn handle_account_command(command: AccountCommand, paths: &DataPaths) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    match command {
        AccountCommand::List => {
            let items = codexi.account_entry();
            view_account(&items);
        }
        AccountCommand::Context => {
            let current_account = codexi.get_current_account()?;
            let account_item = codexi.account_item(current_account);
            view_account_context(&account_item);
        }
        AccountCommand::Create {
            date,
            name,
            account_type,
        } => {
            let date = parse_date(&date)?;
            let name = parse_text(name);
            let bank_id = parse_optional_id(Some(""))?;
            let currency_id = parse_optional_id(Some(""))?;
            let account_type: AccountType = account_type
                .as_deref()
                .map(|s| s.parse().unwrap_or_default())
                .unwrap_or_default();
            let new_account = Account::new(date, name, account_type, bank_id, currency_id)?;
            codexi.add_account(new_account);
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Account created and activated.");
        }

        AccountCommand::Use { id } => {
            let id_n = resolve_id::<Account, CodexiError>(
                &id,
                &codexi.accounts,
            )?;
            codexi.set_current_account(&id_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Switched to account {}", id_n);
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
        AccountCommand::SetContext {
            overdraft,
            balance_min,
            max_monthly_transactions,
            deposit_locked_until,
            interest,
            signers,
        } => {
            let current_account = codexi.get_current_account_mut()?;

            let limit = parse_optional_decimal(&overdraft, "over draft")?;
            let min = parse_optional_decimal(&balance_min, "balance min")?;
            let max_monthly_transactions =
                parse_optional_u32(&max_monthly_transactions, "max_monthly transaction")?;
            let deposit_locked_until = parse_optional_date(&deposit_locked_until)?;

            let warnings = current_account.context.update_context(
                limit,
                min,
                Some(max_monthly_transactions),
                deposit_locked_until,
                interest,
                signers,
            )?;

            FileManagement::save_current_state(&codexi, paths)?;
            if !warnings.is_empty() {
                view_warning(&warnings);
                msg_warn!("Account context set with {} warnings", warnings.len());
            } else {
                msg_info!("Account context set");
            }
        }
    }
    Ok(())
}
