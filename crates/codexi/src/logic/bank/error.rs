// src/logic/bank/error.rs

use thiserror::Error;

/// Error type for Bank
#[derive(Debug, Error)]
pub enum BankError {
    #[error("SYS_BANK_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("DATA_BANK: Bank id not found: {0}")]
    BankNotFound(String),
    #[error("VAL_BANK_NAME: Bank name: {0}")]
    InvalidName(String),
}
