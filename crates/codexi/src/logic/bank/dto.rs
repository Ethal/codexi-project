// src/logic/bank/dto.rts

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankItem {
    pub id: String,             // Nulid
    pub name: String,           // ex: "Boursorama"
    pub branch: Option<String>, // ex: "Boursorama France"
    pub note: Option<String>,   // Free field (address , code swift, ...)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankEntry {
    pub items: Vec<BankItem>,
}
