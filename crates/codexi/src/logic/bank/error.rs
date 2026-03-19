// src/logic/bank/error.rs

use thiserror::Error;

use crate::logic::utils::ResolveError;

/// Error type for Bank
#[derive(Debug, Error)]
pub enum BankError {
    #[error("SYS_BANK_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("DATA_BANK: Bank id not found: {0}")]
    BankNotFound(String),
    #[error("VAL_BANK_NAME: Bank name: {0}")]
    InvalidName(String),
    #[error("DATA_BANK: Multiple operations match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("DATA_BANK: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
}

impl ResolveError for BankError {
    fn not_found(input: String) -> Self {
        BankError::BankNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        BankError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        BankError::InvalidShortId(input, min)
    }
}
