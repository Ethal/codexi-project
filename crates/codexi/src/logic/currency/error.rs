// src/logic/currency/error.rs

use thiserror::Error;

/// Error type for Currency
#[derive(Debug, Error)]
pub enum CurrencyError {
    #[error("SYS_CURRENCY_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("DATA_CURRENCY: Currency id not found: {0}")]
    CurrencyNotFound(String),
    #[error("VAL_CURRENCY_CODE: Code: {0} invalid")]
    InvalidCode(String),
}
