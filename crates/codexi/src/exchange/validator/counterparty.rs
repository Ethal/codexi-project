// src/excbnage/validator/currency.rs

use std::collections::HashSet;

use crate::core::{CoreWarning, parse_id};
use crate::exchange::{ExchangeCounterpartyList, ExchangeError};

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;

/// Exchange counterparty validation
pub fn validate_import_counterparty(
    import: &ExchangeCounterpartyList,
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

    for counterparty in &import.counterparties {
        // Validate id format if present — parse attempted here + check duplicate
        if let Some(raw_id) = &counterparty.id {
            let id = parse_id(raw_id)?;
            if !seen_ids.insert(id) {
                return Err(ExchangeError::DuplicateCounterparty(format!(
                    "Duplicate counterparty id {}",
                    id
                )));
            }
        }
    }

    Ok(warnings)
}
