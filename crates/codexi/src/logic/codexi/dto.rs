// src/logic/codexi/dto.rs

use rust_decimal::Decimal;

use crate::core::format_date;
use crate::logic::{
    account::{AccountContextItem, SearchItem},
    balance::BalanceItem,
    bank::BankEntry,
    category::CategoryEntry,
    counts::Counts,
    currency::CurrencyEntry,
    operation::OperationFlow,
};

#[derive(Debug, Default, Clone)]
pub struct AccountEntry {
    pub items: Vec<AccountItem>,
}

#[derive(Debug, Default, Clone)]
pub struct AccountItem {
    pub id: String,
    pub name: String,
    pub current: bool,
    pub close: bool,
    pub bank: String,
    pub currency: String,
    pub context: AccountContextItem,
}

#[derive(Debug)]
pub struct CodexiContext {
    pub banks: BankEntry,
    pub currencies: CurrencyEntry,
    pub categories: CategoryEntry,
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

/// Detailed view of a single operation — all referenced fields resolved
/// (currency name, category name, etc.). Built by Codexi::operation_detail().
#[derive(Debug, Default, Clone)]
pub struct OperationDetailItem {
    // ── Identity ─────────────────────────────────────────────
    pub id: String,
    pub date: String,
    pub kind: String,
    pub flow: String,
    pub amount: Decimal,
    pub balance: Decimal,
    pub description: String,
    pub can_be_void: bool,

    // ── Links ─────────────────────────────────────────────────
    pub void_of: Option<String>,
    pub void_by: Option<String>,
    pub transfer_id: Option<String>,
    pub transfer_account_id: Option<String>,

    // ── Context — resolved ────────────────────────────────────
    pub currency: String, // resolved from currency_id e.g. "IDR"
    pub exchange_rate: Decimal,
    pub category: String,           // resolved from category_id or "—"
    pub payee: Option<String>,      // payee name or "—"
    pub reconciled: Option<String>, // formatted date or "—"

    // ── Meta ──────────────────────────────────────────────────
    pub tags: String,         // comma-separated or "—"
    pub note: Option<String>, // note or "—"
    pub attachment: String,   // path or "—"
}
