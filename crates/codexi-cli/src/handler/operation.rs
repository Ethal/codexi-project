// src/handler/operation.rs

use anyhow::Result;

use codexi::{
    core::DataPaths,
    dto::SearchOperationItem,
    file_management::FileManagement,
    logic::{
        category::{Category, CategoryError},
        counterparty::{Counterparty, CounterpartyError},
        operation::{Operation, OperationError},
        search::{SearchError, SearchOperation, SearchParamsBuilder, search},
        utils::{resolve_by_id_or_name, resolve_id},
    },
};

use crate::{
    command::OperationCommand,
    msg_info, msg_warn,
    ui::{view_operation, view_operation_raw},
};

pub fn handle_operation_command(command: OperationCommand, paths: &DataPaths) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    match command {
        OperationCommand::View { id, raw } => {
            let account = codexi.get_current_account()?;

            let params = SearchParamsBuilder::default().build()?;
            let s_ops = search(account, &params)?;
            let op_id = resolve_id::<SearchOperation, SearchError>(&id, &s_ops.items)?;

            if let Some(op) = s_ops.get_search_operation_by_id(op_id) {
                let s_op = SearchOperationItem::build(&codexi, account, op);
                if raw {
                    view_operation_raw(&s_op);
                } else {
                    view_operation(&s_op);
                }
            } else {
                msg_warn!("No operation with {}", id);
            }
        }
        OperationCommand::Update {
            id,
            description,
            counterparty,
            category,
        } => {
            let account = codexi.get_current_account()?;
            let op_id = resolve_id::<Operation, OperationError>(&id, &account.operations)?;
            let counterparty_id = counterparty
                .map(|name| {
                    resolve_by_id_or_name::<Counterparty, CounterpartyError>(&name, &codexi.counterparties.list)
                })
                .transpose()?;
            let category_id = category
                .map(|name| resolve_by_id_or_name::<Category, CategoryError>(&name, &codexi.categories.list))
                .transpose()?;

            codexi.update_operation(op_id, description.as_deref(), counterparty_id, category_id)?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("update done.");
        }
    }
    Ok(())
}
