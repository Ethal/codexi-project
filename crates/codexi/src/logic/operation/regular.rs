// src/logic/operation/regular.rs

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::logic::operation::error::RegularKindError;

/// Enum representing the regular kinds of operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum RegularKind {
    Transaction,
    Fee,
    Transfer,
    Refund,
}
/// Methods for RegularKind
impl RegularKind {
    /// Get the string representation of the specific regular kind
    pub fn as_str(&self) -> &'static str {
        match self {
            RegularKind::Transaction => "Transaction",
            RegularKind::Fee => "Fee",
            RegularKind::Transfer => "Transfer",
            RegularKind::Refund => "Refund",
        }
    }
    /// Try to create a RegularKind from a string
    pub fn try_from_str(s: &str) -> Result<Self, RegularKindError> {
        let lower = s.to_ascii_lowercase();
        match lower.as_ref() {
            "transaction" | "trans" => Ok(RegularKind::Transaction),
            "fee" => Ok(RegularKind::Fee),
            "transfer" => Ok(RegularKind::Transfer),
            "refund" => Ok(RegularKind::Refund),
            _ => Err(RegularKindError::Unknown(s.to_string())),
        }
    }
}
/// Implement TryFrom<&str> for RegularKind
impl TryFrom<&str> for RegularKind {
    type Error = RegularKindError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        RegularKind::try_from_str(s)
    }
}
/// Implement From<RegularKind> for &'static str
impl From<RegularKind> for &'static str {
    fn from(t: RegularKind) -> Self {
        t.as_str()
    }
}
/// Implement Display for RegularKind
impl fmt::Display for RegularKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<11}", self.as_str())
    }
}
