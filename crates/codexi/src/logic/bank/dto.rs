// src/logic/bank/dto.rts

use crate::{core::format_id, logic::bank::Bank};

#[derive(Debug)]
pub struct BankItem {
    pub id: String,             // Nulid
    pub name: String,           // ex: "Boursorama"
    pub branch: Option<String>, // ex: "Boursorama France"
    pub note: Option<String>,   // Free field (address , code swift, ...)
}

#[derive(Debug)]
pub struct BankEntry {
    pub items: Vec<BankItem>,
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
