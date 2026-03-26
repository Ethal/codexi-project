// src/logic/currency/currency.rs

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::validate_text_rules;
use crate::logic::currency::CurrencyError;
use crate::logic::utils::{HasName, HasNulid};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub id: Nulid,           // Nulid
    pub code: String,        // ex: "EUR", "USD"
    pub symbol: String,      // ex: "€", "$"
    pub decimal_places: u32, // Usually  2
    pub note: Option<String>,
}

impl Currency {
    pub fn new(code: &str, symbol: &str, note: Option<&str>) -> Result<Self, CurrencyError> {
        let id = Nulid::new()?;
        let min = 3;
        let max = 3; // as per ISO 4217
        if let Err(e) = validate_text_rules(code, min, max) {
            return Err(CurrencyError::InvalidCode(e));
        }

        Ok(Self {
            id,
            code: code.to_string(),
            symbol: symbol.to_string(),
            decimal_places: 2,
            note: note.map(str::to_owned),
        })
    }
}

impl HasNulid for Currency {
    fn id(&self) -> Nulid {
        self.id
    }
}

impl HasName for Currency {
    fn name(&self) -> &str {
        &self.code
    }
}
