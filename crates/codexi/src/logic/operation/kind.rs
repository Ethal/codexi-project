// src/logic/operation/kind.rs

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::logic::operation::RegularKind;
use crate::logic::operation::SystemKind;
use crate::logic::operation::error::OperationKindError;

/// Enum representing the kind of operation: System or Regular
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum OperationKind {
    System(SystemKind),
    Regular(RegularKind),
}
/// Methods for OperationKind
impl OperationKind {
    /// Check if the OperationKind is a System kind
    pub fn is_system(&self) -> bool {
        matches!(self, OperationKind::System(_))
    }
    /// Check if the OperationKind is a Regular kind
    pub fn is_regular(&self) -> bool {
        matches!(self, OperationKind::Regular(_))
    }
    /// Check if the operation is purely structural operation (Init/Close)
    pub fn is_structural(&self) -> bool {
        matches!(
            self,
            OperationKind::System(SystemKind::Init | SystemKind::Checkpoint)
        )
    }
    /// Check if the Operation is a Void type
    pub fn is_void(&self) -> bool {
        matches!(self, OperationKind::System(SystemKind::Void))
    }
    /// Check if the Operation is an adjust type
    pub fn is_adjust(&self) -> bool {
        matches!(self, OperationKind::System(SystemKind::Adjust))
    }
    /// Get the type of OperationKind as a string
    pub fn kind_type(&self) -> &'static str {
        match self {
            OperationKind::System(_) => "System",
            OperationKind::Regular(_) => "Regular",
        }
    }
    /// Get the string representation of the specific kind
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationKind::System(kind) => kind.as_str(),
            OperationKind::Regular(kind) => kind.as_str(),
        }
    }
    /// Try to create an OperationKind from a string
    pub fn try_from_str(s: &str) -> Result<Self, OperationKindError> {
        let lower = s.to_ascii_lowercase();

        if let Ok(sk) = SystemKind::try_from_str(&lower) {
            return Ok(OperationKind::System(sk));
        }

        if let Ok(rk) = RegularKind::try_from_str(&lower) {
            return Ok(OperationKind::Regular(rk));
        }

        Err(OperationKindError::Unknown(lower.to_string()))
    }
}
/// Implement TryFrom<&str> for OperationKind
impl TryFrom<&str> for OperationKind {
    type Error = OperationKindError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        OperationKind::try_from_str(s)
    }
}
/// Implement From<OperationKind> for &'static str
impl From<OperationKind> for &'static str {
    fn from(t: OperationKind) -> Self {
        t.as_str()
    }
}
/// Implement Display for OperationKind
impl fmt::Display for OperationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationKind::System(system_kind) => write!(f, "System::{system_kind}"),
            OperationKind::Regular(regular_kind) => write!(f, "Regular::{regular_kind}"),
        }
    }
}
