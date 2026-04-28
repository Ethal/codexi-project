// src/logic/account/audit.rs

use nulid::Nulid;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

use crate::{
    core::{CoreWarning, CoreWarningKind},
    logic::{
        account::{Account, AccountAnchors, AccountError, AccountMeta, ComplianceAction, TemporalAction},
        codexi::{Codexi, CodexiError},
        operation::{Operation, OperationFlow},
    },
};

impl Codexi {
    pub fn main_audit(&self) -> Result<(Vec<CoreWarning>, String), CodexiError> {
        let account = self.get_current_account()?;
        let name = account.name.clone();

        let warnings = self.audit(account)?;

        Ok((warnings, name))
    }

    /// Perfomed rebuild of balance and, account_id
    pub fn rebuild(&mut self) -> Result<(), CodexiError> {
        let account = self.get_current_account_mut()?;
        // rebuild balance
        account.rebuild_balances_from(account.operations.first().map(|op| op.date).unwrap_or_default());
        // rebuild account_id
        for op in &mut account.operations {
            if op.is_legacy_account() {
                op.account_id = account.id;
            }
        }

        Ok(())
    }
    /// Audit the account
    /// TEST 1 — policy via replay /
    /// TEST 2 — locked period
    /// TEST 3 — balance by opération
    /// TEST 4 — current balance
    /// TEST 5 — Symmetry of void/void_of
    /// TEST 6 — double void
    /// TEST 7 — anchors consistency
    /// TEST 8 — transfer links
    /// TEST 9 — kegacy_account (operation with account_id.is_nil())
    /// TEST 10 — Counter party exist
    pub fn audit(&self, account: &Account) -> Result<Vec<CoreWarning>, AccountError> {
        let mut warnings = Vec::new();

        self.audit_void_links(account, &mut warnings); // TEST 5
        self.audit_double_void(account, &mut warnings); // TEST 6
        self.audit_transfer_links(account, &mut warnings); // TEST 8

        // Remembering the FINAL closing anchor of the file
        let final_checkpoint = account.anchors.last_checkpoint.clone();
        // Create the shadow account
        let mut shadow_account = Self::clone_empty(account);

        let mut first_op = true;
        let mut running_balance = Decimal::ZERO;

        // Replay
        for op in &account.operations {
            if first_op {
                first_op = false;
                // No temporal/compliance policy check on the fist op — the shadow start from zero
                shadow_account.commit_operation(op.clone());
                running_balance = match op.flow {
                    OperationFlow::Credit => running_balance + op.amount,
                    OperationFlow::Debit => running_balance - op.amount,
                    OperationFlow::None => running_balance,
                };
                continue;
            }

            // TEST 1 : policy on all operation except the first
            shadow_account.temporal_policy(TemporalAction::Create(op.kind), op.date)?;
            shadow_account.compliance_policy(ComplianceAction::Create(op.kind, op.flow, op.amount), op.date)?;

            // TEST 2 : locked period
            // If the operation is a regular transaction, it is not allowed
            // to exist on a date BEFORE or EQUALLY equal to the last checkpoint of the file.
            if op.kind.is_regular()
                && let Some(final_chk_date) = final_checkpoint.clone()
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
            // TEST 9 : Operation with legacy account id or account.id != op.account_id
            if op.is_legacy_account() || op.account_id != account.id {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!("TEST 9: Operation {} missing account_id (legacy data)", op.id),
                });
            }
            // TEST 10 : chek counterparty and category
            if let Some(counterparty_id) = op.context.counterparty_id
                && !self.counterparties.is_exist(&counterparty_id)
            {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!(
                        "TEST 10: Operation {} has invalid counterparty_id {}",
                        op.id, counterparty_id
                    ),
                });
            }
            if let Some(category_id) = op.context.category_id
                && !self.categories.is_exist(&category_id)
            {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!("TEST 10: Operation {} has invalid category_id {}", op.id, category_id),
                });
            }
        }

        // TEST 4 : current balance of the account
        if (account.current_balance - running_balance).abs() > dec!(0.001) {
            warnings.push(CoreWarning {
                kind: CoreWarningKind::InvalidData,
                message: format!(
                    "TEST 4: Account current_balance mismatch: stored {}, calculated {}",
                    account.current_balance, running_balance
                ),
            });
        }
        // TEST 7
        self.audit_anchors(account, &shadow_account, &mut warnings);

        Ok(warnings)
    }
    /// Creates a copy of the account with the same metadata (ID, name, bank, etc.)
    /// but resets all history and calculation anchors.
    fn clone_empty(account: &Account) -> Account {
        Account {
            id: account.id,
            name: account.name.clone(),
            context: account.context.clone(),
            bank_id: account.bank_id,
            currency_id: account.currency_id,
            carry_forward_balance: Decimal::ZERO, // Sera défini par le premier Init rejoué
            open_date: account.open_date,         // On garde la date d'ouverture originale
            operations: Vec::new(),               // Historique vide
            terminated_date: None,                // État ouvert par défaut pour le replay
            // --- Reset des ancres de calcul and current balance---
            current_balance: Decimal::ZERO,
            checkpoints: account.checkpoints.clone(),
            anchors: AccountAnchors::default(),
            meta: AccountMeta::default(),
        }
    }
    /// Audit of void links
    fn audit_void_links(&self, account: &Account, warnings: &mut Vec<CoreWarning>) {
        let op_index: HashMap<Nulid, &Operation> = account.operations.iter().map(|op| (op.id, op)).collect();

        for op in &account.operations {
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
    fn audit_double_void(&self, account: &Account, warnings: &mut Vec<CoreWarning>) {
        let mut void_targets: HashMap<Nulid, Nulid> = HashMap::new();
        for op in &account.operations {
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
    fn audit_anchors(&self, account: &Account, shadow: &Account, warnings: &mut Vec<CoreWarning>) {
        let checks = [
            ("last_init", &shadow.anchors.last_init, &account.anchors.last_init),
            (
                "last_checkpoint",
                &shadow.anchors.last_checkpoint,
                &account.anchors.last_checkpoint,
            ),
            ("last_adjust", &shadow.anchors.last_adjust, &account.anchors.last_adjust),
            ("last_void", &shadow.anchors.last_void, &account.anchors.last_void),
            (
                "last_regular",
                &shadow.anchors.last_regular,
                &account.anchors.last_regular,
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

    /// Audit of transfer links — internal consistency only.
    /// Checks that every operation with a transfer_id also has
    /// a transfer_account_id set, and vice versa.
    /// Cross-account link validity (twin exists in the other account)
    /// cannot be checked here — handled at Codexi level.
    fn audit_transfer_links(&self, account: &Account, warnings: &mut Vec<CoreWarning>) {
        for op in &account.operations {
            let has_transfer_id = op.links.transfer_id.is_some();
            let has_transfer_acc = op.links.transfer_account_id.is_some();

            // Both fields must be set together — one without the other is a broken link
            match (has_transfer_id, has_transfer_acc) {
                (true, false) => warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!("TEST 8: Operation {} has transfer_id but no transfer_account_id", op.id),
                }),
                (false, true) => warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!("TEST 8: Operation {} has transfer_account_id but no transfer_id", op.id),
                }),
                _ => {}
            }

            // A transfer operation must use Regular::Transfer kind
            if has_transfer_id
                && !matches!(
                    op.kind,
                    crate::logic::operation::OperationKind::Regular(crate::logic::operation::RegularKind::Transfer)
                )
            {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::InvalidData,
                    message: format!(
                        "TEST 8: Operation {} has transfer_id but kind is not Regular::Transfer",
                        op.id
                    ),
                });
            }
        }
    }
}
