// src/core/warning.rs

use std::fmt;

#[derive(Debug)]
pub struct CoreWarning {
    pub kind: CoreWarningKind,
    pub message: String,
}

#[derive(Debug)]
pub enum CoreWarningKind {
    VoidOfNotFound,
    InvalidData,
    ContextNotApplicable,
    TransferAccountNotFound, // ← nouveau
}

impl CoreWarningKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoreWarningKind::VoidOfNotFound => "VoidOfNotFound",
            CoreWarningKind::InvalidData => "InvalidData",
            CoreWarningKind::ContextNotApplicable => "ContextNotApplicable",
            CoreWarningKind::TransferAccountNotFound => "TransferAccountNotFound",
        }
    }
}

impl fmt::Display for CoreWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.message)
    }
}
