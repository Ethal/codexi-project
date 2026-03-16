// src/logic/bank/bank.rs

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::{format_id, validate_text_rules};
use crate::logic::bank::{BankError, BankItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bank {
    pub id: Nulid,              // Nulid
    pub name: String,           // ex: "Boursorama"
    pub branch: Option<String>, // ex: "Boursorama France"
    pub note: Option<String>,   // Free field (address , code swift, ...)
}

impl Bank {
    pub fn new(name: &str, branch: Option<&str>, note: Option<&str>) -> Result<Self, BankError> {
        let id = Nulid::new()?;

        let min = 3;
        let max = 20;
        if let Err(e) = validate_text_rules(name, min, max) {
            return Err(BankError::InvalidName(e));
        }

        Ok(Self {
            id,
            name: name.to_string(),
            branch: branch.map(str::to_owned),
            note: note.map(str::to_owned),
        })
    }
}

impl From<&Bank> for BankItem {
    fn from(bank: &Bank) -> Self {
        Self {
            id: format_id(bank.id),
            name: bank.name.clone(),
            branch: bank.branch.clone(),
            note: bank.note.clone(),
        }
    }
}
