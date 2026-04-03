// src/handler/counterparty.rs

use anyhow::Result;

use codexi::{core::DataPaths, dto::CounterpartyCollection, file_management::FileManagement};

use crate::{command::CounterpartyCommand, ui::view_counterparty};

pub fn handle_counterparty_command(command: CounterpartyCommand, paths: &DataPaths) -> Result<()> {
    let codexi = FileManagement::load_current_state(paths)?;
    match command {
        CounterpartyCommand::List => {
            let items = CounterpartyCollection::build(&codexi.counterparties);
            view_counterparty(&items);
        }
    }
    Ok(())
}
