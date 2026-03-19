// src/logic/currency/error.rs

use thiserror::Error;

use crate::logic::utils::ResolveError;

/// Error type for Currency
#[derive(Debug, Error)]
pub enum CurrencyError {
    #[error("SYS_CURRENCY_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("DATA_CURRENCY: Currency id not found: {0}")]
    CurrencyNotFound(String),
    #[error("VAL_CURRENCY_CODE: Code: {0} invalid")]
    InvalidCode(String),
    #[error("DATA_BANK: Multiple operations match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("DATA_BANK: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
}

impl ResolveError for CurrencyError {
    fn not_found(input: String) -> Self {
        CurrencyError::CurrencyNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        CurrencyError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        CurrencyError::InvalidShortId(input, min)
    }
}
