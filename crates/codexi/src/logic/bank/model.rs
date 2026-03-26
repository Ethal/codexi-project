// src/logic/bank/bank.rs

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::validate_text_rules;
use crate::logic::bank::BankError;
use crate::logic::utils::{HasName, HasNulid};

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

impl HasNulid for Bank {
    fn id(&self) -> Nulid {
        self.id
    }
}

impl HasName for Bank {
    fn name(&self) -> &str {
        &self.name
    }
}
