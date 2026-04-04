// src/dto/bank.rs

use crate::{
    core::format_id,
    logic::bank::{Bank, BankList},
};

#[derive(Debug)]
pub struct BankItem {
    pub id: String, // Nulid
    pub name: String,
    pub branch: Option<String>,
    pub note: Option<String>, // Free field (address , code swift, ...)
}

#[derive(Debug)]
pub struct BankCollection {
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

impl BankCollection {
    pub fn build(banks: &BankList) -> Self {
        let items: Vec<BankItem> = banks.banks.iter().map(BankItem::from).collect();
        Self { items }
    }
}
