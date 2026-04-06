// src/handler/counterparty.rs

use anyhow::Result;

use codexi::{
    core::DataPaths,
    dto::CounterpartyCollection,
    file_management::FileManagement,
    logic::{
        counterparty::{Counterparty, CounterpartyError, CounterpartyKind},
        utils::resolve_by_id_or_name,
    },
};

use crate::{command::CounterpartyCommand, ui::view_counterparty};

pub fn handle_counterparty_command(command: CounterpartyCommand, paths: &DataPaths) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    match command {
        CounterpartyCommand::List => {
            let items = CounterpartyCollection::build(&codexi.counterparties);
            view_counterparty(&items);
        }
        CounterpartyCommand::Add { name, kind, note } => {
            let name_n = name.join(" ");
            let kind_n = CounterpartyKind::try_from(kind.as_str())?;
            let note = note.map(|n| n.join(" "));
            let note_n = note.as_deref();
            codexi.counterparties.create(&name_n, kind_n, note_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
        }
        CounterpartyCommand::Terminate { id } => {
            let id_n = resolve_by_id_or_name::<Counterparty, CounterpartyError>(&id, &codexi.counterparties.list)?;
            codexi.counterparties.terminate(&id_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
        }
    }
    Ok(())
}
