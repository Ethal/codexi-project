// src/handler/bank.rs

use anyhow::Result;

use codexi::{core::DataPaths, file_management::FileManagement};

use crate::command::BankCommand;
use crate::ui::view_bank;

pub fn handle_bank_command(command: BankCommand, paths: &DataPaths) -> Result<()> {
    let codexi = FileManagement::load_current_state(paths)?;
    match command {
        BankCommand::List => {
            let items = codexi.banks.bank_entry();
            view_bank(&items);
        }
    }
    Ok(())
}
