// src/logic/balance/dto.rs

use rust_decimal::Decimal;

use crate::{
    core::format_id,
    logic::balance::{AccountBalance, Balance},
};

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

#[derive(Debug, Default, Clone)]
pub struct AccountBalanceItem {
    pub id: String,
    pub name: String,
    pub balance: BalanceItem,
}

impl From<AccountBalance> for AccountBalanceItem {
    fn from(b: AccountBalance) -> Self {
        AccountBalanceItem {
            id: format_id(b.id),
            name: b.name.clone(),
            balance: BalanceItem::from(b.balance),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CodexiBalanceEntry {
    pub balances: Vec<AccountBalanceItem>,
}
