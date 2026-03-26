// src/logic/account/policy/dto.rs

use rust_decimal::Decimal;

use crate::{
    core::{format_max_monthly_transactions, format_optional_date, yes_no},
    logic::account::policy::AccountContext,
};

#[derive(Debug, Default, Clone)]
pub struct AccountContextItem {
    pub account_type: String,
    pub overdraft_limit: Decimal,
    pub min_balance: Decimal,
    pub max_monthly_transactions: String,
    pub deposit_locked_until: Option<String>,
    pub allows_interest: String,
    pub allows_joint_signers: String,
}

impl From<&AccountContext> for AccountContextItem {
    fn from(ac: &AccountContext) -> Self {
        Self {
            account_type: ac.account_type.as_str().to_string(),
            overdraft_limit: ac.overdraft_limit,
            min_balance: ac.min_balance,
            max_monthly_transactions: format_max_monthly_transactions(ac.max_monthly_transactions),
            deposit_locked_until: format_optional_date(ac.deposit_locked_until),
            allows_interest: yes_no(ac.allows_interest),
            allows_joint_signers: yes_no(ac.allows_joint_signers),
        }
    }
}
