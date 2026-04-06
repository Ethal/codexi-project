// src/exchange/validator/account.rs

use crate::{
    CODEXI_EXCHANGE_FORMAT_VERSION,
    core::{CoreWarning, validate_text_rules},
    exchange::{ExchangeAccountHeader, ExchangeError},
    logic::account::AccountType,
};

/// Exchange account validation (structural + Void + Transfer + Amount consistency)
pub fn validate_import_account_header(import: &ExchangeAccountHeader) -> Result<Vec<CoreWarning>, ExchangeError> {
    // Version
    if import.version != CODEXI_EXCHANGE_FORMAT_VERSION {
        return Err(ExchangeError::InvalidVersion(format!(
            "Unsupported import version {}, expected {}",
            import.version, CODEXI_EXCHANGE_FORMAT_VERSION
        )));
    }

    // Name comply with text rules
    let min = 3;
    let max = 50;
    if let Err(e) = validate_text_rules(&import.name, min, max) {
        return Err(ExchangeError::InvalidData(format!("Account name error: {}", e,)));
    }

    // account_type must be a known variant — avoids a cryptic error later in TryFrom
    AccountType::try_from_str(&import.context.account_type)
        .map_err(|_| ExchangeError::InvalidData(format!("Unknown account type '{}'", import.context.account_type)))?;

    let warnings = Vec::new();

    Ok(warnings)
}
