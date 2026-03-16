// src/logic/account/dto.rs

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::core::format_date;
use crate::logic::{
    account::SearchItem, balance::BalanceItem, counts::Counts, operation::OperationFlow,
};

/// Struct for Operation Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationEntry {
    pub operation_count: String,
    pub items: Vec<OperationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationItem {
    pub id: String,
    pub date: String,
    pub kind: String,
    pub flow: String,
    pub amount: String,
    pub description: String,
    pub can_be_void: bool,
    pub void_by: Option<String>,
    pub void_of: Option<String>,
    pub balance: String,
}

impl From<&SearchItem> for OperationItem {
    fn from(item: &SearchItem) -> Self {
        Self {
            id: item.operation.id.to_string(),
            date: format_date(item.operation.date),
            kind: item.operation.kind.as_str().to_string(),
            flow: item.operation.flow.as_str().to_string(),
            amount: item.operation.amount.to_string(),
            description: item.operation.description.clone(),
            can_be_void: false,
            void_by: item.operation.links.void_by.map(|id| id.to_string()),
            void_of: item.operation.links.void_of.map(|id| id.to_string()),
            balance: item.balance.to_string(),
        }
    }
}

// StatementEntry
#[derive(Debug, Default, Clone)]
pub struct StatementEntry {
    pub account_number: String,
    pub account_name: String,
    pub account_bank: String,
    pub account_currency: String,
    pub date_min: String,
    pub date_max: String,
    pub balance: BalanceItem,
    pub counts: Counts,
    pub items: Vec<StatementItem>,
}

#[derive(Debug, Default, Clone)]
pub struct StatementItem {
    pub id: String,
    pub date: String,
    pub description: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
}

impl From<&SearchItem> for StatementItem {
    fn from(item: &SearchItem) -> Self {
        let (debit, credit) = match item.operation.flow {
            OperationFlow::Debit => (item.operation.amount, Decimal::ZERO),
            OperationFlow::Credit => (Decimal::ZERO, item.operation.amount),
            _ => (Decimal::ZERO, Decimal::ZERO),
        };
        Self {
            id: item.operation.id.to_string(),
            date: format_date(item.operation.date),
            description: item.operation.description.clone(),
            debit,
            credit,
            balance: item.balance,
        }
    }
}
