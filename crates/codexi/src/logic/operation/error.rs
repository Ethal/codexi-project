// src/logic/operation/error.rs

use thiserror::Error;

use crate::{core::CoreError, logic::utils::ResolveError};

/// Error type for Operation
#[derive(Debug, Error)]
pub enum OperationError {
    #[error("SYS_OP_ID: {0}")]
    InvalidId(#[from] nulid::Error),
    #[error("SYS_OP_DATE: {0}")]
    InvalidDate(#[from] chrono::ParseError),
    #[error("SYS_CORE: {0}")]
    Core(#[from] CoreError),
    #[error("SYS_KIND: {0}")]
    Kind(#[from] OperationKindError),
    #[error("SYS_FLOW: {0}")]
    Flow(#[from] OperationFlowError),
    #[error("OP_INVALID_DESC: Operation description: {0}")]
    InvalidDescription(String),
    #[error("OP_BUILD: Operation Build: {0}")]
    OperationBuilder(String),
    #[error("OP_AMOUNT: Operation amount below zero or negative: {0}")]
    InvalidAmount(String),
    #[error("VAL_OP: No Operation with id: {0}")]
    OperationNotFound(String),
    #[error("OP_ACCOUNT: Multiple operatin match '{0}', use more characters")]
    AmbiguousShortId(String),
    #[error("OP_ACCOUNT: Invalid short id {0}, expected {1} characters minimum")]
    InvalidShortId(String, usize),
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

impl ResolveError for OperationError {
    fn not_found(input: String) -> Self {
        OperationError::OperationNotFound(input)
    }
    fn ambiguous(input: String) -> Self {
        OperationError::AmbiguousShortId(input)
    }
    fn invalid(input: String, min: usize) -> Self {
        OperationError::InvalidShortId(input, min)
    }
}
