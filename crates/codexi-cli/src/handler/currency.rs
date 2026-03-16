// src/handler/currency.rs

use anyhow::Result;

use codexi::{core::DataPaths, file_management::FileManagement};

use crate::command::CurrencyCommand;
use crate::ui::view_currency;

pub fn handle_currency_command(command: CurrencyCommand, paths: &DataPaths) -> Result<()> {
    let codexi = FileManagement::load_current_state(paths)?;
    match command {
        CurrencyCommand::List => {
            let items = codexi.currencies.currency_entry();
            view_currency(&items);
        }
    }
    Ok(())
}
