// src/logic/search/error.rs

use thiserror::Error;

use crate::core::CoreError;
use crate::logic::utils::ResolveError;

/// Error type for search in account
#[derive(Debug, Error)]
pub enum SearchError {
    #[error("SRCH_INVALID_DATE: {0}")]
    InvalidDate(String),
    #[error("SYS_COMMON: {0}")]
    Common(#[from] CoreError),
    #[error("SRCH_BUILD: search parameters build: {0}")]
    SearchParametersBuilder(String),
    #[error("SRCH_VAL: Operation #{0} not found in search item")]
    OperationNotFound(String),
    #[error("SRCH_VAL: Multiple operations in search item match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("SRCH_VAL: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
}

impl ResolveError for SearchError {
    fn not_found(input: String) -> Self {
        SearchError::OperationNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        SearchError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        SearchError::InvalidShortId(input, min)
    }
}
