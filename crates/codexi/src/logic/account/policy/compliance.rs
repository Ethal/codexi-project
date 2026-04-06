// src/logic/account/policy/compliance.rs

use chrono::{Datelike, Local};
use rust_decimal::Decimal;
use std::ops::Neg;

use crate::logic::{
    account::policy::{AccountContext, ComplianceViolation},
    account::{Account, AccountType},
    operation::{OperationFlow, OperationKind, RegularKind, SystemKind},
};

#[derive(Debug)]
pub enum ComplianceAction {
    Create(OperationKind, OperationFlow, Decimal),
    Void,
}

pub trait CompliancePolicy {
    fn validate(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        action: ComplianceAction,
        monthly_count: u32,
    ) -> Result<(), ComplianceViolation> {
        match action {
            ComplianceAction::Void => Ok(()),

            ComplianceAction::Create(kind, flow, amount) => match kind {
                // Checkpoint and Void — always OK, no financial movement
                OperationKind::System(SystemKind::Checkpoint) | OperationKind::System(SystemKind::Void) => Ok(()),

                // Init and Adjust — overdraft only, no quota, no min_balance
                OperationKind::System(SystemKind::Init) | OperationKind::System(SystemKind::Adjust) => {
                    let signed = amount * flow.to_sign();
                    self.validate_overdraft(ctx, current_balance, signed)
                }

                // Regular — common guards applied here before validate_full
                OperationKind::Regular(regular_kind) => {
                    // Amount must be strictly positive for all Regular ops
                    if amount <= Decimal::ZERO {
                        return Err(ComplianceViolation::InvalidAmount(amount));
                    }
                    // Interest Debit — never allowed on any type (use void instead)
                    if regular_kind == RegularKind::Interest && flow == OperationFlow::Debit {
                        return Err(ComplianceViolation::KindNotAllowed(ctx.account_type));
                    }
                    // Refund Debit — never allowed on any type (use void instead)
                    if regular_kind == RegularKind::Refund && flow == OperationFlow::Debit {
                        return Err(ComplianceViolation::KindNotAllowed(ctx.account_type));
                    }
                    // Interest Credit — requires allows_interest flag on the account
                    if regular_kind == RegularKind::Interest && flow == OperationFlow::Credit && !ctx.allows_interest {
                        return Err(ComplianceViolation::NotAllowedInterestOperation);
                    }
                    let signed = amount * flow.to_sign();
                    self.validate_full(ctx, current_balance, signed, monthly_count, &regular_kind, flow)
                }
            },
        }
    }

    /// Checks only the overdraft limit.
    /// Used for Init and Adjust — no quota, no min_balance, no kind guard.
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

