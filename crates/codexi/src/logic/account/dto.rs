// src/logic/account/dto.rs

use rust_decimal::Decimal;

use crate::core::{format_date, format_id, format_id_short};
use crate::logic::{
    account::{AccountAnchors, SearchItem},
    balance::BalanceItem,
    counts::Counts,
};

/*------------------------ OPERATION ENTRY ------------------------*/

/// Struct for Operation Entry
#[derive(Debug, Clone)]
pub struct OperationEntry {
    pub counts: Counts,
    pub items: Vec<OperationItem>,
}

#[derive(Debug, Clone)]
pub struct OperationItem {
    pub id: String,
    pub date: String,
    pub kind: String,
    pub flow: String,
    pub amount: Decimal,
    pub description: String,
    pub can_be_void: bool,
    pub void_by: Option<String>,
    pub void_of: Option<String>,
    pub balance: Decimal,
}

impl From<&SearchItem> for OperationItem {
    fn from(item: &SearchItem) -> Self {
        Self {
            id: item.operation.id.to_string(),
            date: format_date(item.operation.date),
            kind: item.operation.kind.as_str().to_string(),
            flow: item.operation.flow.as_str().to_string(),
            amount: item.operation.amount,
            description: item.operation.description.clone(),
            can_be_void: false, // always set by Account::operation_entry() — not available here
            void_by: item
                .operation
                .links
                .void_by
                .map(|id| format_id_short(&format_id(id))),
            void_of: item
                .operation
                .links
                .void_of
                .map(|id| format_id_short(&format_id(id))),
            balance: item.balance,
        }
    }
}

/*---------------------- ACCOUNT ANCHORS ITEM ---------------------*/

#[derive(Debug, Default, Clone)]
pub struct AccountAnchorsItem {
    pub last_regular: Option<String>,
    pub last_init: Option<String>,
    pub last_adjust: Option<String>,
    pub last_void: Option<String>,
    pub last_checkpoint: Option<String>,
}

impl From<&AccountAnchors> for AccountAnchorsItem {
    fn from(a: &AccountAnchors) -> Self {
        Self {
            last_regular: a.last_regular.clone().map(|la| format_date(la.date)),
            last_init: a.last_init.clone().map(|la| format_date(la.date)),
            last_adjust: a.last_adjust.clone().map(|la| format_date(la.date)),
            last_void: a.last_void.clone().map(|la| format_date(la.date)),
            last_checkpoint: a.last_checkpoint.clone().map(|la| format_date(la.date)),
        }
    }
}

/*-------------------------- SUMMARY ENTRY -------------------------*/

#[derive(Debug, Default, Clone)]
pub struct SummaryEntry {
    pub counts: Counts,
    pub balance: BalanceItem,
    pub anchors: AccountAnchorsItem,
}
