// src/logic/account/policy/compliance.rs

use chrono::{Datelike, Local};
use rust_decimal::Decimal;
use std::ops::Neg;

use crate::logic::{
    account::policy::{AccountContext, ComplianceViolation},
    account::{Account, AccountType},
    operation::{OperationFlow, OperationKind, SystemKind},
};

/// Action passed to compliance_policy() — a subclass of TemporalAction.
/// Contains the full context of the operation to be validated.
#[derive(Debug)]
pub enum ComplianceAction<'a> {
    /// Creating a transaction: type + direction + gross amount (always positive for Regular,
    /// may be signed for System ops calculated internally e.g. Init, Adjust)
    Create(&'a OperationKind, OperationFlow, Decimal),
    /// Reversal of a transaction — accounting adjustment, always goes through
    Void,
}

/// Business policy linked to the account type.
/// Receives a ComplianceAction containing kind, flow and amount.
/// Each implementation decides what to check based on the kind.
/// Numeric values always come from AccountContext.
///
/// Amount contract:
///   - Regular(_)         → amount must be strictly positive (user-provided, validated here)
///   - System(Init|Adjust)→ amount is always positive (pre-sanitized via .abs() by caller)
///   - System(Checkpoint) → no financial validation
///   - System(Void)       → no financial validation
pub trait CompliancePolicy {
    fn validate(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        action: ComplianceAction,
        monthly_count: u32,
    ) -> Result<(), ComplianceViolation> {
        match action {
            // Void — accounting adjustment, always OK
            ComplianceAction::Void => Ok(()),

            ComplianceAction::Create(kind, flow, amount) => {
                match kind {
                    // Checkpoint — end of period, not an actual financial movement
                    OperationKind::System(SystemKind::Checkpoint) => Ok(()),

                    // System Void — already handled by ComplianceAction::Void
                    // but in case it goes through Create
                    OperationKind::System(SystemKind::Void) => Ok(()),

                    // Init and Adjust — internal regularizations.
                    // Amount is pre-sanitized via .abs() by the caller (action.rs),
                    // flow is derived via OperationFlow::from_sign().
                    // Only overdraft is checked — no quota, no min_balance.
                    OperationKind::System(SystemKind::Init)
                    | OperationKind::System(SystemKind::Adjust) => {
                        // Amount is guaranteed positive by caller — compute signed directly
                        let signed = amount * flow.to_sign();
                        self.validate_overdraft(ctx, current_balance, signed)
                    }

                    // Regular (Transaction, Fee, Transfer, Refund) — full validation.
                    // Amount must be strictly positive — user-provided value, validated here
                    // before operation.build() to give a meaningful compliance error early.
                    // flow carries the direction (Debit/Credit).
                    OperationKind::Regular(_) => {
                        // ← MODIFIED: amount > 0 check moved here from operation.build()
                        // for Regular ops to provide a compliance-level error with context
                        if amount <= Decimal::ZERO {
                            return Err(ComplianceViolation::InvalidAmount { amount });
                        }
                        let signed = amount * flow.to_sign();
                        self.validate_full(ctx, current_balance, signed, monthly_count)
                    }
                }
            }
        }
    }

    /// Checks only the overdraft limit.
    /// Used for Init and Adjust — no quota, no min_balance.
    fn validate_overdraft(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
    ) -> Result<(), ComplianceViolation> {
        let resulting = current_balance + signed;
        if resulting < ctx.overdraft_limit.neg() {
            return Err(ComplianceViolation::OverdraftExceeded {
                limit: ctx.overdraft_limit,
                resulting,
            });
        }
        Ok(())
    }