    /// Full validation for Regular operations.
    /// kind and flow are passed to allow per-type kind/flow guards without
    /// duplicating the dispatch logic from validate().
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
        _kind: &RegularKind,
        _flow: OperationFlow,
    ) -> Result<(), ComplianceViolation> {
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
// Shared helper — no-overdraft account types (Saving, Deposit, Loan)
// ─────────────────────────────────────────────────────────────

fn validate_no_overdraft(
    ctx: &AccountContext,
    current_balance: Decimal,
    signed: Decimal,
    monthly_count: u32,
) -> Result<(), ComplianceViolation> {
    if current_balance + signed < Decimal::ZERO {
        return Err(ComplianceViolation::NegativeBalanceNotAllowed(ctx.account_type));
    }
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

// ─────────────────────────────────────────────────────────────
// Implementations per account type
// ─────────────────────────────────────────────────────────────

pub struct CurrentPolicy;
impl CompliancePolicy for CurrentPolicy {}
// Default behavior: overdraft allowed, no kind restrictions

pub struct JointPolicy;
impl CompliancePolicy for JointPolicy {}

pub struct BusinessPolicy;
impl CompliancePolicy for BusinessPolicy {}

pub struct StudentPolicy;
impl CompliancePolicy for StudentPolicy {}

// ── Saving ─────────────────────────────────────────
// No overdraft. Only Transfer allowed for debit movements.
// Transaction Debit and Fee Debit forbidden — Transfer only.
// Credits unrestricted (Transaction, Transfer, Refund, Fee, Interest).
pub struct SavingPolicy;
impl CompliancePolicy for SavingPolicy {
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
        kind: &RegularKind,
        flow: OperationFlow,
    ) -> Result<(), ComplianceViolation> {
        // Only Transfer is allowed as a debit movement on Saving
        if flow == OperationFlow::Debit && *kind != RegularKind::Transfer {
            return Err(ComplianceViolation::KindNotAllowed(ctx.account_type));
        }
        validate_no_overdraft(ctx, current_balance, signed, monthly_count)
    }
}

// ── Deposit ─────────────────────────────────────────────
// Credits always allowed. Only Transfer allowed for debit movements.
// Transaction Debit forbidden even after maturity — Transfer only.
// All debits blocked before maturity date regardless of kind.
// No overdraft — no negative balance.
pub struct DepositPolicy;
impl CompliancePolicy for DepositPolicy {
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
        kind: &RegularKind,
        flow: OperationFlow,
    ) -> Result<(), ComplianceViolation> {
        if flow == OperationFlow::Debit {
            // All debits blocked before maturity date
            if let Some(locked_until) = ctx.deposit_locked_until
                && Local::now().date_naive() < locked_until
            {
                return Err(ComplianceViolation::NoWithdrawalAllowed);
            }
            // Only Transfer allowed as debit movement (after maturity)
            if *kind != RegularKind::Transfer {
                return Err(ComplianceViolation::KindNotAllowed(ctx.account_type));
            }
        }
        validate_no_overdraft(ctx, current_balance, signed, monthly_count)
    }
}

// ── Income ────────────────────────────────────────────────────
// Pure accumulation account — Transfer only in both directions.
// Transaction, Fee, Refund, Interest all forbidden.
// No overdraft — no negative balance.
pub struct IncomePolicy;
impl CompliancePolicy for IncomePolicy {
    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
        kind: &RegularKind,
        _flow: OperationFlow,
    ) -> Result<(), ComplianceViolation> {
        // Only Transfer allowed — all other kinds forbidden
        if *kind != RegularKind::Transfer {
            return Err(ComplianceViolation::KindNotAllowed(ctx.account_type));
        }
        validate_no_overdraft(ctx, current_balance, signed, monthly_count)
    }
}

// ── Loan ──────────────────────────────────────────────────────
// Only Transfer (both), Interest Credit, Fee (both) allowed.
// Transaction and Refund Credit forbidden — Transfer only.
// Init with non-zero amount forbidden.
// No overdraft — no negative balance.

pub struct LoanPolicy;
impl CompliancePolicy for LoanPolicy {
    fn validate(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        action: ComplianceAction,
        monthly_count: u32,
    ) -> Result<(), ComplianceViolation> {
        // Init with non-zero amount — forbidden on Loan accounts
        if let ComplianceAction::Create(OperationKind::System(SystemKind::Init), flow, amount) = &action
            && *flow != OperationFlow::None
            && *amount != Decimal::ZERO
        {
            return Err(ComplianceViolation::InitNonZeroOnLoan);
        }
        // Delegate to default dispatch — validate_full handles kind/flow guards
        match action {
            ComplianceAction::Void => Ok(()),
            ComplianceAction::Create(kind, flow, amount) => match kind {
                OperationKind::System(SystemKind::Checkpoint) | OperationKind::System(SystemKind::Void) => Ok(()),
                OperationKind::System(SystemKind::Init) | OperationKind::System(SystemKind::Adjust) => {
                    let signed = amount * flow.to_sign();
                    self.validate_overdraft(ctx, current_balance, signed)
                }
                OperationKind::Regular(regular_kind) => {
                    if amount <= Decimal::ZERO {
                        return Err(ComplianceViolation::InvalidAmount(amount));
                    }
                    if regular_kind == RegularKind::Interest && flow == OperationFlow::Debit {
                        return Err(ComplianceViolation::KindNotAllowed(ctx.account_type));
                    }
                    if regular_kind == RegularKind::Refund && flow == OperationFlow::Debit {
                        return Err(ComplianceViolation::KindNotAllowed(ctx.account_type));
                    }
                    if regular_kind == RegularKind::Interest && flow == OperationFlow::Credit && !ctx.allows_interest {
                        return Err(ComplianceViolation::NotAllowedInterestOperation);
                    }
                    let signed = amount * flow.to_sign();
                    self.validate_full(ctx, current_balance, signed, monthly_count, &regular_kind, flow)
                }
            },
        }
    }

