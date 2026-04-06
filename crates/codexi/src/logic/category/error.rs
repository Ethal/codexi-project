// src/logic/category/error.rs

use thiserror::Error;

use crate::{core::CoreError, logic::utils::ResolveError};

/// Error type for Category
#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("SYS_CATEGORY_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("SYS_CORE: {0}")]
    Core(#[from] CoreError),
    #[error("DATA_CATEGORY: Category id not found: {0}")]
    CategoryNotFound(String),
    #[error("VAL_CATEGORY_NAME: Bank name: {0}")]
    InvalidName(String),
    #[error("DATA_CATEGORY: Multiple operations match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("DATA_CATEGORY: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
    #[error("VAL_CATEGORY: Duplicate category name {0}")]
    DuplicateName(String),
    #[error("VAL_CATEGORY: Has children {0}")]
    HasActiveChildren(String),
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
