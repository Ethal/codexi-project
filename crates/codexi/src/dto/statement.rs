// src/dto/staement.rs

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::{
    core::format_date,
    dto::{
        AccountItem, BalanceItem, BankCollection, CategoryCollection, CurrencyCollection,
        SearchOperationItem,
    },
    logic::{
        account::Account,
        balance::Balance,
        codexi::Codexi,
        counts::Counts,
        operation::OperationFlow,
        search::{SearchOperation, SearchOperationList},
    },
};

#[derive(Debug)]
pub struct CodexiContext {
    pub banks: BankCollection,
    pub currencies: CurrencyCollection,
    pub categories: CategoryCollection,
}

#[derive(Debug)]
pub struct StatementItem {
    pub id: String,
    pub date: String,
    pub description: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
}

impl From<&SearchOperation> for StatementItem {
    fn from(item: &SearchOperation) -> Self {
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

// StatementCollection
#[derive(Debug)]
pub struct StatementCollection {
    pub account: AccountItem,
    pub from: String,
    pub to: String,
    pub counts: Counts,
    pub balance: BalanceItem,
    pub items: Vec<SearchOperationItem>,
}

impl StatementCollection {
    /// Builds a StatementEntry for the given account, enriched with bank and currency
    /// names from the Codexi context. Returns None if the account is not found.
    pub fn build(codexi: &Codexi, account: &Account, s_ops: &SearchOperationList) -> Self {
        // date min/max
        let (date_min, date_max) = find_date_range(s_ops)
            .map(|(min, max)| (format_date(min), format_date(max)))
            .unwrap_or(("N/A".into(), "N/A".into()));

        let min = match s_ops.params.from {
            Some(v) => format_date(v),
            None => date_min,
        };
        let max = match s_ops.params.to {
            Some(v) => format_date(v),
            None => date_max,
        };

        Self {
            account: AccountItem::build(codexi, account),
            from: min,
            to: max,
            counts: Counts::new(s_ops),
            balance: BalanceItem::from(Balance::build(s_ops)),
            items: s_ops
                .iter()
                .map(|s_op| SearchOperationItem::build(codexi, account, s_op))
                .collect(),
        }
    }
}

/*-------------------------- HELPER -------------------------*/

/// Returns the min and max operation dates from a search result.
fn find_date_range(items: &SearchOperationList) -> Option<(NaiveDate, NaiveDate)> {
    let mut iter = items.iter().map(|i| i.operation.date);
    let first = iter.next()?;
    let (min, max) = iter.fold((first, first), |(min, max), d| (min.min(d), max.max(d)));
    Some((min, max))
}
