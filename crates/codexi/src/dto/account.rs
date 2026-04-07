// src/dto/accunt.rs

use rust_decimal::Decimal;

use crate::{
    core::{format_id, format_optional_date},
    dto::{BalanceItem, BankItem, CurrencyItem},
    logic::{
        account::{Account, AccountContext},
        balance::Balance,
        codexi::Codexi,
    },
};

#[derive(Debug, Clone)]
pub struct AccountContextItem {
    pub account_type: String,
    pub overdraft_limit: Decimal,
    pub min_balance: Decimal,
    pub max_monthly_transactions: Option<u32>,
    pub deposit_locked_until: Option<String>,
    pub allows_interest: bool,
    pub allows_joint_signers: bool,
}

impl From<&AccountContext> for AccountContextItem {
    fn from(ac: &AccountContext) -> Self {
        Self {
            account_type: ac.account_type.as_str().to_string(),
            overdraft_limit: ac.overdraft_limit,
            min_balance: ac.min_balance,
            max_monthly_transactions: ac.max_monthly_transactions,
            deposit_locked_until: format_optional_date(ac.deposit_locked_until),
            allows_interest: ac.allows_interest,
            allows_joint_signers: ac.allows_joint_signers,
        }
    }
}

#[derive(Debug)]
pub struct AccountItem {
    pub id: String,
    pub name: String,
    pub current: bool,
    pub close: bool,
    pub bank: Option<BankItem>,
    pub currency: Option<CurrencyItem>,
    pub context: AccountContextItem,
    pub balance: BalanceItem,
    pub is_zero_balance_expected: bool,
}

impl AccountItem {
    pub fn build(codexi: &Codexi, account: &Account) -> Self {
        let currency = account
            .currency_id
            .and_then(|id| codexi.currencies.get_by_id(&id).ok())
            .map(CurrencyItem::from);

        let bank = account
            .bank_id
            .and_then(|id| codexi.banks.get_by_id(&id).ok())
            .map(BankItem::from);

        Self {
            id: format_id(account.id),
            name: account.name.clone(),
            current: account.id == codexi.current_account,
            close: account.terminated_date.is_some(),
            bank,
            currency,
            context: AccountContextItem::from(&account.context),
            balance: BalanceItem::from(Balance::for_account(account)),
            is_zero_balance_expected: account.context.account_type.is_zero_balance_expected(),
        }
    }
}

#[derive(Debug)]
pub struct AccountCollection {
    pub items: Vec<AccountItem>,
}

impl AccountCollection {
    pub fn build(codexi: &Codexi) -> AccountCollection {
        let items = codexi
            .accounts
            .iter()
            .map(|acc| AccountItem::build(codexi, acc))
            .collect();

        Self { items }
    }
}