    fn validate_full(
        &self,
        ctx: &AccountContext,
        current_balance: Decimal,
        signed: Decimal,
        monthly_count: u32,
        kind: &RegularKind,
        flow: OperationFlow,
    ) -> Result<(), ComplianceViolation> {
        // Only Transfer, Interest Credit, and Fee allowed
        // Transaction and Refund Credit forbidden (Transfer only for cash movements)
        match (kind, flow) {
            (RegularKind::Transfer, _) => {}
            (RegularKind::Interest, OperationFlow::Credit) => {}
            (RegularKind::Fee, _) => {}
            _ => return Err(ComplianceViolation::KindNotAllowed(ctx.account_type)),
        }
        validate_no_overdraft(ctx, current_balance, signed, monthly_count)
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
        AccountType::Income => Box::new(IncomePolicy),
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
            ComplianceAction::Create(kind, _, _) if kind.is_regular() => self.monthly_operation_count(date),
            _ => 0,
        };

        // Balance at operation date — not current_balance
        let balance_at_date = self.balance_at(date);

        policy_for(self.context.account_type).validate(&self.context, balance_at_date, action, monthly_count)
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
    use chrono::{Duration, Local};
    use rust_decimal_macros::dec;

    // ── Context helpers ───────────────────────────────────────

    fn ctx_current() -> AccountContext {
        AccountContext::from_type(AccountType::Current)
    }
    fn ctx_saving() -> AccountContext {
        AccountContext::from_type(AccountType::Saving)
    }
    fn ctx_loan() -> AccountContext {
        AccountContext::from_type(AccountType::Loan)
    }
    fn ctx_loan_no_interest() -> AccountContext {
        let mut ctx = AccountContext::from_type(AccountType::Loan);
        ctx.update_context(None, None, None, None, Some(false), None).unwrap();
        ctx
    }
    fn ctx_deposit_locked() -> AccountContext {
        let until = Local::now().date_naive() + Duration::days(30);
        let mut ctx = AccountContext::from_type(AccountType::Deposit);
        ctx.update_context(None, None, None, Some(until), None, None).unwrap();
        ctx
    }
    fn ctx_deposit_unlocked() -> AccountContext {
        let past = Local::now().date_naive() - Duration::days(1);
        let mut ctx = AccountContext::from_type(AccountType::Deposit);
        ctx.update_context(None, None, None, Some(past), None, None).unwrap();
        ctx
    }
    fn ctx_deposit_no_lock() -> AccountContext {
        AccountContext::from_type(AccountType::Deposit)
    }
    fn ctx_income() -> AccountContext {
        AccountContext::from_type(AccountType::Income)
    }

    // ── Action helpers ────────────────────────────────────────

    fn regular(kind: RegularKind, flow: OperationFlow, amount: Decimal) -> ComplianceAction {
        ComplianceAction::Create(OperationKind::Regular(kind), flow, amount)
    }
    fn system(kind: SystemKind, flow: OperationFlow, amount: Decimal) -> ComplianceAction {
        ComplianceAction::Create(OperationKind::System(kind), flow, amount)
    }

    // ── Common guards (apply to all types) ───────────────────

    #[test]
    fn void_always_ok() {
        for policy in [
            &CurrentPolicy as &dyn CompliancePolicy,
            &JointPolicy,
            &BusinessPolicy,
            &StudentPolicy,
            &SavingPolicy,
            &DepositPolicy,
            &LoanPolicy,
            &IncomePolicy,
        ] {
            assert!(
                policy
                    .validate(&ctx_current(), dec!(-900), ComplianceAction::Void, 0)
                    .is_ok()
            );
        }
    }

