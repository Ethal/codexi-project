// src/logic/account/policy/dto.rs

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountContextItem {
    pub account_type: String,
    pub overdraft_limit: Decimal,
    pub min_balance: Decimal,
    pub max_monthly_transactions: u32,
    pub deposit_locked_until: String,
    pub allows_interest: String,
    pub allows_joint_signers: String,
}
