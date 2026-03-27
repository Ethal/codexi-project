// src/logic/balance/model.rs

use nulid::Nulid;
use rust_decimal::Decimal;

use crate::logic::{
    account::{Account, SearchEntry},
    balance::{AccountBalanceItem, CodexiBalanceEntry},
    codexi::Codexi,
    operation::OperationFlow,
};

#[derive(Debug, Default, Clone)]
pub struct Balance {
    pub credit: Decimal,
    pub debit: Decimal,
}

#[derive(Debug, Default, Clone)]
pub struct AccountBalance {
    pub id: Nulid,
    pub name: String,
    pub balance: Balance,
}

#[derive(Debug, Default, Clone)]
pub struct CodexiBalance {
    pub balances: Vec<AccountBalance>,
}

impl Balance {
    /// Calculates the total of credits, debits,
    /// with several date filters (from/to/day/month/year).
    /// Returns a Balance struct.
    pub fn new(items: &SearchEntry) -> Self {
        let mut credit = Decimal::ZERO;
        let mut debit = Decimal::ZERO;

        for item in items.iter() {
            match item.operation.flow {
                OperationFlow::Credit => credit += item.operation.amount,
                OperationFlow::Debit => debit += item.operation.amount,
                OperationFlow::None => {}
            }
        }
        Self { credit, debit }
    }
    pub fn total(&self) -> Decimal {
        self.credit - self.debit
    }
    pub fn codexi_balance(codexi: &Codexi) -> CodexiBalance {
        let mut balances = Vec::new();
        for account in &codexi.accounts {
            let account_bal = Self::account_balance(account);
            balances.push(account_bal);
        }
        CodexiBalance { balances }
    }

    pub fn account_balance(account: &Account) -> AccountBalance {
        let mut credit = Decimal::ZERO;
        let mut debit = Decimal::ZERO;
        for op in &account.operations {
            match op.flow {
                OperationFlow::Credit => credit += op.amount,
                OperationFlow::Debit => debit += op.amount,
                OperationFlow::None => {}
            }
        }
        AccountBalance {
            id: account.id,
            name: account.name.clone(),
            balance: Balance { credit, debit },
        }
    }

    pub fn codexi_balance_entry(codexi: &Codexi) -> CodexiBalanceEntry {
        let bal_codexi = Self::codexi_balance(codexi);
        let mut bal_codexi_entry = Vec::new();
        for bal in bal_codexi.balances {
            let bal_acc = AccountBalanceItem::from(bal);
            bal_codexi_entry.push(bal_acc);
        }
        CodexiBalanceEntry {
            balances: bal_codexi_entry,
        }
    }
}
