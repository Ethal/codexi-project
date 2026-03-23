///src/logic/account/error.rs
use thiserror::Error;

use crate::core::CoreError;
use crate::file_management::FileArchiveError;
use crate::logic::account::policy::{ComplianceViolation, LifecycleViolation, TemporalViolation};
use crate::logic::operation::OperationError;
use crate::logic::utils::ResolveError;

/// Struct representing the Account Error
#[derive(Debug, Error)]
pub enum AccountError {
    #[error("VAL_DATE: Invalid Date format: {0}")]
    InvalidDate(#[from] chrono::ParseError),
    #[error("SYS_ID: Invalid id: {0}")]
    Id(#[from] nulid::Error),
    #[error("OP_INVALID: {0}")]
    Operation(#[from] OperationError),
    #[error("SYS_ACCOUNT: {0}")]
    AccountType(#[from] AccountTypeError),
    #[error("VAL_DATA: {0}")]
    InvalidData(String),
    #[error("OP_CLOSING: No operation to close and archived")]
    NothingClose,
    #[error("OP_ADJUST: No Adjust performed, balance and physical amout are equal")]
    NoAdjust,
    #[error("OP_ANY: Operation #{0} not found")]
    OperationNotFound(String),
    #[error("OP_VOID: Operation #{0} already voided")]
    OperationAlreadyVoided(String),
    #[error("SYS_ARCHIVE: {0}")]
    FileArchive(#[from] FileArchiveError),
    #[error("SRCH_VALIDATION: {0}")]
    Search(#[from] SearchError),
    #[error("FIN_VALIDATION: {0}")]
    TemporalViolation(#[from] TemporalViolation),
    #[error("FIN_VALIDATION: {0}")]
    ComplianceViolation(#[from] ComplianceViolation),
    #[error("FIN_VALIDATION: {0}")]
    LifecycleViolation(#[from] LifecycleViolation),
    #[error("OP_VAL: Multiple operations match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("OP_VAL: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
}

/// Error type for search in account
#[derive(Debug, Error)]
pub enum SearchError {
    #[error("SRCH_INVALID_DATE: {0}")]
    InvalidDate(String),
    #[error("SYS_COMMON: {0}")]
    Common(#[from] CoreError),
    #[error("SRCH_BUILD: search parameters build: {0}")]
    SearchParametersBuilder(String),
    #[error("SRCH_VAL: Operation #{0} not found in search item")]
    OperationNotFound(String),
    #[error("SRCH_VAL: Multiple operations in search item match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("SRCH_VAL: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
}

/// Error type for AccountType
#[derive(Debug, Error)]
pub enum AccountTypeError {
    #[error("OP_ACCOUNT_TYPE: Unknown account type: {0}")]
    Unknown(String),
}

impl ResolveError for AccountError {
    fn not_found(input: String) -> Self {
        AccountError::OperationNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        AccountError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        AccountError::InvalidShortId(input, min)
    }
}

impl ResolveError for SearchError {
    fn not_found(input: String) -> Self {
        SearchError::OperationNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        SearchError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        SearchError::InvalidShortId(input, min)
    }
}