    /// Checks overdraft + min_balance + monthly quota.
    /// Used for Regular (Transaction, Fee, Transfer, Refund).
    /// Can be overridden per account type (e.g. Saving, Deposit).
    /// Receives signed amount — caller is responsible for computing it.
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
    ) -> Result<(), ComplianceViolation> {
        let resulting = current_balance + signed;

        // Overdraft
        if resulting < ctx.overdraft_limit.neg() {
            return Err(ComplianceViolation::OverdraftExceeded {
                limit: ctx.overdraft_limit,
                resulting,
            });
        }

        // Minimum balance — only enforced when no overdraft is allowed
        if ctx.overdraft_limit == Decimal::ZERO && resulting < ctx.min_balance {
            return Err(ComplianceViolation::MinBalanceViolated {
                minimum: ctx.min_balance,
                resulting,
            });
        }

        // Monthly quota
        if let Some(max) = ctx.max_monthly_transactions
            && monthly_count >= max
        {
            return Err(ComplianceViolation::MonthlyLimitExceeded { max });
        }

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────
// Implementations per account type
// ─────────────────────────────────────────────────────────────

pub struct CurrentPolicy;
impl CompliancePolicy for CurrentPolicy {}
// No override — default trait behavior

pub struct JointPolicy;
impl CompliancePolicy for JointPolicy {}

pub struct BusinessPolicy;
impl CompliancePolicy for BusinessPolicy {}

pub struct StudentPolicy;
impl CompliancePolicy for StudentPolicy {}

pub struct LoanPolicy;
impl CompliancePolicy for LoanPolicy {
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
    ) -> Result<(), ComplianceViolation> {
        // A loan account can never go negative — no overdraft allowed
        if current_balance + signed < Decimal::ZERO {
            return Err(ComplianceViolation::NotAllowed {
                reason: "saving account cannot go negative",
            });
        }

        // Delegate remaining checks to shared logic
        // Note: overdraft check is skipped here since savings accounts
        // are already blocked from going negative above
        let resulting = current_balance + signed;

        if ctx.overdraft_limit == Decimal::ZERO && resulting < ctx.min_balance {
            return Err(ComplianceViolation::MinBalanceViolated {
                minimum: ctx.min_balance,
                resulting,
            });
        }
        if let Some(max) = ctx.max_monthly_transactions
            && monthly_count >= max
        {
            return Err(ComplianceViolation::MonthlyLimitExceeded { max });
        }
        Ok(())
    }
}

pub struct SavingPolicy;
impl CompliancePolicy for SavingPolicy {
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
    ) -> Result<(), ComplianceViolation> {
        // A savings account can never go negative — no overdraft allowed
        if current_balance + signed < Decimal::ZERO {
            return Err(ComplianceViolation::NotAllowed {
                reason: "saving account cannot go negative",
            });
        }

        // Delegate remaining checks to shared logic
        // Note: overdraft check is skipped here since savings accounts
        // are already blocked from going negative above
        let resulting = current_balance + signed;

        if ctx.overdraft_limit == Decimal::ZERO && resulting < ctx.min_balance {
            return Err(ComplianceViolation::MinBalanceViolated {
                minimum: ctx.min_balance,
                resulting,
            });
        }
        if let Some(max) = ctx.max_monthly_transactions
            && monthly_count >= max
        {
            return Err(ComplianceViolation::MonthlyLimitExceeded { max });
        }
        Ok(())
    }
}

