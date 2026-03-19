// src/logic/codexi/error.rs

use thiserror::Error;

use crate::{
    core::CoreError,
    logic::{account::AccountError, bank::BankError, currency::CurrencyError, utils::ResolveError},
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
    #[error("DATA_ACCOUNT: No current account selected — use `account use <id>` to select one")]
    NoCurrentAccount,
    #[error("DATA_ACCOUNT: Multiple account match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("DATA_ACCOUNT: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
}

impl ResolveError for CodexiError {
    fn not_found(input: String) -> Self {
        CodexiError::AccountNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        CodexiError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        CodexiError::InvalidShortId(input,min)
    }
}
