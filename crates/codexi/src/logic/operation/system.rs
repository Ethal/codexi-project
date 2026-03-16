// src/logic/operation/system.rs

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::logic::operation::error::SystemKindError;

/// Enum representing the system kinds of operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum SystemKind {
    Init,
    Adjust,
    Checkpoint,
    Void,
}
/// Methods for SystemKind
impl SystemKind {
    /// Get the string representation of the specific system kind
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemKind::Init => "Initialize",
            SystemKind::Adjust => "Adjust",
            SystemKind::Checkpoint => "Checkpoint",
            SystemKind::Void => "Void",
        }
    }
    /// Try to create a SystemKind from a string
    pub fn try_from_str(s: &str) -> Result<Self, SystemKindError> {
        let lower = s.to_ascii_lowercase();
        match lower.as_ref() {
            "init" => Ok(SystemKind::Init),
            "adjust" => Ok(SystemKind::Adjust),
            "checkpoint" => Ok(SystemKind::Checkpoint),
            "close" => Ok(SystemKind::Checkpoint),
            "void" => Ok(SystemKind::Void),
            _ => Err(SystemKindError::Unknown(s.to_string())),
        }
    }
}
/// Implement TryFrom<&str> for SystemKind
impl TryFrom<&str> for SystemKind {
    type Error = SystemKindError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        SystemKind::try_from_str(s)
    }
}
/// Implement From<SystemKind> for &'static str
impl From<SystemKind> for &'static str {
    fn from(t: SystemKind) -> Self {
        t.as_str()
    }
}
/// Implement Display for SystemKind
impl fmt::Display for SystemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<7}", <&'static str>::from(*self))
    }
}
