// src/dto/staement.rs

use rust_decimal::Decimal;

use crate::{
    core::format_date,
    dto::{AccountItem, BalanceItem, BankCollection, CategoryCollection, CurrencyCollection, SearchOperationItem},
    logic::{
        account::Account,
        balance::Balance,
        codexi::Codexi,
        counts::Counts,
        operation::OperationFlow,
        search::{SearchOperation, SearchOperationList},
    },
    types::DateRange,
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
    pub from: Option<String>,
    pub to: Option<String>,
    pub counts: Counts,
    pub balance: BalanceItem,
    pub items: Vec<SearchOperationItem>,
}

impl StatementCollection {
    /// Builds a StatementEntry for the given account, enriched with bank and currency
    /// names from the Codexi context. Returns None if the account is not found.
    pub fn build(codexi: &Codexi, account: &Account, s_ops: &SearchOperationList) -> Self {
        // date min/max

        let (from, to) = DateRange::compute(s_ops, s_ops.params.from, s_ops.params.to).formatted();

        Self {
            account: AccountItem::build(codexi, account),
            from,
            to,
            counts: Counts::new(s_ops),
            balance: BalanceItem::from(Balance::build(s_ops)),
            items: s_ops
                .iter()
                .map(|s_op| SearchOperationItem::build(codexi, account, s_op))
                .collect(),
        }
    }
}
