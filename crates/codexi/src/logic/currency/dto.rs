// src/logic/currency/dto.rts

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyItem {
    pub id: String,           // Nulid
    pub code: String,         // ex: "EUR", "USD"
    pub symbol: String,       // ex: "€"
    pub decimal_places: u32,  // Usually  2
    pub note: Option<String>, // free field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyEntry {
    pub items: Vec<CurrencyItem>,
}
