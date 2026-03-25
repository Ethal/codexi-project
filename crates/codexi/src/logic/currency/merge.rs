// src/logic/currency/merge.rs

use nulid::Nulid;
use std::collections::HashMap;

use crate::exchange::ImportSummary;
use crate::logic::currency::{Currency, CurrencyError, CurrencyList};

impl CurrencyList {
    /// Merges an imported currencies into self, respecting update rules.
    /// Validation is assumed to have been done upstream by validate_import().
    pub fn merge_from_import(
        &mut self,
        imported: CurrencyList,
    ) -> Result<ImportSummary, CurrencyError> {
        let mut summary = self.merge_currencies(imported.currencies)?;
        summary.name = "Currencies".into();
        Ok(summary)
    }

    fn merge_currencies(
        &mut self,
        imported_currency: Vec<Currency>,
    ) -> Result<ImportSummary, CurrencyError> {
        let mut summary = ImportSummary::default();
        let mut to_add = Vec::new();
        let mut to_update = Vec::new();

        {
            let existing_by_id: HashMap<Nulid, &Currency> =
                self.currencies.iter().map(|c| (c.id, c)).collect();

            for currency in imported_currency {
                summary.total_processed += 1;
                if existing_by_id.contains_key(&currency.id) {
                    to_update.push(currency);
                } else if self.currencies.iter().any(|c| c.code == currency.code) {
                    // Code already exists with a different id — reject the whole import
                    return Err(CurrencyError::DuplicateCode(currency.code));
                } else {
                    to_add.push(currency);
                }
            }
        }

        for currency in to_update {
            self.update(currency.id, &currency.symbol, currency.note.as_deref())
                .expect("currency id confirmed above");
            summary.updated += 1;
        }

        for currency in to_add {
            self.add(currency);
            summary.created += 1;
        }

        Ok(summary)
    }
}
