// src/logic/counterparty/merge.rs

use nulid::Nulid;
use std::collections::HashMap;

use crate::exchange::ImportSummary;
use crate::logic::counterparty::{Counterparty, CounterpartyError, CounterpartyList};

impl CounterpartyList {
    /// Merges an imported counterparties into self, respecting update rules.
    /// Validation is assumed to have been done upstream by validate_import().
    pub fn merge_from_import(
        &mut self,
        imported: CounterpartyList,
    ) -> Result<ImportSummary, CounterpartyError> {
        let mut summary = self.merge_counterparties(imported.list)?;
        summary.name = "Counterparties".into();
        Ok(summary)
    }

    fn merge_counterparties(
        &mut self,
        imported_counterparty: Vec<Counterparty>,
    ) -> Result<ImportSummary, CounterpartyError> {
        let mut summary = ImportSummary::default();
        let mut to_add = Vec::new();
        let mut to_update = Vec::new();

        {
            let existing_by_id: HashMap<Nulid, &Counterparty> =
                self.list.iter().map(|c| (c.id, c)).collect();

            for counterparty in imported_counterparty {
                summary.total_processed += 1;
                if existing_by_id.contains_key(&counterparty.id) {
                    to_update.push(counterparty);
                } else if self
                    .list
                    .iter()
                    .any(|c| c.name.to_lowercase() == counterparty.name.to_lowercase())
                {
                    // Name already exists with a different id — reject the whole import
                    return Err(CounterpartyError::DuplicateName(counterparty.name));
                } else {
                    to_add.push(counterparty);
                }
            }
        }

        for counterparty in to_update {
            self.update(
                counterparty.id,
                &counterparty.name,
                counterparty.kind,
                counterparty.note.as_deref(),
            )
            .expect("counterparty id confirmed above");
            summary.updated += 1;
        }

        for counterparty in to_add {
            self.add(counterparty);
            summary.created += 1;
        }

        Ok(summary)
    }
}