    #[test]
    fn checkpoint_always_ok() {
        let action = || system(SystemKind::Checkpoint, OperationFlow::None, dec!(0));
        assert!(CurrentPolicy.validate(&ctx_current(), dec!(-900), action(), 0).is_ok());
        assert!(LoanPolicy.validate(&ctx_loan(), dec!(0), action(), 0).is_ok());
        assert!(SavingPolicy.validate(&ctx_saving(), dec!(0), action(), 0).is_ok());
    }

    #[test]
    fn invalid_amount_refused_all_types() {
        let action = || regular(RegularKind::Transaction, OperationFlow::Credit, dec!(0));
        assert!(matches!(
            CurrentPolicy.validate(&ctx_current(), dec!(100), action(), 0),
            Err(ComplianceViolation::InvalidAmount(_))
        ));
        assert!(matches!(
            SavingPolicy.validate(&ctx_saving(), dec!(100), action(), 0),
            Err(ComplianceViolation::InvalidAmount(_))
        ));
    }

    #[test]
    fn interest_debit_refused_all_types() {
        let action = || regular(RegularKind::Interest, OperationFlow::Debit, dec!(10));
        assert!(matches!(
            CurrentPolicy.validate(&ctx_current(), dec!(100), action(), 0),
            Err(ComplianceViolation::KindNotAllowed(_))
        ));
        assert!(matches!(
            SavingPolicy.validate(&ctx_saving(), dec!(100), action(), 0),
            Err(ComplianceViolation::KindNotAllowed(_))
        ));
        assert!(matches!(
            LoanPolicy.validate(&ctx_loan(), dec!(100), action(), 0),
            Err(ComplianceViolation::KindNotAllowed(_))
        ));
    }

    #[test]
    fn refund_debit_refused_all_types() {
        let action = || regular(RegularKind::Refund, OperationFlow::Debit, dec!(10));
        assert!(matches!(
            CurrentPolicy.validate(&ctx_current(), dec!(100), action(), 0),
            Err(ComplianceViolation::KindNotAllowed(_))
        ));
        assert!(matches!(
            LoanPolicy.validate(&ctx_loan(), dec!(100), action(), 0),
            Err(ComplianceViolation::KindNotAllowed(_))
        ));
    }

    #[test]
    fn interest_credit_refused_without_flag() {
        let action = || regular(RegularKind::Interest, OperationFlow::Credit, dec!(10));
        assert!(matches!(
            CurrentPolicy.validate(&ctx_current(), dec!(100), action(), 0),
            Err(ComplianceViolation::NotAllowedInterestOperation)
        ));
        assert!(matches!(
            LoanPolicy.validate(&ctx_loan_no_interest(), dec!(100), action(), 0),
            Err(ComplianceViolation::NotAllowedInterestOperation)
        ));
    }

    // ── Current / Joint / Business / Student ─────────────────

