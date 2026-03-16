// src/logic/balance/dto.rs

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceItem {
    pub credit: Decimal,
    pub debit: Decimal,
}

impl BalanceItem {
    pub fn total(&self) -> Decimal {
        self.credit - self.debit
    }
}
