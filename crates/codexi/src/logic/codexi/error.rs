// src/logic/codexi/error.rs

use thiserror::Error;

use crate::{
    core::CoreError,
    logic::{account::AccountError, bank::BankError, currency::CurrencyError},
};

#[derive(Debug, Error)]
pub enum CodexiError {
    #[error("SYS_IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("SYS_ID: {0}")]
    Id(#[from] nulid::Error),
    #[error("DATA_JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("SYS_COMMON: {0}")]
    Common(#[from] CoreError),
    #[error("SYS_ACCOUNT: {0}")]
    Account(#[from] AccountError),
    #[error("SYS_BANK: {0}")]
    Bank(#[from] BankError),
    #[error("SYS_BANK: {0}")]
    Currency(#[from] CurrencyError),
    #[error("DATA_ACCOUNT: No account with id: {0}")]
    AccountNotFound(String),
    #[error("No current account selected — use `account use <id>` to select one")]
    NoCurrentAccount,
}