    #[test]
    fn current_transaction_credit_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(0),
            regular(RegularKind::Transaction, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn current_transaction_debit_within_overdraft_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(100),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(550)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn current_transaction_debit_exceeds_overdraft() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(100),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(700)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::OverdraftExceeded { .. })));
    }

    #[test]
    fn current_custom_overdraft_ok() {
        let mut ctx = ctx_current();
        ctx.update_context(Some(dec!(2_000)), None, None, None, None, None)
            .unwrap();
        let res = CurrentPolicy.validate(
            &ctx,
            dec!(100),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(1_500)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn current_fee_credit_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(0),
            regular(RegularKind::Fee, OperationFlow::Credit, dec!(10)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn current_fee_debit_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(100),
            regular(RegularKind::Fee, OperationFlow::Debit, dec!(10)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn current_refund_credit_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(0),
            regular(RegularKind::Refund, OperationFlow::Credit, dec!(50)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn current_transfer_credit_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(0),
            regular(RegularKind::Transfer, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn current_transfer_debit_within_overdraft_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(100),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(550)),
            0,
        );
        assert!(res.is_ok());
    }

    // ── System / Init / Adjust ────────────────────────────────

    #[test]
    fn init_within_overdraft_ok() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(0),
            system(SystemKind::Init, OperationFlow::Debit, dec!(300)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn init_exceeds_overdraft() {
        let res = CurrentPolicy.validate(
            &ctx_current(),
            dec!(0),
            system(SystemKind::Init, OperationFlow::Debit, dec!(700)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::OverdraftExceeded { .. })));
    }

    #[test]
    fn init_ignores_monthly_quota() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(0),
            system(SystemKind::Init, OperationFlow::Credit, dec!(100)),
            99,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn adjust_saving_no_negative() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(50),
            system(SystemKind::Adjust, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::OverdraftExceeded { .. })));
    }

    // ── Saving ────────────────────────────────────────────────

    // Transaction Debit maintenant refusé (transfer only)
    #[test]
    fn saving_transaction_debit_refused() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(500),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    // Fee Debit maintenant refusé (transfer only)
    #[test]
    fn saving_fee_debit_refused() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(500),
            regular(RegularKind::Fee, OperationFlow::Debit, dec!(10)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    // Transfer Debit maintenant autorisé (no negative)
    #[test]
    fn saving_transfer_debit_ok() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(1_000),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn saving_transfer_debit_no_negative() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(50),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NegativeBalanceNotAllowed(_))));
    }

    #[test]
    fn saving_transaction_credit_ok() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(0),
            regular(RegularKind::Transaction, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn saving_transaction_debit_no_negative() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(50),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NegativeBalanceNotAllowed(_))));
    }

    #[test]
    fn saving_transfer_credit_ok() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(0),
            regular(RegularKind::Transfer, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn saving_interest_credit_ok() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(0),
            regular(RegularKind::Interest, OperationFlow::Credit, dec!(10)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn saving_monthly_limit_reached() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(500),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(10)),
            6,
        );
        assert!(matches!(res, Err(ComplianceViolation::MonthlyLimitExceeded { max: 6 })));
    }

    #[test]
    fn saving_refund_credit_ok() {
        let res = SavingPolicy.validate(
            &ctx_saving(),
            dec!(0),
            regular(RegularKind::Refund, OperationFlow::Credit, dec!(50)),
            0,
        );
        assert!(res.is_ok());
    }

    // ── Deposit ───────────────────────────────────────────────

    // Transaction Debit refusé même après maturité (transfer only)
    #[test]
    fn deposit_transaction_debit_refused_after_maturity() {
        let res = DepositPolicy.validate(
            &ctx_deposit_unlocked(),
            dec!(1_000),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    // Transfer Debit autorisé après maturité
    #[test]
    fn deposit_transfer_debit_ok_after_maturity() {
        let res = DepositPolicy.validate(
            &ctx_deposit_unlocked(),
            dec!(1_000),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn deposit_transaction_credit_always_ok() {
        let res = DepositPolicy.validate(
            &ctx_deposit_locked(),
            dec!(0),
            regular(RegularKind::Transaction, OperationFlow::Credit, dec!(500)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn deposit_transfer_credit_always_ok() {
        let res = DepositPolicy.validate(
            &ctx_deposit_locked(),
            dec!(0),
            regular(RegularKind::Transfer, OperationFlow::Credit, dec!(500)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn deposit_fee_credit_always_ok() {
        let res = DepositPolicy.validate(
            &ctx_deposit_locked(),
            dec!(1_000),
            regular(RegularKind::Fee, OperationFlow::Credit, dec!(10)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn deposit_transaction_debit_blocked_before_maturity() {
        let res = DepositPolicy.validate(
            &ctx_deposit_locked(),
            dec!(1_000),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NoWithdrawalAllowed)));
    }

    #[test]
    fn deposit_transfer_debit_blocked_before_maturity() {
        let res = DepositPolicy.validate(
            &ctx_deposit_locked(),
            dec!(1_000),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NoWithdrawalAllowed)));
    }

    #[test]
    fn deposit_fee_debit_blocked_before_maturity() {
        let res = DepositPolicy.validate(
            &ctx_deposit_locked(),
            dec!(1_000),
            regular(RegularKind::Fee, OperationFlow::Debit, dec!(10)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NoWithdrawalAllowed)));
    }

    #[test]
    fn deposit_debit_no_negative_after_maturity() {
        let res = DepositPolicy.validate(
            &ctx_deposit_unlocked(),
            dec!(50),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NegativeBalanceNotAllowed(_))));
    }

    #[test]
    fn deposit_no_lock_date_debit_ok() {
        let res = DepositPolicy.validate(
            &ctx_deposit_no_lock(),
            dec!(1_000),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    // ── Loan ──────────────────────────────────────────────────

    #[test]
    fn loan_init_zero_ok() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(0),
            system(SystemKind::Init, OperationFlow::None, dec!(0)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn loan_init_nonzero_refused() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(0),
            system(SystemKind::Init, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::InitNonZeroOnLoan)));
    }

    #[test]
    fn loan_transaction_credit_refused() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(1_000),
            regular(RegularKind::Transaction, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    #[test]
    fn loan_transaction_debit_refused() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(1_000),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    #[test]
    fn loan_refund_credit_refused() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(1_000),
            regular(RegularKind::Refund, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    #[test]
    fn loan_transfer_credit_ok() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(0),
            regular(RegularKind::Transfer, OperationFlow::Credit, dec!(500)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn loan_transfer_debit_ok() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(1_000),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(500)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn loan_transfer_debit_no_negative() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(100),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(200)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NegativeBalanceNotAllowed(_))));
    }

    #[test]
    fn loan_interest_credit_ok() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(1_000),
            regular(RegularKind::Interest, OperationFlow::Credit, dec!(20)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn loan_interest_credit_refused_no_flag() {
        let res = LoanPolicy.validate(
            &ctx_loan_no_interest(),
            dec!(1_000),
            regular(RegularKind::Interest, OperationFlow::Credit, dec!(20)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NotAllowedInterestOperation)));
    }

    #[test]
    fn loan_fee_credit_ok() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(1_000),
            regular(RegularKind::Fee, OperationFlow::Credit, dec!(10)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn loan_fee_debit_ok() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(1_000),
            regular(RegularKind::Fee, OperationFlow::Debit, dec!(10)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn loan_fee_debit_no_negative() {
        let res = LoanPolicy.validate(
            &ctx_loan(),
            dec!(5),
            regular(RegularKind::Fee, OperationFlow::Debit, dec!(10)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NegativeBalanceNotAllowed(_))));
    }

    #[test]
    fn loan_void_ok() {
        let res = LoanPolicy.validate(&ctx_loan(), dec!(0), ComplianceAction::Void, 0);
        assert!(res.is_ok());
    }

    // ── Income ──────────────────────────────────────────────────

    #[test]
    fn income_transfer_credit_ok() {
        let res = IncomePolicy.validate(
            &ctx_income(),
            dec!(0),
            regular(RegularKind::Transfer, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn income_transfer_debit_ok() {
        let res = IncomePolicy.validate(
            &ctx_income(),
            dec!(1_000),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn income_transfer_debit_no_negative() {
        let res = IncomePolicy.validate(
            &ctx_income(),
            dec!(50),
            regular(RegularKind::Transfer, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::NegativeBalanceNotAllowed(_))));
    }

    #[test]
    fn income_transaction_credit_refused() {
        let res = IncomePolicy.validate(
            &ctx_income(),
            dec!(0),
            regular(RegularKind::Transaction, OperationFlow::Credit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    #[test]
    fn income_transaction_debit_refused() {
        let res = IncomePolicy.validate(
            &ctx_income(),
            dec!(1_000),
            regular(RegularKind::Transaction, OperationFlow::Debit, dec!(100)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    #[test]
    fn income_fee_refused() {
        let res = IncomePolicy.validate(
            &ctx_income(),
            dec!(1_000),
            regular(RegularKind::Fee, OperationFlow::Debit, dec!(10)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    #[test]
    fn income_refund_refused() {
        let res = IncomePolicy.validate(
            &ctx_income(),
            dec!(1_000),
            regular(RegularKind::Refund, OperationFlow::Credit, dec!(50)),
            0,
        );
        assert!(matches!(res, Err(ComplianceViolation::KindNotAllowed(_))));
    }

    #[test]
    fn income_void_ok() {
        let res = IncomePolicy.validate(&ctx_income(), dec!(0), ComplianceAction::Void, 0);
        assert!(res.is_ok());
    }

    // Income in void_always_ok loop — à ajouter
    // &IncomePolicy dans le tableau du test void_always_ok
}
