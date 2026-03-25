// src/excbnage/validator/currency.rs

use std::collections::HashSet;

use crate::core::{CoreWarning, parse_id};
use crate::exchange::{ExchangeCurrencyList, ExchangeError};

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;

/// Exchange currency validation
pub fn validate_import_currency(
    import: &ExchangeCurrencyList,
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

    for currency in &import.currencies {
        // Validate id format if present — parse attempted here + check duplicate
        if let Some(raw_id) = &currency.id {
            let id = parse_id(raw_id)?;
            if !seen_ids.insert(id) {
                return Err(ExchangeError::DuplicateCurrency(format!(
                    "Duplicate currency id {}",
                    id
                )));
            }
        }

        // check currency code as per Currency::new()
        if currency.code.len() < 3 || currency.code.len() > 3 {
            return Err(ExchangeError::InvalidData(format!(
                "Currency code '{}' must be exactly 3 characters (ISO 4217)",
                currency.code
            )));
        }
    }

    Ok(warnings)
}
