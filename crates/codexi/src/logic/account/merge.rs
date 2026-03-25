// src/logic/account/merge.rs

use crate::exchange::ImportSummary;
use crate::logic::account::{Account, AccountError};

impl Account {
    /// Merges an imported account into self, respecting update rules.
    /// Validation is assumed to have been done upstream by validate_import().
    pub fn merge_from_import(&mut self, imported: Account) -> Result<ImportSummary, AccountError> {
        // Terminated accounts are immutable — reject any update
        self.is_terminated()?;

        // --- Account level update ---
        let mut summary = ImportSummary::default();

        self.update(
            &imported.name,
            imported.currency_id,
            imported.bank_id,
            imported.context,
            imported.meta,
        );

        summary.name = self.name.clone();
        summary.updated = 1;
        summary.created = 0;
        summary.total_processed = 1;

        Ok(summary)
    }
}
