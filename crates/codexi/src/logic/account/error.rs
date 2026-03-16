///src/logic/account/error.rs
use thiserror::Error;

use crate::core::CoreError;
use crate::file_management::FileArchiveError;
use crate::logic::operation::OperationError;

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
    FinancialPolicy(#[from] FinancialPolicyError),
    #[error("OP_VAL: {0}")]
    AmbiguousShortId(String),
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
}

/// Error type for AccountType
#[derive(Debug, Error)]
pub enum AccountTypeError {
    #[error("OP_ACCOUNT_TYPE: Unknown account type: {0}")]
    Unknown(String),
}

/// Error type for financial policy
#[derive(Debug, Error)]
pub enum FinancialPolicyError {
    #[error(
        "FIN_OP: Source: {0} Account have no operation, performed a command, init <DATE> <AMOUNT>"
    )]
    HaveNoOperation(String),
    #[error("FIN_OP: Account have operation, Init not allowed")]
    HaveOperation,
    #[error("FIN_OP: Only Initial Operation, no close allowed")]
    OnlyInit,
    #[error("FIN_ACCOUNT: Account is close no operation is allowed")]
    AccountClose,
    #[error("FIN_DATA: {0}")]
    InvalidData(String),
    #[error("FIN_OP: Operation #{0} already voided")]
    OperationAlreadyVoided(String),
    #[error("FIN_OP: Operation #{0} not found")]
    OperationNotFound(String),
}
