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
    #[error("DATA_ACCOUNT: Only one account")]
    OnlyOneAccount,
    #[error("DATA_ACCOUNT: Can not close the current account")]
    CloseCurentAccount,
    #[error("DATA_ACCOUNT: Closing date cannot before opening date {0}")]
    CloseDateAccount(String),
}
