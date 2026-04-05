// src/logic/codexi/model.rs

use chrono::NaiveDate;
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::{
    core::{CoreWarning, format_id},
    exchange::ImportSummary,
    logic::{
        account::{Account, AccountError},
        bank::BankList,
        category::CategoryList,
        codexi::{
            CodexiError, CodexiSettings, default_banks, default_categories, default_counterparties,
            default_currencies,
        },
        counterparty::CounterpartyList,
        currency::CurrencyList,
        operation::AccountOperations,
    },
};

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
    #[serde(default)]
    pub counterparties: CounterpartyList,

    pub note: Option<String>,

    // The accounts
    pub accounts: Vec<Account>,
}

impl Codexi {
    pub fn new(settings: CodexiSettings) -> Result<Self, CodexiError> {
        let id = Nulid::new()?;

        // Seed data as per language settings
        let categories = CategoryList::from(default_categories(&settings.language));
        let counterparties = CounterpartyList::from(default_counterparties(&settings.language));
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
            counterparties,
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
        if self.current_account == Nulid::nil() {
            return Err(CodexiError::NoCurrentAccount);
        }
        self.accounts
            .iter()
            .find(|c| c.id == self.current_account)
            .ok_or_else(|| CodexiError::AccountNotFound(format_id(self.current_account)))
    }
    pub fn get_current_account_mut(&mut self) -> Result<&mut Account, CodexiError> {
        if self.current_account == Nulid::nil() {
            return Err(CodexiError::NoCurrentAccount);
        }
        self.accounts
            .iter_mut()
            .find(|c| c.id == self.current_account)
            .ok_or_else(|| CodexiError::AccountNotFound(format_id(self.current_account)))
    }

    /// Set the bank id to the current account
    pub fn set_account_bank(&mut self, id: &Nulid) -> Result<(), CodexiError> {
        self.banks.get_by_id(id)?;
        let acc = self.get_current_account_mut()?;
        acc.is_terminated()
            .map_err(AccountError::TemporalViolation)?;
        acc.bank_id = Some(*id);
        Ok(())
    }
    /// Set a currency id to the current account
    pub fn set_account_currency(
        &mut self,
        id: &Nulid,
        update_operation: bool,
    ) -> Result<(), CodexiError> {
        self.currencies.get_by_id(id)?;
        let acc = self.get_current_account_mut()?;
        acc.is_terminated()
            .map_err(AccountError::TemporalViolation)?;
        acc.currency_id = Some(*id);
        if update_operation {
            for op in acc.operations.iter_mut() {
                op.context.currency_id = Some(*id);
            }
        }
        Ok(())
    }
    /// Close an account
    /// Allowed if not the latest open account, current account, close date below the open_date
    pub fn close_account(&mut self, id: Nulid, date: NaiveDate) -> Result<(), CodexiError> {
        // Life cycle rules
        let account = self.get_account_by_id_mut(&id)?;
        account
            .validate_close_date(date)
            .map_err(AccountError::LifecycleViolation)?;

        // Apply the terminated date
        account.terminated_date = Some(date);

        // codexi rules
        // Switch current_account to nil if it was the current account
        if id == self.current_account {
            self.current_account = Nulid::nil();
        }

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
    /// Return an import summary
    pub fn import_account(
        &mut self,
        imported_account: Account,
    ) -> Result<ImportSummary, CodexiError> {
        let id = imported_account.id;
        let mut summary = ImportSummary::default();

        if let Ok(existing) = self.get_account_by_id_mut(&id) {
            // Existing account — merge then refresh anchors
            summary = existing.merge_account_header_from_import(imported_account)?;
            existing.refresh_anchors(); // ← always recalculate after merge

            Ok(summary)
        } else {
            // New account
            let mut new_account = Account::new(
                imported_account.open_date,
                imported_account.name.clone(),
                imported_account.context.account_type,
                imported_account.bank_id,
                imported_account.currency_id,
            )?;

            // update context, meta, recalculate anchors, audit
            summary.name = new_account.name.clone();
            new_account.update_meta(imported_account.meta);
            new_account.update_context(imported_account.context);
            new_account.refresh_anchors(); //
            new_account.audit()?;
            self.add_account(new_account);

            summary.created = 1;
            summary.total_processed = 1;
            Ok(summary)
        }
    }
    /// Import currencies from json, toml, csv
    /// Return an import summary
    pub fn import_currencies(
        &mut self,
        imported_currencies: CurrencyList,
    ) -> Result<ImportSummary, CodexiError> {
        let summary = self.currencies.merge_from_import(imported_currencies)?;
        Ok(summary)
    }
    /// Import counterparties from json, toml, csv
    /// Return an import summary
    pub fn import_counterparties(
        &mut self,
        imported_counterparties: CounterpartyList,
    ) -> Result<ImportSummary, CodexiError> {
        let summary = self
            .counterparties
            .merge_from_import(imported_counterparties)?;
        Ok(summary)
    }
    /// Import categories from json, toml, csv
    /// Return an import summary
    pub fn import_categories(
        &mut self,
        imported_categories: CategoryList,
    ) -> Result<ImportSummary, CodexiError> {
        let summary = self.categories.merge_from_import(imported_categories)?;
        Ok(summary)
    }
    /// Import operations to an acccount from json, toml, csv
    /// Return an import summary
    pub fn import_operations(
        &mut self,
        imported_operations: AccountOperations,
    ) -> Result<(ImportSummary, Vec<CoreWarning>), CodexiError> {
        let account = self.get_account_by_id_mut(&imported_operations.account_id)?;
        let (summary, warnings) = account.merge_operation_from_import(&imported_operations)?;
        // Recalculate anchors and balances after merge
        account.refresh_anchors();
        Ok((summary, warnings))
    }

    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }
}
