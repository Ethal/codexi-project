// src/logic/currency/list.rs

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::format_id;
use crate::logic::currency::{Currency, CurrencyError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyList {
    pub currencies: Vec<Currency>,
}

impl CurrencyList {
    pub fn new() -> Self {
        Self {
            currencies: Vec::new(),
        }
    }

    pub fn add(&mut self, currency: Currency) -> Nulid {
        let id = currency.id;
        self.currencies.push(currency);
        id
    }

    pub fn create(
        &mut self,
        code: &str,
        symbol: &str,
        note: Option<&str>,
    ) -> Result<Nulid, CurrencyError> {
        let currency = Currency::new(code, symbol, note)?;
        let id = self.add(currency);
        Ok(id)
    }

    pub fn update(
        &mut self,
        id: Nulid,
        symbol: &str,
        note: Option<&str>,
    ) -> Result<(), CurrencyError> {
        let currency = self.get_by_id_mut(&id)?;
        currency.symbol = symbol.into();
        currency.note = note.map(str::to_owned);
        Ok(())
    }

    pub fn get_by_id(&self, id: &Nulid) -> Result<&Currency, CurrencyError> {
        self.currencies
            .iter()
            .find(|c| &c.id == id)
            .ok_or_else(|| CurrencyError::CurrencyNotFound(format_id(*id)))
    }
    pub fn get_by_id_mut(&mut self, id: &Nulid) -> Result<&mut Currency, CurrencyError> {
        self.currencies
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or_else(|| CurrencyError::CurrencyNotFound(format_id(*id)))
    }
    pub fn currency_id_by_code(&self, code: &str) -> Option<Nulid> {
        self.currencies
            .iter()
            .find(|c| c.code == code)
            .map(|c| c.id)
    }

    pub fn currency_code_by_id(&self, id: &Nulid) -> Option<String> {
        self.currencies
            .iter()
            .find(|c| &c.id == id)
            .map(|c| c.code.clone())
    }

    pub fn count(&self) -> usize {
        self.currencies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.currencies.is_empty()
    }

    pub fn is_exist(&self, id: &Nulid) -> bool {
        self.currencies.iter().any(|c| &c.id == id)
    }
}

impl Default for CurrencyList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Currency>> for CurrencyList {
    fn from(currencies: Vec<Currency>) -> Self {
        Self { currencies }
    }
}
