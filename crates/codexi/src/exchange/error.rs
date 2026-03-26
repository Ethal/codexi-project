// src/exchange/error.rs

use thiserror::Error;

use crate::{
    core::CoreError,
    logic::{account::AccountError, operation::OperationError},
};

#[derive(Debug, Error)]
pub enum ExchangeError {
    #[error("SYS_IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("DATA_JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("SYS_CORE: {0}")]
    Core(#[from] CoreError),
    #[error("SYS_ACCOUNT: {0}")]
    Account(#[from] AccountError),
    #[error("SYS_OP: {0}")]
    Operation(#[from] OperationError),
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
    #[error("EX_DUPLICATE_CU: {0}")]
    DuplicateCurrency(String),
    #[error("EX_VAL: {0}")]
    InvalidData(String),
}
