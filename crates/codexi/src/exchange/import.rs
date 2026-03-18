// src/exchange/import.rs

use crate::core::CoreWarning;
use crate::exchange::{
    ExchangeCheckpointRef, ExchangeData, ExchangeError, ExchangeOperation, validate_import,
};
use crate::logic::account::{Account, AccountMeta, CheckpointRef};
use crate::logic::operation::Operation;

impl ExchangeData {
    /// Single entry point for importing a account (JSON / TOML / CSV)
    pub fn import_data(data: &ExchangeData) -> Result<(Account, Vec<CoreWarning>), ExchangeError> {
        let warnings = validate_import(data)?;
        let account = Self::build_from_export(data);
        Ok((account, warnings))
    }

    /// internal build after validation
    fn build_from_export(import: &ExchangeData) -> Account {
        let operations: Vec<Operation> = import
            .operations
            .iter()
            .cloned()
            .map(Self::map_operation)
            .collect();

        let checkpoints: Vec<CheckpointRef> = import
            .checkpoints
            .iter()
            .cloned()
            .map(Self::map_checkpoint)
            .collect();

        Account {
            id: import.id,
            name: import.name.clone(),
            context: import.context.clone(),
            bank_id: import.bank_id,                             // Bank Id
            currency_id: import.currency_id,                     // Currency id for the account
            carry_forward_balance: import.carry_forward_balance, // for internal calculation
            open_date: import.open_date, // Open date of the account,typivcaly the date of the init.
            terminated_date: import.terminated_date, // Close date of the account.
            operations,
            current_balance: import.current_balance,
            checkpoints,
            anchors: import.anchors.clone(),
            meta: AccountMeta::default(),
        }
    }

    /// Mapping strict Export → Doamin (without alteration)
    fn map_operation(op: ExchangeOperation) -> Operation {
        Operation {
            id: op.id,
            date: op.date,
            kind: op.kind,
            flow: op.flow,
            amount: op.amount,
            description: op.description.clone(),

            balance: op.balance,

            links: op.links,
            context: op.context,
            meta: op.meta,
        }
    }
    /// Mapping strict Export → Doamin (without alteration)
    fn map_checkpoint(ck: ExchangeCheckpointRef) -> CheckpointRef {
        CheckpointRef {
            checkpoint_date: ck.checkpoint_date,
            checkpoint_balance: ck.checkpoint_balance,
            archive_file: ck.archive_file,
        }
    }
}
