// src/handler/operation.rs

use anyhow::Result;

use codexi::{
    core::DataPaths,
    dto::SearchOperationItem,
    file_management::FileManagement,
    logic::{
        search::{SearchError, SearchOperation, SearchParamsBuilder, search},
        utils::resolve_id,
    },
};

use crate::{
    command::OperationCommand,
    msg_warn,
    ui::{view_operation, view_operation_raw},
};

pub fn handle_operation_command(command: OperationCommand, paths: &DataPaths) -> Result<()> {
    let codexi = FileManagement::load_current_state(paths)?;
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
    }
    Ok(())
}
