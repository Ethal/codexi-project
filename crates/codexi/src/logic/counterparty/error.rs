// src/logic/counterparty/error.rs

use thiserror::Error;

use crate::logic::utils::ResolveError;

/// Error type for Counterparty
#[derive(Debug, Error)]
pub enum CounterpartyError {
    #[error("SYS_COUNTERPARTY_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("DATA_COUNTERPARTY: Counterparty id not found: {0}")]
    CounterpartyNotFound(String),
    #[error("VAL_COUNTERPARTY_NAME: Counterparty name: {0}")]
    InvalidName(String),
    #[error("DATA_COUNTERPARTY: Multiple counterparties match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("DATA_COUNTERPARTY: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
    #[error("VAL_COUNTERPARTY: Duplicate counterparty name {0}")]
    DuplicateName(String),
}

/// Error type for Counterparty Kind
#[derive(Debug, Error)]
pub enum CounterpartyKindError {
    #[error("OP_INVALID_COUNTERPARTY_KIND: Unknown counterparty Kind: {0}")]
    Unknown(String),
}

impl ResolveError for CounterpartyError {
    fn not_found(input: String) -> Self {
        CounterpartyError::CounterpartyNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        CounterpartyError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        CounterpartyError::InvalidShortId(input, min)
    }
}
