// src/logic/account/policy/lifecycle.rs

use chrono::Local;
use chrono::NaiveDate;

use crate::logic::account::Account;
use crate::logic::account::AccountType;
use crate::logic::account::policy::LifecycleViolation;

impl Account {
    /// Validates and applies the account type change.
    /// Not allowed if at least one transaction already exists.
    pub fn set_account_type(
        &mut self,
        account_type: AccountType,
    ) -> Result<(), LifecycleViolation> {
        if self.anchors.latest().is_some() {
            return Err(LifecycleViolation::AccountTypeImmutable);
        }
        self.context.account_type = account_type;
        Ok(())
    }

    /// Validates an account closure date.
    /// Called by Codexi::close_account before applying terminated_date.
    pub fn validate_close_date(&self, date: NaiveDate) -> Result<(), LifecycleViolation> {
        let today = Local::now().date_naive();

        // Not in the futur
        if date > today {
            return Err(LifecycleViolation::CloseDateInFuture);
        }

        // Not befor the open date
        if date < self.open_date {
            return Err(LifecycleViolation::CloseDateBeforeOpenDate(
                date,
                self.open_date,
            ));
        }

        // Not before the last operation
        if let Some(last) = self.anchors.latest()
            && date < last
        {
            return Err(LifecycleViolation::CloseDateBeforeLastOperation(date, last));
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
    use crate::logic::operation::{OperationBuilder, OperationFlow, OperationKind, RegularKind};
    use chrono::Duration;
    use rust_decimal_macros::dec;

    fn setup_empty_account() -> Account {
        Account::new(
            parse_date("2025-01-01").unwrap(),
            "Test".into(),
            AccountType::Current,
            None,
            None,
        )
        .unwrap()
    }

    fn setup_account_with_operation() -> Account {
        let mut account = setup_empty_account();
        let op = OperationBuilder::default()
            .date(parse_date("2025-06-01").unwrap())
            .kind(OperationKind::Regular(RegularKind::Transaction))
            .flow(OperationFlow::Credit)
            .amount(dec!(100))
            .description("test".to_string())
            .account_id(account.id)
            .build()
            .unwrap();
        account.commit_operation(op);
        account
    }

    // ── set_account_type ─────────────────────────────────────

    #[test]
    fn set_type_allowed_on_empty_account() {
        let mut account = setup_empty_account();
        let res = account.set_account_type(AccountType::Saving);
        assert!(res.is_ok());
        assert_eq!(account.context.account_type, AccountType::Saving);
    }

    #[test]
    fn set_type_blocked_with_operations() {
        let mut account = setup_account_with_operation();
        let res = account.set_account_type(AccountType::Saving);
        assert!(matches!(res, Err(LifecycleViolation::AccountTypeImmutable)));
    }

    // ── validate_close_date ──────────────────────────────────

    #[test]
    fn close_date_in_future_fails() {
        let account = setup_empty_account();
        let tomorrow = Local::now().date_naive() + Duration::days(1);
        let res = account.validate_close_date(tomorrow);
        assert!(matches!(res, Err(LifecycleViolation::CloseDateInFuture)));
    }

    #[test]
    fn close_date_before_open_date_fails() {
        let account = setup_empty_account();
        // open_date = 2025-01-01, on tente 2024-12-31
        let before_open = parse_date("2024-12-31").unwrap();
        let res = account.validate_close_date(before_open);
        assert!(matches!(
            res,
            Err(LifecycleViolation::CloseDateBeforeOpenDate(_, _))
        ));
    }

    #[test]
    fn close_date_before_last_operation_fails() {
        let account = setup_account_with_operation();
        // dernière op = 2025-06-01, on tente 2025-05-01
        let before_last = parse_date("2025-05-01").unwrap();
        let res = account.validate_close_date(before_last);
        assert!(matches!(
            res,
            Err(LifecycleViolation::CloseDateBeforeLastOperation(_, _))
        ));
    }

    #[test]
    fn close_date_valid() {
        let account = setup_account_with_operation();
        // dernière op = 2025-06-01, today >= 2025-06-01
        let today = Local::now().date_naive();
        let res = account.validate_close_date(today);
        assert!(res.is_ok());
    }
}
