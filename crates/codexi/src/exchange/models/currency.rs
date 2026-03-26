// src/exchange/models/currency@.rs

use serde::{Deserialize, Serialize};

use crate::{core::format_id, logic::currency::Currency};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCurrency {
    #[serde(default)]
    pub id: Option<String>,
    pub code: String,        // ex: "EUR", "USD"
    pub symbol: String,      // ex: "€", "$"
    pub decimal_places: u32, // Usually  2
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCurrencyList {
    pub version: u16,
    pub currencies: Vec<ExchangeCurrency>,
}

impl From<&Currency> for ExchangeCurrency {
    fn from(c: &Currency) -> Self {
        Self {
            id: Some(format_id(c.id)),
            code: c.code.clone(),
            symbol: c.symbol.clone(),
            decimal_places: c.decimal_places,
            note: c.note.clone(),
        }
    }
}
