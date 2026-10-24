// src/core/error.rs

use miette::Diagnostic;
use thiserror::Error;

/// Error type for Core
#[derive(Debug, Error, Diagnostic)]
pub enum CoreError {
    // --- ERRORS SYSTÈME (no specific diagnostic) ---
    #[error("SYS_IO: {0}")]
    #[diagnostic(code(codexi::core::io))]
    Io(#[from] std::io::Error),

    #[error("SYS_ID: {0}")]
    #[diagnostic(code(codexi::core::id))]
    Id(#[from] nulid::Error),

    #[error("SYS_NO_DATA_DIR: {0}")]
    #[diagnostic(code(codexi::core::sys::no_data_dir))]
    NoDataDirectory(String),

    #[error("SYS_NO_CONFIG_DIR: {0}")]
    #[diagnostic(code(codexi::core::sys::no_config_dir))]
    NoConfigDirectory(String),

    #[error("SYS_NO_DOCUMENTS_DIR: {0}")]
    #[diagnostic(code(codexi::core::sys::no_documents_dir))]
    NoDocumentsDirectory(String),

    // --- VALIDATION ERRORS (with codes and help message) ---
    #[error("VAL_DECIMAL: Decimal error in field, {field}: {source}")]
    #[diagnostic(
        code(codexi::core::val::decimal),
        help("Ensure the number uses a dot '.' as a decimal separator (e.g., 1250.50).")
    )]
    Decimal { source: rust_decimal::Error, field: String },

    #[error("VAL_INT: Number error in field, {field}: {source}")]
    #[diagnostic(
        code(codexi::core::val::integer),
        help("This field expects a valid whole number without decimals or text.")
    )]
    Number {
        source: std::num::ParseIntError,
        field: String,
    },

    #[error("VAL_DATE: {0}")]
    #[diagnostic(
        code(codexi::core::val::date::invalid),
        help("Dates must strictly follow the ISO-8601 YYYY-MM-DD format (e.g., 2026-10-24).")
    )]
    InvalidDate(String),

    #[error("VAL_DATE:{0}")]
    #[diagnostic(code(codexi::core::val::date::start_invalid))]
    InvalidStartDate(String),

    #[error("VAL_DATE: {0}")]
    #[diagnostic(code(codexi::core::val::date::end_invalid))]
    InvalidEndDate(String),

    #[error("VAL_DATE: {0}")]
    #[diagnostic(code(codexi::core::val::date::intermediate_invalid))]
    InvalidIntermediateDate(String),

    #[error("VAL_MONTH: {0}")]
    #[diagnostic(
        code(codexi::core::val::month::invalid),
        help("Months should be specified in YYYY-MM format or via a valid numeric range (1-12).")
    )]
    InvalidMonth(String),

    #[error("VAL_MONTH: {0}")]
    #[diagnostic(code(codexi::core::val::month::compute_error))]
    ErrorComputingEndOfMonth(String),

    #[error("VAL_DATA: {0}")]
    #[diagnostic(code(codexi::core::val::data_corrupted))]
    InvalidData(String),
}
