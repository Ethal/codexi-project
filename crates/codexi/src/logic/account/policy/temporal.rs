// src/logic/account/policy/temporal.rs

use chrono::{Local, NaiveDate};
use nulid::Nulid;

use crate::logic::{
    account::Account,
    account::policy::TemporalViolation,
    operation::{OperationKind, SystemKind},
};

#[derive(Debug)]
pub enum TemporalAction<'a> {
    Create(&'a OperationKind), // Create a new operation (Regular, Adjust, Close...)
    Void(Nulid),               // Void(op_id)
}

impl Account {
    /// Check thet operations is not empty.
    fn operations_empty(&self, kind: &OperationKind) -> Result<(), TemporalViolation> {
        if self.operations.is_empty() {
            Err(TemporalViolation::HaveNoOperation(format!(
                "{}  count op: {} ",
                kind.as_str(),
                self.operations.len()
            )))
        } else {
            Ok(())
        }
    }
    /// Check thet operations is empty.
    fn operations_not_empty(&self) -> Result<(), TemporalViolation> {
        if !self.operations.is_empty() {
            Err(TemporalViolation::HaveOperation)
        } else {
            Ok(())
        }
    }
    /// Return an error if only init operation
    fn has_only_init(&self) -> Result<(), TemporalViolation> {
        let has_only_init = self.operations.len() == 1
            && self
                .operations
                .iter()
                .all(|op| matches!(op.kind, OperationKind::System(SystemKind::Init)));
        if has_only_init {
            return Err(TemporalViolation::OnlyInit);
        }
        Ok(())
    }
    /// Return an error in the account is close(terminated)
    fn is_terminated(&self) -> Result<(), TemporalViolation> {
        if self.terminated_date.is_some() {
            return Err(TemporalViolation::AccountClose);
        }
        Ok(())
    }
    /// Check the temporal policy, if not reach -> Error
    pub fn temporal_policy(
        &self,
        action: TemporalAction,
        date: NaiveDate,
    ) -> Result<(), TemporalViolation> {
        let today = Local::now().date_naive();

        // Check account is close, no opaeration is allowed
        self.is_terminated()?;
        // Extraction of ancors
        // Plus de scans ! On utilise le cache
        let last_init = self.anchors.last_init;
        let last_chk = self.anchors.last_checkpoint;
        let last_adj = self.anchors.last_adjust;

        match action {
            TemporalAction::Create(kind) => {
                match kind {
                    // --- INITIALIZATION ---
                    OperationKind::System(SystemKind::Init) => {
                        self.operations_not_empty()?;
                        if date > today {
                            return Err(TemporalViolation::InvalidData(
                                "Init date cannot be in the future".into(),
                            ));
                        }
                    }

                    // --- CLOSING (Close) ---
                    OperationKind::System(SystemKind::Checkpoint) => {
                        self.operations_empty(kind)?;
                        if date > today {
                            return Err(TemporalViolation::InvalidData(
                                "Closing date cannot be in the future".into(),
                            ));
                        }
                        if let Some(cld_dt) = last_chk
                            && date <= cld_dt
                        {
                            return Err(TemporalViolation::InvalidData(format!(
                                "Must be > last closing ({})",
                                cld_dt
                            )));
                        }
                        if let Some(init_dt) = last_init
                            && date < init_dt
                        {
                            return Err(TemporalViolation::InvalidData(
                                "Close cannot be before Init".into(),
                            ));
                        }
                        self.has_only_init()?;
                    }

                    // --- AJUSTEMENT (Adjust) ---
                    OperationKind::System(SystemKind::Adjust) => {
                        self.operations_empty(kind)?;
                        if date > today {
                            return Err(TemporalViolation::InvalidData(
                                "Adjustment date cannot be in the future".into(),
                            ));
                        }
                        if let Some(cld_dt) = last_chk
                            && date <= cld_dt
                        {
                            return Err(TemporalViolation::InvalidData("Period closed".into()));
                        }
                        if let Some(adj_dt) = last_adj
                            && date < adj_dt
                        {
                            return Err(TemporalViolation::InvalidData(format!(
                                "Adjustment date must be >= {}",
                                adj_dt
                            )));
                        }
                    }

                    // --- REGULAR OPERATION ---
                    _ => {
                        self.operations_empty(kind)?;
                        if let Some(cld_dt) = last_chk
                            && date <= cld_dt
                        {
                            return Err(TemporalViolation::InvalidData("Period closed".into()));
                        }
                        let anchor = last_adj.or(last_init);
                        if let Some(a_dt) = anchor
                            && date < a_dt
                        {
                            return Err(TemporalViolation::InvalidData(format!(
                                "Must be >= {}",
                                a_dt
                            )));
                        }
                    }
                }
            }

            TemporalAction::Void(target_id) => {
                self.operations_empty(&OperationKind::System(SystemKind::Void))?;

                if date < today {
                    return Err(TemporalViolation::InvalidData(
                        "Void operation cannot be in the past".into(),
                    ));
                }

                let target_op = match self.get_operation_by_id(target_id) {
                    Some(op) => op,
                    None => {
                        return Err(TemporalViolation::OperationNotFound(target_id.to_string()));
                    }
                };

                // an operation can not be void twice
                if target_op.links.void_by.is_some() {
                    return Err(TemporalViolation::OperationAlreadyVoided(
                        target_id.to_string(),
                    ))?;
                }

                // No void on Init, Adj, Checkpoint
                if target_op.kind.is_system() {
                    return Err(TemporalViolation::InvalidData(
                        "System op cannot be voided".into(),
                    ));
                }

                let latest_system_dt = [last_init, last_chk, last_adj]
                    .into_iter()
                    .flatten() // remove Option::None
                    .max();

                if let Some(sys_dt) = latest_system_dt
                    && target_op.date <= sys_dt
                {
                    return Err(TemporalViolation::InvalidData(format!(
                        "Operation #{} is locked by a system operation at {}.",
                        target_id, sys_dt
                    )));
                }
            }
        }
        Ok(())
    }
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parse_date;
    use crate::logic::account::AccountType;
    use crate::logic::operation::{OperationFlow, RegularKind};
    use chrono::Duration;
    use rust_decimal_macros::dec;

    // Helper to create an empty account
    fn setup_empty_account() -> Account {
        // init
        Account::new(
            parse_date("2025-09-01").unwrap(),
            "Test".into(),
            AccountType::Current,
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn test_void_locked_by_system_date() {
        let mut account = setup_empty_account();
        let today = Local::now().date_naive();
        let yesterday = today - Duration::days(1); // yesterday

        println!("{}", today);
        println!("{}", yesterday);

        // init the account
        account.initialize(yesterday, dec!(100)).unwrap();

        // add operation
        let op_id = account
            .register_transaction(
                yesterday,
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Debit,
                dec!(25),
                "Grocery".into(),
            )
            .unwrap();

        // adjust account amount
        account.adjust_balance(today, dec!(50)).unwrap();

        // 3. test polycy : void of the yeterday operation shall fail due to the adjust
        let res = account.temporal_policy(TemporalAction::Void(op_id), today);

        assert!(res.is_err(), "void should be lock due to the adjustment");
    }

    #[test]
    fn test_create_future_date_fail() {
        let account = setup_empty_account();
        let tomorrow = Local::now().date_naive() + Duration::days(1); // tomorrow

        let res = account.temporal_policy(
            TemporalAction::Create(&OperationKind::System(SystemKind::Init)),
            tomorrow,
        );

        assert!(matches!(res, Err(TemporalViolation::InvalidData(msg)) if msg.contains("future")));
    }
}
