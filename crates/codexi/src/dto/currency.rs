// src/dto/currency.rs

use crate::{
    core::format_id,
    logic::currency::{Currency, CurrencyList},
};

#[derive(Debug)]
pub struct CurrencyItem {
    pub id: String,           // Nulid
    pub code: String,         // ex: "EUR", "USD"
    pub symbol: String,       // ex: "€"
    pub decimal_places: u32,  // Usually  2
    pub note: Option<String>, // free field
}

#[derive(Debug)]
pub struct CurrencyCollection {
    pub items: Vec<CurrencyItem>,
}

impl From<&Currency> for CurrencyItem {
    fn from(currency: &Currency) -> Self {
        Self {
            id: format_id(currency.id),
            code: currency.code.clone(),
            symbol: currency.symbol.clone(),
            decimal_places: currency.decimal_places,
            note: currency.note.clone(),
        }
    }
}

impl CurrencyCollection {
    pub fn build(currencies: &CurrencyList) -> Self {
        let items: Vec<CurrencyItem> = currencies.currencies.iter().map(CurrencyItem::from).collect();
        Self { items }
    }
}
