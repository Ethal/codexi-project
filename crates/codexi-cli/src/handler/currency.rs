// src/handler/currency.rs

use anyhow::Result;

use codexi::{core::DataPaths, dto::CurrencyCollection, file_management::FileManagement};

use crate::{command::CurrencyCommand, ui::view_currency};

pub fn handle_currency_command(command: CurrencyCommand, paths: &DataPaths) -> Result<()> {
    let codexi = FileManagement::load_current_state(paths)?;
    match command {
        CurrencyCommand::List => {
            let items = CurrencyCollection::build(&codexi.currencies);
            view_currency(&items);
        }
    }
    Ok(())
}
