// src/logic/codexi/dto.rs

use serde::{Deserialize, Serialize};

use crate::logic::account::AccountContextItem;
use crate::logic::bank::BankEntry;
use crate::logic::category::CategoryEntry;
use crate::logic::currency::CurrencyEntry;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountItem {
    pub id: String,
    pub name: String,
    pub current: bool,
    pub close: bool,
    pub bank: String,
    pub currency: String,
    pub context: AccountContextItem,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountEntry {
    pub items: Vec<AccountItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexiContext {
    pub banks: BankEntry,
    pub currencies: CurrencyEntry,
    pub categories: CategoryEntry,
}
