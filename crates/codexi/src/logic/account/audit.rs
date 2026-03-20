// src/logic/account/audit.rs

use nulid::Nulid;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

use crate::{
    core::{CoreWarning, CoreWarningKind},
    logic::{
        account::{
            Account, AccountAnchors, AccountError, AccountMeta, ComplianceAction, TemporalAction,
        },
        operation::{Operation, OperationFlow},
    },
};

impl Account {
    /// Creates a copy of the account with the same metadata (ID, name, bank, etc.)
    /// but resets all history and calculation anchors.
    pub(crate) fn clone_empty(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            context: self.context.clone(),
            bank_id: self.bank_id,
            currency_id: self.currency_id,
            carry_forward_balance: Decimal::ZERO, // Sera défini par le premier Init rejoué
            open_date: self.open_date,            // On garde la date d'ouverture originale
            operations: Vec::new(),               // Historique vide
            terminated_date: None,                // État ouvert par défaut pour le replay
            // --- Reset des ancres de calcul and current balance---
            current_balance: Decimal::ZERO,
            checkpoints: self.checkpoints.clone(),
            anchors: AccountAnchors::default(),
            meta: AccountMeta::default(),
        }
    }
    /// Audit the account
    /// TEST 1 — policy via replay /
    /// TEST 2 — locked period
    /// TEST 3 — balance by opération
    /// TEST 4 — current balance
    /// TEST 5 — Symmetry of void/void_of
    /// TEST 6 — double void
    /// TEST 7 — anchors consistency
    pub fn audit(&self) -> Result<Vec<CoreWarning>, AccountError> {
        let mut warnings = Vec::new();

        self.audit_void_links(&mut warnings); // TEST 5
        self.audit_double_void(&mut warnings); // TEST 6

        // Remembering the FINAL closing anchor of the file
        let final_checkpoint = &self.anchors.last_checkpoint;
        // Create the shadow account
        let mut shadow_account = self.clone_empty();

        let mut first_op = true;
        let mut running_balance = Decimal::ZERO;

        // Replay
        for op in &self.operations {
            if first_op {
                first_op = false;
                // No temporal policy chek on the fist op — le shadow start from zero
                shadow_account.commit_operation(op.clone());
                running_balance = match op.flow {
                    OperationFlow::Credit => running_balance + op.amount,
                    OperationFlow::Debit => running_balance - op.amount,
                    OperationFlow::None => running_balance,
                };
                continue;
            }

            // TEST 1 : policy on all operation except the first
            shadow_account.temporal_policy(TemporalAction::Create(&op.kind), op.date)?;
            shadow_account.compliance_policy(
                ComplianceAction::Create(&op.kind, op.flow, op.amount),
                op.date,
            )?;

            // TEST 2 : locked period
            // If the operation is a normal transaction, it is not allowed
            // to exist on a date BEFORE or EQUALLY equal to the last checkpoint of the file.
            if op.kind.is_regular()
                && let Some(final_chk_date) = final_checkpoint
                && op.date <= final_chk_date.date
            {
                return Err(AccountError::InvalidData(format!(
                    " TEST 2: Audit Failure: Operation {} (date {}) found in a locked period (Checkpoint at {})",
                    op.id, op.date, final_chk_date.date
                )));
            }

            // if Test 1 and 2 push the operation in shadow_account, commit_operation update the anchors
            shadow_account.commit_operation(op.clone());

            // live calulation of the current balance
            running_balance = match op.flow {
                OperationFlow::Credit => running_balance + op.amount,
                OperationFlow::Debit => running_balance - op.amount,
                OperationFlow::None => running_balance,
            };

            // TEST 3 : crossref balance operation vs live calculation
            if (op.balance - running_balance).abs() > dec!(0.001) {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!(
                        "Balance mismatch on operation {} (date {}): stored {}, calculated {}",
                        op.id, op.date, op.balance, running_balance
                    ),
                });
            }
        }

        // TEST 4 : current balance of the account
        if (self.current_balance - running_balance).abs() > dec!(0.001) {
            warnings.push(CoreWarning {
                kind: CoreWarningKind::InvalidData,
                message: format!(
                    "TEST 4: Account current_balance mismatch: stored {}, calculated {}",
                    self.current_balance, running_balance
                ),
            });
        }
        // TEST 7
        self.audit_anchors(&shadow_account, &mut warnings);

        Ok(warnings)
    }

    /// Audit of void links
    fn audit_void_links(&self, warnings: &mut Vec<CoreWarning>) {
        let op_index: HashMap<Nulid, &Operation> =
            self.operations.iter().map(|op| (op.id, op)).collect();

        for op in &self.operations {
            if let Some(void_by_id) = op.links.void_by {
                match op_index.get(&void_by_id) {
                    None => warnings.push(CoreWarning {
                        kind: CoreWarningKind::InvalidData,
                        message: format!(
                            "TEST 5: Operation {} has void_by {} but that operation does not exist",
                            op.id, void_by_id
                        ),
                    }),
                    Some(void_op) => {
                        if void_op.links.void_of != Some(op.id) {
                            warnings.push(CoreWarning {
                                    kind: CoreWarningKind::InvalidData,
                                    message: format!(
                                        "TEST 5: Broken void link: op {} void_by {} but {} does not point back",
                                        op.id, void_by_id, void_by_id
                                    ),
                                });
                        }
                    }
                }
            }
            if let Some(void_of_id) = op.links.void_of {
                match op_index.get(&void_of_id) {
                    None => warnings.push(CoreWarning {
                        kind: CoreWarningKind::InvalidData,
                        message: format!(
                            "TEST 5: Operation {} has void_of {} but that operation does not exist",
                            op.id, void_of_id
                        ),
                    }),
                    Some(target_op) => {
                        if target_op.links.void_by != Some(op.id) {
                            warnings.push(CoreWarning {
                                    kind: CoreWarningKind::InvalidData,
                                    message: format!(
                                        "TEST 5: Broken void link: op {} void_of {} but {} does not point back",
                                        op.id, void_of_id, void_of_id
                                    ),
                                });
                        }
                    }
                }
            }
        }
    }
    /// audit of the double void
    fn audit_double_void(&self, warnings: &mut Vec<CoreWarning>) {
        let mut void_targets: HashMap<Nulid, Nulid> = HashMap::new();
        for op in &self.operations {
            if let Some(void_of_id) = op.links.void_of
                && let Some(existing) = void_targets.insert(void_of_id, op.id)
            {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!(
                        "TEST 6: Operation {} is voided twice: by {} and by {}",
                        void_of_id, existing, op.id
                    ),
                });
            }
        }
    }
    /// Audit of the anchors
    fn audit_anchors(&self, shadow: &Account, warnings: &mut Vec<CoreWarning>) {
        let checks = [
            (
                "last_init",
                &shadow.anchors.last_init,
                &self.anchors.last_init,
            ),
            (
                "last_checkpoint",
                &shadow.anchors.last_checkpoint,
                &self.anchors.last_checkpoint,
            ),
            (
                "last_adjust",
                &shadow.anchors.last_adjust,
                &self.anchors.last_adjust,
            ),
            (
                "last_void",
                &shadow.anchors.last_void,
                &self.anchors.last_void,
            ),
            (
                "last_regular",
                &shadow.anchors.last_regular,
                &self.anchors.last_regular,
            ),
        ];
        for (name, calculated, stored) in checks {
            if calculated != stored {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!(
                        "TEST 7: Anchor mismatch on {}: stored {:?}, calculated {:?}",
                        name, stored, calculated
                    ),
                });
            }
        }
    }

    /// Perfomed and audit and a balance rebuild
    pub fn audit_and_rebuild(&mut self) -> Result<Vec<CoreWarning>, AccountError> {
        let warnings = self.audit()?; // si Err → blocking, NO rebuild

        let has_only_balance_warnings = warnings
            .iter()
            .all(|w| matches!(w.kind, CoreWarningKind::InvalidData));

        if !warnings.is_empty() && has_only_balance_warnings {
            self.rebuild_balances_from(
                self.operations
                    .first()
                    .map(|op| op.date)
                    .unwrap_or_default(),
            );
        }

        Ok(warnings)
    }
}
