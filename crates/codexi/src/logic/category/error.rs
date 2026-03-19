// src/logic/category/error.rs

use thiserror::Error;

use crate::logic::utils::ResolveError;

/// Error type for Category
#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("SYS_CATEGORY_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("DATA_CATEGORY: Category id not found: {0}")]
    CategoryNotFound(String),
    #[error("VAL_CATEGORY_NAME: Bank name: {0}")]
    InvalidName(String),
    #[error("DATA_BANK: Multiple operations match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("DATA_BANK: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
}

impl ResolveError for CategoryError {
    fn not_found(input: String) -> Self {
        CategoryError::CategoryNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        CategoryError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        CategoryError::InvalidShortId(input, min)
    }
}
