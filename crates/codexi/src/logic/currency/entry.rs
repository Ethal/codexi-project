// src/logic/currency/entry.rs

use nulid::Nulid;

use crate::logic::currency::{CurrencyEntry, CurrencyItem, CurrencyList};

impl CurrencyList {
    pub fn currency_entry(&self) -> CurrencyEntry {
        let items: Vec<CurrencyItem> = self.currencies.iter().map(CurrencyItem::from).collect();
        CurrencyEntry { items }
    }

    pub fn currency_item(&self, id: &Nulid) -> Option<CurrencyItem> {
        self.get_by_id(id).map(CurrencyItem::from).ok()
    }
}
