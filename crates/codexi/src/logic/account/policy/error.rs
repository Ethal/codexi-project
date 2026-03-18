// src/logic/account/policy/error.rs

use chrono::NaiveDate;
use rust_decimal::Decimal;
use thiserror::Error;

/// Error type for temporal policy
#[derive(Debug, Error)]
pub enum TemporalViolation {
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

#[derive(Debug, Error)]
pub enum ComplianceViolation {
    #[error("Overdraft exceeded: limit {limit}, resulting balance {resulting}")]
    OverdraftExceeded { limit: Decimal, resulting: Decimal },

    #[error("Minimum balance violated: minimum {minimum}, resulting balance {resulting}")]
    MinBalanceViolated {
        minimum: Decimal,
        resulting: Decimal,
    },

    #[error("Monthly transaction limit reached: {max} operations/month maximum")]
    MonthlyLimitExceeded { max: u32 },

    #[error("Operation not allowed for this account type: {reason}")]
    NotAllowed { reason: &'static str },

    #[error("Invalid context value: {reason}")]
    InvalidContextValue { reason: &'static str },
}

#[derive(Debug, Error)]
pub enum LifecycleViolation {
    #[error("Account type cannot be changed once operations exist")]
    AccountTypeImmutable,

    #[error("Close date cannot be in the future")]
    CloseDateInFuture,

    #[error("Close date {0} cannot be before the account open date {1}")]
    CloseDateBeforeOpenDate(NaiveDate, NaiveDate),

    #[error("Close date {0} cannot be before the last operation date {1}")]
    CloseDateBeforeLastOperation(NaiveDate, NaiveDate),
}
