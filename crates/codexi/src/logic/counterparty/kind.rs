// src/logic/counterparty/kind.rs

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::logic::counterparty::CounterpartyKindError;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum CounterpartyKind {
    Person,
    #[default]
    Organization,
}

/// Methods for CounterpartyKind
impl CounterpartyKind {
    /// Get the string representation of the specific counter party kind
    pub fn as_str(&self) -> &'static str {
        match self {
            CounterpartyKind::Person => "Person",
            CounterpartyKind::Organization => "Organization",
        }
    }
    /// Try to create a counter party kind from a string
    pub fn try_from_str(s: &str) -> Result<Self, CounterpartyKindError> {
        let lower = s.to_ascii_lowercase();
        match lower.as_ref() {
            "person" | "pers" => Ok(CounterpartyKind::Person),
            "organization" | "org" => Ok(CounterpartyKind::Organization),
            _ => Err(CounterpartyKindError::Unknown(s.to_string())),
        }
    }
}
/// Implement TryFrom<&str> for counterparty kind
impl TryFrom<&str> for CounterpartyKind {
    type Error = CounterpartyKindError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        CounterpartyKind::try_from_str(s)
    }
}

/// Implement From<CounterpartyKind> for &'static str
impl From<CounterpartyKind> for &'static str {
    fn from(t: CounterpartyKind) -> Self {
        t.as_str()
    }
}
/// Implement Display for Counterparty kind
impl fmt::Display for CounterpartyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
