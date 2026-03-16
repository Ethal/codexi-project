// src/logic/account/account_type.rs

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::logic::account::error::AccountTypeError;

/// Enum representing the type of an account
#[derive(Debug, Clone, Default, Copy, PartialEq, Serialize, Deserialize)]
pub enum AccountType {
    #[default]
    Current,
    Saving,
    Joint,
    Deposit,
    Business,
    Student,
}

/// Methods for AccountType
impl AccountType {
    /// Get the string representation of the specific account
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Current => "Current",
            AccountType::Saving => "Saving",
            AccountType::Joint => "Joint",
            AccountType::Deposit => "Deposit",
            AccountType::Business => "Business",
            AccountType::Student => "Student",
        }
    }
    /// Try to create an AccountType from a string
    pub fn try_from_str(s: &str) -> Result<Self, AccountTypeError> {
        match s.trim().to_ascii_lowercase().as_str() {
            "current" | "cur" => Ok(AccountType::Current),
            "saving" | "sav" => Ok(AccountType::Saving),
            "joint" | "joi" => Ok(AccountType::Joint),
            "deposit" | "dep" => Ok(AccountType::Deposit),
            "business" | "bus" => Ok(AccountType::Business),
            "student" | "stu" => Ok(AccountType::Student),
            _ => Err(AccountTypeError::Unknown(s.to_string())),
        }
    }
    pub fn is_interest_bearing(&self) -> bool {
        matches!(self, AccountType::Saving | AccountType::Deposit)
    }

    pub fn is_shared(&self) -> bool {
        matches!(self, AccountType::Joint)
    }
}

/// Implement TryFrom<&str> for AccountType
impl TryFrom<&str> for AccountType {
    type Error = AccountTypeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AccountType::try_from_str(value)
    }
}

/// Implement From<AccountType> for &'static str
impl From<AccountType> for &'static str {
    fn from(t: AccountType) -> Self {
        t.as_str()
    }
}
/// Implement Display for AccountType
impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<7}", self.as_str())
    }
}
