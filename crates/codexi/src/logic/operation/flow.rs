// src/logic/operation/flow.rs

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::logic::operation::error::OperationFlowError;

/// Enum representing the flow of an operation: Debit or Credit
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OperationFlow {
    Debit,
    Credit,
    None,
}
/// Methods for OperationFlow
impl OperationFlow {
    /// Get the string representation of the specific flow
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationFlow::Debit => "Debit",
            OperationFlow::Credit => "Credit",
            OperationFlow::None => "None",
        }
    }
    /// Try to create an OperationFlow from a string
    pub fn try_from_str(s: &str) -> Result<Self, OperationFlowError> {
        let lower = s.trim().to_ascii_lowercase();
        match lower.as_ref() {
            "debit" | "db" => Ok(OperationFlow::Debit),
            "credit" | "cr" => Ok(OperationFlow::Credit),
            "none" | "no" => Ok(OperationFlow::None),
            _ => Err(OperationFlowError::Unknown(s.to_string())),
        }
    }
    /// Check if the OperationFlow is Debit or Credit
    pub fn is_debit(&self) -> bool {
        matches!(self, OperationFlow::Debit)
    }
    /// Check if the OperationFlow is Credit
    pub fn is_credit(&self) -> bool {
        matches!(self, OperationFlow::Credit)
    }
    /// Check if the OperationFlow is None
    pub fn is_none(&self) -> bool {
        matches!(self, OperationFlow::None)
    }
    /// Get the opposite flow
    pub fn opposite(&self) -> Self {
        match self {
            OperationFlow::Debit => OperationFlow::Credit,
            OperationFlow::Credit => OperationFlow::Debit,
            OperationFlow::None => OperationFlow::None,
        }
    }
    /// Toggle the flow in place
    pub fn toggle(&mut self) {
        *self = self.opposite();
    }
    /// Get the sign associated with the flow
    pub fn to_sign(&self) -> Decimal {
        match self {
            OperationFlow::Debit => Decimal::NEGATIVE_ONE,
            OperationFlow::Credit => Decimal::ONE,
            OperationFlow::None => Decimal::ZERO,
        }
    }
    /// Create an OperationFlow from a sign
    pub fn from_sign(sign: Decimal) -> Self {
        if sign > Decimal::ZERO {
            OperationFlow::Credit
        } else if sign < Decimal::ZERO {
            OperationFlow::Debit
        } else {
            OperationFlow::None
        }
    }
}
/// Implement TryFrom<&str> for OperationFlow
impl TryFrom<&str> for OperationFlow {
    type Error = OperationFlowError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        OperationFlow::try_from_str(value)
    }
}
/// Implement From<OperationFlow> for &'static str
impl From<OperationFlow> for &'static str {
    fn from(t: OperationFlow) -> Self {
        t.as_str()
    }
}
/// Implement Display for OperationFlow
impl fmt::Display for OperationFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<7}", self.as_str())
    }
}
