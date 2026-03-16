// src/logic/category/error.rs

use thiserror::Error;

/// Error type for Category
#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("SYS_CATEGORY_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("DATA_CATEGORY: Category id not found: {0}")]
    CategoryNotFound(String),
    #[error("VAL_CATEGORY_NAME: Bank name: {0}")]
    InvalidName(String),
}
