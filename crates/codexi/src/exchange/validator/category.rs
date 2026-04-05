// src/excbnage/validator/currency.rs

use std::collections::HashSet;

use crate::{
    CODEXI_EXCHANGE_FORMAT_VERSION,
    core::{CoreWarning, parse_id, validate_text_rules},
    exchange::{ExchangeCategoryList, ExchangeError},
};

/// Exchange counterparty validation
pub fn validate_import_category(
    import: &ExchangeCategoryList,
) -> Result<Vec<CoreWarning>, ExchangeError> {
    // Version
    if import.version != CODEXI_EXCHANGE_FORMAT_VERSION {
        return Err(ExchangeError::InvalidVersion(format!(
            "Unsupported import version {}, expected {}",
            import.version, CODEXI_EXCHANGE_FORMAT_VERSION
        )));
    }

    let warnings = Vec::new();

    // Unique IDs
    let mut seen_ids = HashSet::new();

    for category in &import.list {
        // Validate id format if present — parse attempted here + check duplicate
        if let Some(raw_id) = &category.id {
            let id = parse_id(raw_id)?;
            if !seen_ids.insert(id) {
                return Err(ExchangeError::DuplicateCategory(format!(
                    "Duplicate category id {}",
                    id
                )));
            }
        }
        // Name comply with text rules
        let min = 3;
        let max = 20;
        if let Err(e) = validate_text_rules(&category.name, min, max) {
            return Err(ExchangeError::InvalidData(format!(
                "category name error: {}",
                e,
            )));
        }
    }

    Ok(warnings)
}
