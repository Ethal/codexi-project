// src/logic/category/merge.rs

use nulid::Nulid;
use std::collections::HashMap;

use crate::exchange::ImportSummary;
use crate::logic::category::{Category, CategoryError, CategoryList};

impl CategoryList {
    /// Merges an imported categories into self, respecting update rules.
    /// Validation is assumed to have been done upstream by validate_import().
    pub fn merge_from_import(
        &mut self,
        imported: CategoryList,
    ) -> Result<ImportSummary, CategoryError> {
        let mut summary = self.merge_categories(imported.list)?;
        summary.name = "Categories".into();
        Ok(summary)
    }

    fn merge_categories(
        &mut self,
        imported_category: Vec<Category>,
    ) -> Result<ImportSummary, CategoryError> {
        let mut summary = ImportSummary::default();
        let mut to_add = Vec::new();
        let mut to_update = Vec::new();

        {
            let existing_by_id: HashMap<Nulid, &Category> =
                self.list.iter().map(|c| (c.id, c)).collect();

            for category in imported_category {
                summary.total_processed += 1;
                if existing_by_id.contains_key(&category.id) {
                    to_update.push(category);
                } else if self
                    .list
                    .iter()
                    .any(|c| c.name.to_lowercase() == category.name.to_lowercase())
                {
                    // Name already exists with a different id — reject the whole import
                    return Err(CategoryError::DuplicateName(category.name));
                } else {
                    to_add.push(category);
                }
            }
        }

        for category in to_update {
            self.update(
                category.id,
                &category.name,
                category.parent_id,
                category.terminated,
                category.note.as_deref(),
            )
            .expect("category id confirmed above");
            summary.updated += 1;
        }

        for category in to_add {
            self.add(category);
            summary.created += 1;
        }

        Ok(summary)
    }
}
