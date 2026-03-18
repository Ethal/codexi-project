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
    pub fn merge_from_import(&mut self, imported: Account) -> Result<ImportSummary, AccountError> {
        // --- ACCOUNT LEVEL MERGE ---
        // allow field to be updated
        self.name = imported.name;
        self.meta = imported.meta;

        // --- OPERATIONS LEVEL MERGE ---
        let mut summary = self.merge_operations(imported.operations)?;
        summary.account_name = self.name.clone();

        Ok(summary)
    }

    fn merge_operations(
        &mut self,
        imported_ops: Vec<Operation>,
    ) -> Result<ImportSummary, AccountError> {
        let mut summary = ImportSummary::default();
        // temporary vector for new operation
        let mut to_process_as_new = Vec::new();

        {
            let mut existing_ops: HashMap<Nulid, &mut Operation> =
                self.operations.iter_mut().map(|op| (op.id, op)).collect();

            for new_op in imported_ops {
                summary.total_processed += 1;
                if let Some(existing) = existing_ops.get_mut(&new_op.id) {
                    // update field
                    existing.description = new_op.description;
                    existing.context = new_op.context;
                    existing.meta = new_op.meta;
                    summary.updated += 1;
                } else {
                    // new operation : add to temporary vector
                    to_process_as_new.push(new_op);
                }
            }
        }

        // add new operations
        for op in to_process_as_new {
            self.register_transaction(op.date, op.kind, op.flow, op.amount, op.description)?;
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
            SystemKind,
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
    fn test_import_rejects_negative_amount() {
        let mut account = setup_test_account();

        // On simule une opération importée avec un montant invalide
        let invalid_op = Operation {
            id: Nulid::new().unwrap(),
            date: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
            kind: OperationKind::System(SystemKind::Adjust),
            flow: OperationFlow::Credit,
            amount: dec!(-50), // Montant négatif -> DOIT ÉCHOUER
            description: "Invalid Import".into(),
            balance: dec!(0),
            links: OperationLinks::default(),
            context: OperationContext::default(),
            meta: OperationMeta::default(),
        };

        let import_data = vec![invalid_op];

        // L'appel au merge doit renvoyer une erreur AccountError::InvalidData
        let result = account.merge_operations(import_data);
        assert!(result.is_err());
        assert!(account.operations.is_empty()); // Rien n'a été ajouté
    }

    #[test]
    fn test_import_respects_temporal_policy() {
        let mut account = setup_test_account();

        // Supposons que votre temporal_policy interdise les opérations
        // dans le futur (ex: après aujourd'hui)
        let future_date = NaiveDate::from_ymd_opt(2099, 1, 1).unwrap();

        let future_op = Operation {
            id: Nulid::new().unwrap(),
            amount: dec!(100),
            date: future_date,
            kind: OperationKind::System(SystemKind::Adjust),
            flow: OperationFlow::Credit,
            description: "Future Op".into(),
            balance: dec!(0),
            links: OperationLinks::default(),
            context: OperationContext::default(),
            meta: OperationMeta::default(),
        };

        let result = account.merge_operations(vec![future_op]);

        // Si votre temporal_policy fonctionne, le merge doit échouer ici
        assert!(
            result.is_err(),
            "L'import devrait échouer car la date est hors politique financière"
        );
    }

    #[test]
    fn test_merge_updates_existing_op_without_changing_id() {
        let mut account = setup_test_account();

        // 1. Ajouter une opération propre
        let op_id = account
            .register_transaction(
                NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
                OperationKind::System(SystemKind::Init),
                OperationFlow::Credit,
                dec!(100),
                "Original Desc".into(),
            )
            .unwrap();

        // 2. Simuler un import avec le MÊME ID mais une description modifiée
        let mut updated_op = Operation {
            id: Nulid::new().unwrap(),
            amount: dec!(100),
            date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
            kind: OperationKind::System(SystemKind::Adjust),
            flow: OperationFlow::Credit,
            description: "Future Op".into(),
            balance: dec!(0),
            links: OperationLinks::default(),
            context: OperationContext::default(),
            meta: OperationMeta::default(),
        };

        updated_op.id = op_id;
        updated_op.description = "Updated via Import".into();
        updated_op.amount = dec!(9999); // On tente de tricher sur le montant
        updated_op.date = NaiveDate::from_ymd_opt(2026, 1, 10).unwrap();

        account.merge_operations(vec![updated_op]).unwrap();

        // 3. Vérifications
        let final_op = &account.operations[0];
        assert_eq!(final_op.description, "Updated via Import");
        assert_eq!(final_op.amount, dec!(100)); // LE MONTANT N'A PAS CHANGÉ (Sécurité merge)
        assert_eq!(account.operations.len(), 1); // Pas de doublon
    }
}
