// src/logic/account/merge.rs

use nulid::Nulid;
use std::collections::HashMap;

use crate::exchange::ImportSummary;
use crate::logic::{
    account::{Account, AccountError},
    operation::Operation,
};

impl Account {
    /// Merges an imported account into self, respecting update rules.
    /// Validation is assumed to have been done upstream by validate_import().
    pub fn merge_from_import(&mut self, imported: Account) -> Result<ImportSummary, AccountError> {
        // --- Account level merge ---
        // Only non-financial fields are updated — id, amounts, dates are immutable
        self.name = imported.name;
        self.meta = imported.meta;

        // --- Operations level merge ---
        let mut summary = self.merge_operations(imported.operations)?;
        summary.account_name = self.name.clone();

        Ok(summary)
    }

    fn merge_operations(
        &mut self,
        imported_ops: Vec<Operation>,
    ) -> Result<ImportSummary, AccountError> {
        let mut summary = ImportSummary::default();
        // Temporary vector for new operations
        let mut to_add = Vec::new();

        {
            let mut existing_ops: HashMap<Nulid, &mut Operation> =
                self.operations.iter_mut().map(|op| (op.id, op)).collect();

            for new_op in imported_ops {
                summary.total_processed += 1;
                if let Some(existing) = existing_ops.get_mut(&new_op.id) {
                    // Update only non-financial fields — amount, date, kind, flow are immutable
                    existing.description = new_op.description;
                    existing.context = new_op.context;
                    existing.meta = new_op.meta;
                    summary.updated += 1;
                } else {
                    // New operation — defer to avoid borrow conflict
                    to_add.push(new_op);
                }
            }
        }

        // Insert new operations directly via commit_operation.
        // Validation (amounts, links, temporal) has already been performed
        // upstream by validate_import() — no need to re-validate here.
        for op in to_add {
            self.commit_operation(op);
            summary.created += 1;
        }

        Ok(summary)
    }
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::{
        account::{AccountAnchors, AccountContext, AccountMeta, AccountType},
        operation::{
            OperationContext, OperationFlow, OperationKind, OperationLinks, OperationMeta,
            RegularKind,
        },
    };
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    // Helper pour créer un compte de test
    fn setup_test_account() -> Account {
        Account {
            id: Nulid::new().unwrap(),
            name: "Test Account".to_string(),
            context: AccountContext::from_type(AccountType::Current),
            bank_id: None,
            currency_id: None,
            carry_forward_balance: dec!(1000),
            open_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            terminated_date: None,
            operations: Vec::new(),
            current_balance: dec!(0),
            checkpoints: Vec::new(),
            anchors: AccountAnchors::default(),
            meta: AccountMeta::default(),
        }
    }

    #[test]
    fn merge_inserts_new_operation_via_commit() {
        let mut account = setup_test_account();
        account
            .initialize(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), dec!(500))
            .unwrap();

        // New operation — inserted directly via commit_operation, no policy check
        let new_op = Operation {
            id: Nulid::new().unwrap(),
            date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
            kind: OperationKind::Regular(RegularKind::Transaction),
            flow: OperationFlow::Debit,
            amount: dec!(50),
            description: "Imported op".into(),
            balance: dec!(0),
            links: OperationLinks::default(),
            context: OperationContext::default(),
            meta: OperationMeta::default(),
        };

        let result = account.merge_operations(vec![new_op]);
        assert!(result.is_ok());
        assert_eq!(account.operations.len(), 2); // init + new op
    }

    #[test]
    fn merge_updates_existing_op_without_changing_financial_fields() {
        let mut account = setup_test_account();

        // Init required before any Regular operation
        account
            .initialize(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), dec!(500))
            .unwrap();

        let op_id = account
            .register_transaction(
                NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Credit,
                dec!(100),
                "Original Desc".into(),
            )
            .unwrap();

        // Same id — only description should update, amount/date/kind/flow are immutable
        let updated_op = Operation {
            id: op_id,
            date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
            kind: OperationKind::Regular(RegularKind::Transaction),
            flow: OperationFlow::Credit,
            amount: dec!(9999), // attempt to change amount — must be ignored
            description: "Updated via Import".into(),
            balance: dec!(0),
            links: OperationLinks::default(),
            context: OperationContext::default(),
            meta: OperationMeta::default(),
        };

        account.merge_operations(vec![updated_op]).unwrap();

        let final_op = account.get_operation_by_id(op_id).unwrap();
        assert_eq!(final_op.description, "Updated via Import"); // updated
        assert_eq!(final_op.amount, dec!(100)); // immutable
        assert_eq!(account.operations.len(), 2); // init + transaction
    }
}