pub struct DepositPolicy;
impl CompliancePolicy for DepositPolicy {
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
    ) -> Result<(), ComplianceViolation> {
        // Withdrawal (negative signed amount) blocked before maturity date
        if signed < Decimal::ZERO
            && let Some(locked_until) = ctx.deposit_locked_until
            && Local::now().date_naive() < locked_until
        {
            return Err(ComplianceViolation::NotAllowed {
                reason: "withdrawal locked until deposit maturity date",
            });
        }

        // Delegate to common checks
        let resulting = current_balance + signed;
        if resulting < ctx.overdraft_limit.neg() {
            return Err(ComplianceViolation::OverdraftExceeded {
                limit: ctx.overdraft_limit,
                resulting,
            });
        }
        if ctx.overdraft_limit == Decimal::ZERO && resulting < ctx.min_balance {
            return Err(ComplianceViolation::MinBalanceViolated {
                minimum: ctx.min_balance,
                resulting,
            });
        }
        if let Some(max) = ctx.max_monthly_transactions
            && monthly_count >= max
        {
            return Err(ComplianceViolation::MonthlyLimitExceeded { max });
        }
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────
// Dispatch
// ─────────────────────────────────────────────────────────────

/// Returns the policy corresponding to the account type.
pub fn policy_for(account_type: AccountType) -> Box<dyn CompliancePolicy> {
    match account_type {
        AccountType::Current => Box::new(CurrentPolicy),
        AccountType::Loan => Box::new(LoanPolicy),
        AccountType::Saving => Box::new(SavingPolicy),
        AccountType::Joint => Box::new(JointPolicy),
        AccountType::Deposit => Box::new(DepositPolicy),
        AccountType::Business => Box::new(BusinessPolicy),
        AccountType::Student => Box::new(StudentPolicy),
    }
}

// ─────────────────────────────────────────────────────────────
// Method on Account
// ─────────────────────────────────────────────────────────────

impl Account {
    /// Validates the business policy of the account for a given action.
    /// Single entry point called from action.rs,
    /// after temporal_policy() and before commit_operation().
    pub fn compliance_policy(
        &self,
        action: ComplianceAction,
        date: chrono::NaiveDate,
    ) -> Result<(), ComplianceViolation> {
        // Monthly count is only relevant for Regular operations
        let monthly_count = match action {
            ComplianceAction::Create(kind, _, _) if kind.is_regular() => {
                self.monthly_operation_count(date)
            }
            _ => 0,
        };

        // Balance at operation date — not current_balance
        let balance_at_date = self.balance_at(date);

        policy_for(self.context.account_type).validate(
            &self.context,
            balance_at_date,
            action,
            monthly_count,
        )
    }

    /// Counts active Regular operations for the month corresponding to `date`.
    /// Voided operations (void_by set) are excluded from the count.
    pub fn monthly_operation_count(&self, date: chrono::NaiveDate) -> u32 {
        self.operations
            .iter()
            .filter(|op| {
                op.kind.is_regular()
                    && op.links.void_by.is_none()
                    && op.date.year() == date.year()
                    && op.date.month() == date.month()
            })
            .count() as u32
    }
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::{account::policy::context::AccountContext, operation::RegularKind};
    use chrono::{Duration, Local};
    use rust_decimal_macros::dec;

    fn ctx_current() -> AccountContext {
        AccountContext::from_type(AccountType::Current)
    }
    fn ctx_saving() -> AccountContext {
        AccountContext::from_type(AccountType::Saving)
    }
    fn ctx_deposit_locked() -> AccountContext {
        let until = Local::now().date_naive() + Duration::days(30);
        let mut ctx = AccountContext::from_type(AccountType::Deposit);
        ctx.update_context(None, None, None, Some(until), None, None)
            .unwrap();
        ctx
    }
    fn ctx_deposit_unlocked() -> AccountContext {
        let past = Local::now().date_naive() - Duration::days(1);
        let mut ctx = AccountContext::from_type(AccountType::Deposit);
        ctx.update_context(None, None, None, Some(past), None, None)
            .unwrap();
        ctx
    }

    // ── Regular / Current ────────────────────────────────────

    #[test]
    fn current_within_overdraft() {
        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(550),
        );
        let res = CurrentPolicy.validate(&ctx_current(), dec!(100), action, 0);
        assert!(res.is_ok());
    }

    #[test]
    fn current_overdraft_exceeded() {
        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(700),
        );
        let res = CurrentPolicy.validate(&ctx_current(), dec!(100), action, 0);
        assert!(matches!(
            res,
            Err(ComplianceViolation::OverdraftExceeded { .. })
        ));
    }

    #[test]
    fn current_custom_overdraft() {
        let mut ctx = AccountContext::from_type(AccountType::Current);
        ctx.update_context(Some(dec!(2_000)), None, None, None, None, None)
            .unwrap();

        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(1_500),
        );
        let res = CurrentPolicy.validate(&ctx, dec!(100), action, 0);
        assert!(res.is_ok());
    }

    // ── Init ─────────────────────────────────────────────────

    #[test]
    fn init_within_overdraft() {
        let action = ComplianceAction::Create(
            &OperationKind::System(SystemKind::Init),
            OperationFlow::Debit,
            dec!(300),
        );
        let res = CurrentPolicy.validate(&ctx_current(), dec!(0), action, 0);
        assert!(res.is_ok());
    }

    #[test]
    fn init_exceeds_overdraft() {
        let action = ComplianceAction::Create(
            &OperationKind::System(SystemKind::Init),
            OperationFlow::Debit,
            dec!(700),
        );
        let res = CurrentPolicy.validate(&ctx_current(), dec!(0), action, 0);
        assert!(matches!(
            res,
            Err(ComplianceViolation::OverdraftExceeded { .. })
        ));
    }

    #[test]
    fn init_ignores_monthly_quota() {
        let action = ComplianceAction::Create(
            &OperationKind::System(SystemKind::Init),
            OperationFlow::Credit,
            dec!(100),
        );
        let res = SavingPolicy.validate(&ctx_saving(), dec!(0), action, 99);
        assert!(res.is_ok());
    }

    // ── Void ─────────────────────────────────────────────────

    #[test]
    fn void_always_passes() {
        let res = CurrentPolicy.validate(&ctx_current(), dec!(-900), ComplianceAction::Void, 0);
        assert!(res.is_ok());
    }

    // ── Checkpoint ───────────────────────────────────────────

    #[test]
    fn checkpoint_always_passes() {
        let action = ComplianceAction::Create(
            &OperationKind::System(SystemKind::Checkpoint),
            OperationFlow::None,
            dec!(0),
        );
        let res = CurrentPolicy.validate(&ctx_current(), dec!(-900), action, 0);
        assert!(res.is_ok());
    }

    // ── Saving ───────────────────────────────────────────────

    #[test]
    fn saving_cannot_go_negative() {
        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(100),
        );
        let res = SavingPolicy.validate(&ctx_saving(), dec!(50), action, 0);
        assert!(matches!(res, Err(ComplianceViolation::NotAllowed { .. })));
    }

    #[test]
    fn saving_monthly_limit_reached() {
        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(10),
        );
        let res = SavingPolicy.validate(&ctx_saving(), dec!(500), action, 6);
        assert!(matches!(
            res,
            Err(ComplianceViolation::MonthlyLimitExceeded { max: 6 })
        ));
    }

    // ── Deposit ──────────────────────────────────────────────

    #[test]
    fn deposit_withdrawal_blocked_before_maturity() {
        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(100),
        );
        let res = DepositPolicy.validate(&ctx_deposit_locked(), dec!(1_000), action, 0);
        assert!(matches!(res, Err(ComplianceViolation::NotAllowed { .. })));
    }

    #[test]
    fn deposit_withdrawal_allowed_after_maturity() {
        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Debit,
            dec!(100),
        );
        let res = DepositPolicy.validate(&ctx_deposit_unlocked(), dec!(1_000), action, 0);
        assert!(res.is_ok());
    }

    #[test]
    fn deposit_credit_always_allowed() {
        let action = ComplianceAction::Create(
            &OperationKind::Regular(RegularKind::Transaction),
            OperationFlow::Credit,
            dec!(500),
        );
        let res = DepositPolicy.validate(&ctx_deposit_locked(), dec!(1_000), action, 0);
        assert!(res.is_ok());
    }
}
