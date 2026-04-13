// src/logic/account/policy/context.rs

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::core::{CoreWarning, CoreWarningKind};
use crate::logic::account::AccountType;
use crate::logic::account::policy::ComplianceViolation;

/// Configurable business settings for a given account.
/// Each account starts with the default values for its type,
/// but these can be adjusted individually.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountContext {
    /// Account type — determines the validation logic
    pub account_type: AccountType,

    /// Overdraft allowed (0 = not allowed)
    pub overdraft_limit: Decimal,

    /// Minimum balance required (0 = none)
    pub min_balance: Decimal,

    /// Maximum number of transactions per month (None = unlimited)
    pub max_monthly_transactions: Option<u32>,

    /// For Deposit accounts: maturity date (withdrawals blocked prior to this date)
    pub deposit_locked_until: Option<NaiveDate>,

    /// Account earn interest
    pub allows_interest: bool,

    /// Account allow multiple joint account holders
    pub allows_joint_signers: bool,
}

impl AccountContext {
    /// Creates a context using the type's default values.
    pub fn from_type(account_type: AccountType) -> Self {
        match account_type {
            AccountType::Current => Self {
                account_type,
                overdraft_limit: dec!(500),
                min_balance: dec!(0),
                max_monthly_transactions: None,
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Cash => Self {
                account_type,
                overdraft_limit: dec!(500),
                min_balance: dec!(0),
                max_monthly_transactions: None,
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Income => Self {
                account_type,
                overdraft_limit: dec!(0),
                min_balance: dec!(0),
                max_monthly_transactions: None,
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Loan => Self {
                account_type,
                overdraft_limit: dec!(0),
                min_balance: dec!(0),
                max_monthly_transactions: None,
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Saving => Self {
                account_type,
                overdraft_limit: dec!(0),
                min_balance: dec!(10),
                max_monthly_transactions: Some(6),
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Joint => Self {
                account_type,
                overdraft_limit: dec!(1_000),
                min_balance: dec!(0),
                max_monthly_transactions: None,
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Deposit => Self {
                account_type,
                overdraft_limit: dec!(0),
                min_balance: dec!(0),
                max_monthly_transactions: None,
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Business => Self {
                account_type,
                overdraft_limit: dec!(10_000),
                min_balance: dec!(0),
                max_monthly_transactions: None,
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
            AccountType::Student => Self {
                account_type,
                overdraft_limit: dec!(100),
                min_balance: dec!(0),
                max_monthly_transactions: Some(30),
                deposit_locked_until: None,
                allows_interest: account_type.allows_interest(),
                allows_joint_signers: account_type.allows_joint_signers(),
            },
        }
    }

    /// Updates the account's configurable settings.
    /// Fields that do not apply to the account type are ignored with a warning.
    /// Returns a fatal error if a value violates a business rule (e.g. negative amount).
    /// None means no change for that field.
    pub fn update_context(
        &mut self,
        overdraft_limit: Option<Decimal>,
        min_balance: Option<Decimal>,
        max_monthly_transactions: Option<Option<u32>>, // Some(None) = no limit
        deposit_locked_until: Option<NaiveDate>,
        allows_interest: Option<bool>,
        allows_joint_signers: Option<bool>,
    ) -> Result<Vec<CoreWarning>, ComplianceViolation> {
        let mut warnings = Vec::new();

        // overdraft_limit — must be >= 0, not applicable for Saving, Deposit, Loan
        if let Some(limit) = overdraft_limit {
            if limit < Decimal::ZERO {
                return Err(ComplianceViolation::InvalidContextValue {
                    reason: "overdraft_limit must be zero or positive",
                });
            }
            match self.account_type {
                AccountType::Saving | AccountType::Deposit | AccountType::Loan | AccountType::Income => {
                    warnings.push(CoreWarning {
                        kind: CoreWarningKind::ContextNotApplicable,
                        message: format!(
                            "overdraft_limit is not applicable to {} account — ignored",
                            self.account_type
                        ),
                    });
                }
                _ => self.overdraft_limit = limit,
            }
        }

        // min_balance — must be >= 0, ignored when overdraft_limit > 0
        // since a negative balance is already allowed up to the overdraft limit
        if let Some(min) = min_balance {
            if min < Decimal::ZERO {
                return Err(ComplianceViolation::InvalidContextValue {
                    reason: "min_balance must be zero or positive",
                });
            }
            if self.overdraft_limit > Decimal::ZERO {
                warnings.push(CoreWarning {
                    kind: CoreWarningKind::ContextNotApplicable,
                    message: "min_balance is ignored when overdraft_limit > 0 — ignored".into(),
                });
            } else {
                self.min_balance = min;
            }
        }

        // max_monthly_transactions — applicable to all account types
        if let Some(max) = max_monthly_transactions {
            self.max_monthly_transactions = max;
        }

        // deposit_locked_until — only applicable to Deposit accounts
        if let Some(until) = deposit_locked_until {
            match self.account_type {
                AccountType::Deposit => self.deposit_locked_until = Some(until),
                _ => {
                    warnings.push(CoreWarning {
                        kind: CoreWarningKind::ContextNotApplicable,
                        message: format!(
                            "deposit_locked_until is not applicable to {} account — ignored",
                            self.account_type
                        ),
                    });
                }
            }
        }

        // allows_interest — only applicable to Saving, Deposit, Loan, Income accounts
        if let Some(value) = allows_interest {
            match self.account_type {
                AccountType::Current
                | AccountType::Saving
                | AccountType::Deposit
                | AccountType::Loan
                | AccountType::Income => self.allows_interest = value,
                _ => {
                    warnings.push(CoreWarning {
                        kind: CoreWarningKind::ContextNotApplicable,
                        message: format!(
                            "allows_interest is not applicable to {} account — ignored",
                            self.account_type
                        ),
                    });
                }
            }
        }

        // allows_joint_signers — only applicable to Joint and Business accounts
        if let Some(value) = allows_joint_signers {
            match self.account_type {
                AccountType::Joint | AccountType::Business => self.allows_joint_signers = value,
                _ => {
                    warnings.push(CoreWarning {
                        kind: CoreWarningKind::ContextNotApplicable,
                        message: format!(
                            "allows_joint_signers is not applicable to {} account — ignored",
                            self.account_type
                        ),
                    });
                }
            }
        }

        Ok(warnings)
    }

    pub fn has_saving_rate(&self) -> bool {
        matches!(
            self.account_type,
            AccountType::Current | AccountType::Joint | AccountType::Business
        )
    }
}

impl Default for AccountContext {
    fn default() -> Self {
        Self::from_type(AccountType::Current)
    }
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    // ── Helpers ──────────────────────────────────────────────

    fn ctx_current() -> AccountContext {
        AccountContext::from_type(AccountType::Current)
    }
    fn ctx_saving() -> AccountContext {
        AccountContext::from_type(AccountType::Saving)
    }
    fn ctx_deposit() -> AccountContext {
        AccountContext::from_type(AccountType::Deposit)
    }
    fn ctx_joint() -> AccountContext {
        AccountContext::from_type(AccountType::Joint)
    }
    fn ctx_business() -> AccountContext {
        AccountContext::from_type(AccountType::Business)
    }
    fn ctx_student() -> AccountContext {
        AccountContext::from_type(AccountType::Student)
    }

    // ── overdraft_limit ──────────────────────────────────────

    #[test]
    fn overdraft_limit_updated_on_current() {
        let mut ctx = ctx_current();
        let warnings = ctx
            .update_context(Some(dec!(2_000)), None, None, None, None, None)
            .unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.overdraft_limit, dec!(2_000));
    }

    #[test]
    fn overdraft_limit_negative_is_error() {
        let mut ctx = ctx_current();
        let res = ctx.update_context(Some(dec!(-100)), None, None, None, None, None);
        assert!(matches!(res, Err(ComplianceViolation::InvalidContextValue { .. })));
    }

    #[test]
    fn overdraft_limit_zero_is_valid() {
        let mut ctx = ctx_current();
        let warnings = ctx.update_context(Some(dec!(0)), None, None, None, None, None).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.overdraft_limit, dec!(0));
    }

    #[test]
    fn overdraft_limit_ignored_on_saving() {
        let mut ctx = ctx_saving();
        let original = ctx.overdraft_limit;
        let warnings = ctx
            .update_context(Some(dec!(500)), None, None, None, None, None)
            .unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].kind, CoreWarningKind::ContextNotApplicable));
        assert_eq!(ctx.overdraft_limit, original); // inchangé
    }

    #[test]
    fn overdraft_limit_ignored_on_deposit() {
        let mut ctx = ctx_deposit();
        let original = ctx.overdraft_limit;
        let warnings = ctx
            .update_context(Some(dec!(500)), None, None, None, None, None)
            .unwrap();
        assert_eq!(warnings.len(), 1);
        assert_eq!(ctx.overdraft_limit, original); // inchangé
    }

    // ── min_balance ──────────────────────────────────────────

    #[test]
    fn min_balance_updated_on_saving() {
        let mut ctx = ctx_saving();
        let warnings = ctx
            .update_context(None, Some(dec!(50)), None, None, None, None)
            .unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.min_balance, dec!(50));
    }

    #[test]
    fn min_balance_negative_is_error() {
        let mut ctx = ctx_saving();
        let res = ctx.update_context(None, Some(dec!(-10)), None, None, None, None);
        assert!(matches!(res, Err(ComplianceViolation::InvalidContextValue { .. })));
    }

    #[test]
    fn min_balance_ignored_when_overdraft_set() {
        let mut ctx = ctx_current(); // overdraft_limit = 500 par défaut
        let original = ctx.min_balance;
        let warnings = ctx
            .update_context(None, Some(dec!(100)), None, None, None, None)
            .unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].kind, CoreWarningKind::ContextNotApplicable));
        assert_eq!(ctx.min_balance, original); // inchangé
    }

    #[test]
    fn min_balance_zero_is_valid() {
        let mut ctx = ctx_saving();
        let warnings = ctx.update_context(None, Some(dec!(0)), None, None, None, None).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.min_balance, dec!(0));
    }

    // ── max_monthly_transactions ─────────────────────────────

    #[test]
    fn max_monthly_transactions_updated() {
        let mut ctx = ctx_current();
        let warnings = ctx
            .update_context(None, None, Some(Some(10)), None, None, None)
            .unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.max_monthly_transactions, Some(10));
    }

    #[test]
    fn max_monthly_transactions_removed() {
        let mut ctx = ctx_saving(); // saving a Some(6) par défaut
        let warnings = ctx.update_context(None, None, Some(None), None, None, None).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.max_monthly_transactions, None);
    }

    // ── deposit_locked_until ─────────────────────────────────

    #[test]
    fn deposit_locked_until_set_on_deposit() {
        let mut ctx = ctx_deposit();
        let date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
        let warnings = ctx.update_context(None, None, None, Some(date), None, None).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.deposit_locked_until, Some(date));
    }

    #[test]
    fn deposit_locked_until_ignored_on_current() {
        let mut ctx = ctx_current();
        let date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
        let warnings = ctx.update_context(None, None, None, Some(date), None, None).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].kind, CoreWarningKind::ContextNotApplicable));
        assert_eq!(ctx.deposit_locked_until, None); // inchangé
    }

    // ── allows_interest ──────────────────────────────────────

    #[test]
    fn allows_interest_set_on_saving() {
        let mut ctx = ctx_saving();
        let warnings = ctx.update_context(None, None, None, None, Some(false), None).unwrap();
        assert!(warnings.is_empty());
        assert!(!ctx.allows_interest);
    }

    #[test]
    fn allows_interest_on_current() {
        let mut ctx = ctx_current();
        let warnings = ctx.update_context(None, None, None, None, Some(true), None).unwrap();
        assert_eq!(warnings.len(), 0);
        assert!(ctx.allows_interest); // inchangé
    }

    // ── allows_joint_signers ─────────────────────────────────

    #[test]
    fn allows_joint_signers_set_on_joint() {
        let mut ctx = ctx_joint();
        let warnings = ctx.update_context(None, None, None, None, None, Some(false)).unwrap();
        assert!(warnings.is_empty());
        assert!(!ctx.allows_joint_signers);
    }

    #[test]
    fn allows_joint_signers_set_on_business() {
        let mut ctx = ctx_business();
        let warnings = ctx.update_context(None, None, None, None, None, Some(false)).unwrap();
        assert!(warnings.is_empty());
        assert!(!ctx.allows_joint_signers);
    }

    #[test]
    fn allows_joint_signers_ignored_on_student() {
        let mut ctx = ctx_student();
        let warnings = ctx.update_context(None, None, None, None, None, Some(true)).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].kind, CoreWarningKind::ContextNotApplicable));
        assert!(!ctx.allows_joint_signers); // inchangé
    }

    // ── multiple fields ──────────────────────────────────────

    #[test]
    fn multiple_warnings_on_wrong_type() {
        // Saving : overdraft + joint_signers → deux warnings
        let mut ctx = ctx_saving();
        let warnings = ctx
            .update_context(Some(dec!(500)), None, None, None, None, Some(true))
            .unwrap();
        assert_eq!(warnings.len(), 2);
    }

    #[test]
    fn none_fields_change_nothing() {
        let mut ctx = ctx_current();
        let original = ctx.clone();
        let warnings = ctx.update_context(None, None, None, None, None, None).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(ctx.overdraft_limit, original.overdraft_limit);
        assert_eq!(ctx.min_balance, original.min_balance);
        assert_eq!(ctx.max_monthly_transactions, original.max_monthly_transactions);
    }
}
