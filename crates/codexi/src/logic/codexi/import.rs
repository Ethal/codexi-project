// src/logic/codexi/import.rs

use crate::{
    core::CoreWarning,
    exchange::ImportSummary,
    logic::{
        account::Account,
        category::CategoryList,
        codexi::{Codexi, CodexiError},
        counterparty::CounterpartyList,
        currency::CurrencyList,
        operation::AccountOperations,
    },
};

impl Codexi {
    /// Import account header from json, toml, csv
    /// Return an import summary
    pub fn import_account_header(
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
}
