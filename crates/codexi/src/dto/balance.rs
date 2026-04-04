// src/dto/balance.rs

use rust_decimal::Decimal;

use crate::logic::balance::Balance;

#[derive(Debug, Default, Clone)]
pub struct BalanceItem {
    pub credit: Decimal,
    pub debit: Decimal,
    pub total: Decimal,
}

impl From<Balance> for BalanceItem {
    fn from(b: Balance) -> Self {
        BalanceItem {
            credit: b.credit,
            debit: b.debit,
            total: b.total(),
        }
    }
}
