// src/handler/operation.rs

use anyhow::Result;

use codexi::{
    core::DataPaths,
    file_management::FileManagement,
    logic::{account::AccountError, operation::Operation, utils::resolve_id},
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
            let current_account = codexi.get_current_account()?;
            let op_id = resolve_id::<Operation, AccountError>(&id, &current_account.operations)?;

            if let Some(detail) = codexi.operation_detail(&op_id) {
                if raw {
                    view_operation_raw(&detail);
                } else {
                    view_operation(&detail);
                }
            } else {
                msg_warn!("No operation with {}", id);
            }
        }
    }
    Ok(())
}
