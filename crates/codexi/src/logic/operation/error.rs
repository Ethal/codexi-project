// src/logic/operation/error.rs

use thiserror::Error;

/// Error type for Operation
#[derive(Debug, Error)]
pub enum OperationError {
    #[error("SYS_OP_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("OP_INVALID_DATE: Invalid Operation Date format: {0}")]
    InvalidDate(#[from] chrono::ParseError),
    #[error("OP_INVALID_DESC: Operation description: {0}")]
    InvalidDescription(String),
    #[error("OP_BUILD: Operation Build: {0}")]
    OperationBuilder(String),
    #[error("OP_AMOUNT: Operation amount below zero or negative: {0}")]
    InvalidAmount(String),
}

/// Error type for OperationFlow
#[derive(Debug, Error)]
pub enum OperationFlowError {
    #[error("OP_INVALID_FLOW: Unknown OperationFlow type: {0}")]
    Unknown(String),
}

/// Error type for OperationKind
#[derive(Debug, Error)]
pub enum OperationKindError {
    #[error("OP_INVALID_OPERATION_KIND: Unknown Operation Kind: {0}")]
    Unknown(String),
}

/// Error type for RegularKind
#[derive(Debug, Error)]
pub enum RegularKindError {
    #[error("OP_INVALID_REGULAR_KIND: Unknown Regular Kind: {0}")]
    Unknown(String),
}

/// Error type for SystemKind
#[derive(Debug, Error)]
pub enum SystemKindError {
    #[error("OP_INVALID_SYSTEM_KIND: Unknown System Kind: {0}")]
    Unknown(String),
}
