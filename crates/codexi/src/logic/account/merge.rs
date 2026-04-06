// src/logic/account/merge.rs

use nulid::Nulid;
use std::collections::HashMap;

use crate::core::{CoreWarning, CoreWarningKind};
use crate::exchange::ImportSummary;
use crate::logic::account::{Account, AccountError};
use crate::logic::operation::{AccountOperations, Operation, OperationKind, RegularKind, SystemKind};

impl Account {
    /// Merges an imported account header into self, respecting update rules.
    /// Validation is assumed to have been done upstream by validate_import().
    pub fn merge_account_header_from_import(&mut self, imported: Account) -> Result<ImportSummary, AccountError> {
        // Terminated accounts are immutable — reject any update
        self.is_terminated()?;

        // --- Account level update ---
        let mut summary = ImportSummary::default();

        self.update(
            &imported.name,
            imported.currency_id,
            imported.bank_id,
            imported.context,
            imported.meta,
        );

        summary.name = self.name.clone();
        summary.updated = 1;
        summary.created = 0;
        summary.total_processed = 1;

        Ok(summary)
    }

    /// Merges operation account into self, respecting update rules.
    /// Validation is assumed to have been done upstream by validate_import().
    pub fn merge_operation_from_import(
        &mut self,
        imported: &AccountOperations,
    ) -> Result<(ImportSummary, Vec<CoreWarning>), AccountError> {
        // Terminated accounts are immutable — reject any update
        self.is_terminated()?;

        let mut summary = ImportSummary::default();
        let mut warnings = Vec::new();
        let mut to_add = Vec::new();
        let mut to_update = Vec::new();

        {
            let existing_by_id: HashMap<Nulid, &Operation> = self.operations.iter().map(|op| (op.id, op)).collect();

            for op in &imported.operations {
                summary.total_processed += 1;
                if existing_by_id.contains_key(&op.id) {
                    to_update.push(op);
                } else {
                    to_add.push(op);
                }
            }
        }

        for op_update in to_update {
            if let Some(op) = self.get_operation_by_id_mut(op_update.id) {
                op.update(&op_update.description, &op_update.context, &op_update.meta);
                summary.updated += 1;
            }
        }

        for op_add in to_add {
            match op_add.kind {
                OperationKind::Regular(RegularKind::Transaction) => {
                    self.register_transaction(
                        op_add.date,
                        op_add.kind,
                        op_add.flow,
                        op_add.amount,
                        op_add.description.clone(),
                        op_add.context.counterparty_id,
                        op_add.context.category_id,
                    )?;
                    summary.created += 1;
                }
                OperationKind::System(SystemKind::Init) => {
                    let amount = op_add.amount * op_add.flow.to_sign();
                    self.initialize(op_add.date, amount)?;
                    summary.created += 1;
                }
                _ => {
                    // Kind not supported for import yet (Void, Transfer, Adjust...)
                    // Skipped intentionally — can be extended later
                    warnings.push(CoreWarning {
                        kind: CoreWarningKind::InvalidData,
                        message: format!("Operation kind '{}' not supported for import — skipped", op_add.kind),
                    });
                }
            }
        }

        summary.name = self.name.clone();
        Ok((summary, warnings))
    }
}
