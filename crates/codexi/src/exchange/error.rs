// src/exchange/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExchangeError {
    #[error("SYS_IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("DATA_JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("EX_INVALID_VERSION: {0}")]
    InvalidVersion(String),
    #[error("EX_DUPLICATE_OP: {0}")]
    DuplicateOperation(String),
    #[error("EX_VOID_UNKNOWN: {0}")]
    UnknowVoidOf(String),
    #[error("EX_VOID_TWICE: {0}")]
    MoreThanOnceVoided(String),
    #[error("EX_VOID_TO_VOID: {0}")]
    VoidToVoid(String),
    #[error("EX_INVALID_AMOUNT: {0}")]
    InvalidAmount(String),
    #[error("EX_BROKEN_TRANSFER_LINK: {0}")]
    BrokenTransferLink(String),
}
