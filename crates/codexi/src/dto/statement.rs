// src/dto/statement.rs

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

/// Global context for enriching data (banks, currencies, categories).
#[derive(Debug)]
pub struct CodexiContext {
    pub banks: BankCollection,
    pub currencies: CurrencyCollection,
    pub categories: CategoryCollection,
}

/// Represents a transaction formatted for a statement.
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
    fn from(op: &SearchOperation) -> Self {
        let (debit, credit) = match op.operation.flow {
            OperationFlow::Debit => (op.operation.amount, Decimal::ZERO),
            OperationFlow::Credit => (Decimal::ZERO, op.operation.amount),
            _ => (Decimal::ZERO, Decimal::ZERO),
        };
        Self {
            id: op.operation.id.to_string(),
            date: format_date(op.operation.date),
            description: op.operation.description.clone(),
            debit,
            credit,
            balance: op.balance,
        }
    }
}

/// Complete collection of a bank statement.
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
    /// Creates a `StatementCollection` from an account and a list of transactions.
    pub fn build(codexi: &Codexi, account: &Account, s_ops: &SearchOperationList) -> Self {
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
