// src/logic/codexi/model.rs

use chrono::{Local, NaiveDate};
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::{format_date, format_id};
use crate::exchange::ImportSummary;
use crate::logic::account::Account;
use crate::logic::bank::BankList;
use crate::logic::category::CategoryList;
use crate::logic::codexi::{
    AccountEntry, AccountItem, CodexiError, CodexiSettings, default_banks, default_categories,
    default_currencies,
};
use crate::logic::currency::CurrencyList;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Codexi {
    pub id: Nulid,
    pub name: String,
    pub settings: CodexiSettings, // Preferences (language, theme...)
    pub current_account: Nulid,

    // Dictionnary
    pub categories: CategoryList,
    pub banks: BankList,
    pub currencies: CurrencyList,

    pub note: Option<String>,

    // The accounts
    pub accounts: Vec<Account>,
}

impl Codexi {
    pub fn new(settings: CodexiSettings) -> Result<Self, CodexiError> {
        let id = Nulid::new()?;

        // Seed data as per language settings
        let categories = CategoryList::from(default_categories(&settings.language));
        let currencies = CurrencyList::from(default_currencies());
        let banks = BankList::from(default_banks());

        let name = "Codexi".into();
        let accounts = Vec::new();

        Ok(Self {
            id,
            name,
            settings,
            categories,
            banks,
            currencies,
            accounts,
            current_account: Nulid::nil(),
            note: None,
        })
    }
    pub fn get_account_by_id(&self, id: &Nulid) -> Result<&Account, CodexiError> {
        self.accounts
            .iter()
            .find(|c| &c.id == id)
            .ok_or_else(|| CodexiError::AccountNotFound(format_id(*id)))
    }
    pub fn get_account_by_id_mut(&mut self, id: &Nulid) -> Result<&mut Account, CodexiError> {
        self.accounts
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or_else(|| CodexiError::AccountNotFound(format_id(*id)))
    }
    pub fn set_current_account(&mut self, id: &Nulid) -> Result<(), CodexiError> {
        if self.accounts.iter().any(|a| &a.id == id) {
            self.current_account = *id;
            Ok(())
        } else {
            Err(CodexiError::AccountNotFound(format_id(*id)))
        }
    }
    pub fn get_current_account(&self) -> Result<&Account, CodexiError> {
        self.accounts
            .iter()
            .find(|c| c.id == self.current_account)
            .ok_or_else(|| CodexiError::AccountNotFound(format_id(self.current_account)))
    }
    pub fn get_current_account_mut(&mut self) -> Result<&mut Account, CodexiError> {
        self.accounts
            .iter_mut()
            .find(|c| c.id == self.current_account)
            .ok_or_else(|| CodexiError::AccountNotFound(format_id(self.current_account)))
    }

    /// Set the bank id to the current account
    pub fn set_account_bank(&mut self, id: &Nulid) -> Result<(), CodexiError> {
        self.banks.get_by_id(id)?;
        let acc = self.get_current_account_mut()?;
        acc.bank_id = Some(*id);
        Ok(())
    }
    /// Set a currency id to the current account
    pub fn set_account_currency(&mut self, id: &Nulid) -> Result<(), CodexiError> {
        self.currencies.get_by_id(id)?;
        let acc = self.get_current_account_mut()?;
        acc.currency_id = Some(*id);
        Ok(())
    }
    /// Close an account
    /// Allowed if not the latest open account, current account, close date below the open_date
    pub fn close_account(&mut self, id: Nulid, date: NaiveDate) -> Result<(), CodexiError> {
        if self.account_count() <= 1 {
            return Err(CodexiError::OnlyOneAccount);
        }
        let account = self.get_account_by_id_mut(&id)?;
        if account.id == id {
            return Err(CodexiError::CloseCurentAccount);
        }
        if account.open_date < date || date > Local::now().date_naive() {
            return Err(CodexiError::CloseDateAccount(format_date(
                account.open_date,
            )));
        }
        account.terminated_date = Some(date);
        Ok(())
    }

    /// Add an account
    /// Return the id of the new account
    pub fn add_account(&mut self, account: Account) -> Nulid {
        let id = account.id;
        self.current_account = id;
        self.accounts.push(account);
        id
    }

    /// Import account from json, toml, csv
    /// Return a import summary
    pub fn import_account(
        &mut self,
        imported_account: Account,
    ) -> Result<ImportSummary, CodexiError> {
        let id = imported_account.id;
        let mut summary = ImportSummary::default();

        if let Ok(existing) = self.get_account_by_id_mut(&id) {
            // The account exist: update / create
            summary = existing.merge_from_import(imported_account)?;

            Ok(summary)
        } else {
            // new account
            let count = imported_account.operations.len();
            self.add_account(imported_account);
            summary.created = count;
            summary.total_processed = count;
            Ok(summary)
        }
    }

    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    pub fn account_entry(&self) -> AccountEntry {
        let items = self
            .accounts
            .iter()
            .map(|acc| self.account_item(acc))
            .collect();

        AccountEntry { items }
    }
    pub fn account_item(&self, acc: &Account) -> AccountItem {
        let mut item = AccountItem::default();
        item.id = format_id(acc.id);
        item.name = acc.name.clone();
        item.current = acc.id == self.current_account;
        item.close = acc.terminated_date.is_some();
        if let Some(id) = acc.bank_id
            && let Ok(bank) = self.banks.get_by_id(&id)
        {
            item.bank = bank.name.clone();
        }
        if let Some(id) = acc.currency_id
            && let Ok(currency) = self.currencies.get_by_id(&id)
        {
            item.currency = currency.code.clone();
        }
        item
    }
}
