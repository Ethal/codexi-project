// src/exchange/error.rs

use thiserror::Error;

/// Error type for exchange data
#[derive(Debug, Error)]
pub enum ExchangeError {
    #[error("VAL_INVALID_VERSION: {0}")]
    InvalidVersion(String),
    #[error("VAL_DUPLICATE_OPERATION: {0}")]
    DuplicateOperation(String),
    #[error("VAL_MORE_THAN_ONCE_VOIDED: {0}")]
    MoreThanOnceVoided(String),
    #[error("VAL_VOID_TO_VOID: {0}")]
    VoidToVoid(String),
    #[error("VAL_INVALID_NEXT_OPID: {0}")]
    InvalidNextOpId(String),
    #[error("VAL_UNKNOW_VOID_OF: operation void with unknow void of")]
    UnknowVoidOf(String),
}
