// src/logic/codexi/init_data.rs

use nulid::Nulid;
use serde::Deserialize;

use crate::logic::bank::Bank;
use crate::logic::category::Category;
use crate::logic::counterparty::{Counterparty, CounterpartyKind};
use crate::logic::currency::Currency;

// ----------------------------------------------------------------
// Seed data — embedded in binary at compile time
// ----------------------------------------------------------------

const CURRENCIES_JSON: &str = include_str!("../../seeds/currencies.json");
const CATEGORIES_FR_JSON: &str = include_str!("../../seeds/categories_fr.json");
const CATEGORIES_EN_JSON: &str = include_str!("../../seeds/categories_en.json");
const COUNTERPARTIES_FR_JSON: &str = include_str!("../../seeds/counterparties_fr.json");
const COUNTERPARTIES_EN_JSON: &str = include_str!("../../seeds/counterparties_en.json");
const BANKS_JSON: &str = include_str!("../../seeds/banks.json");

// ----------------------------------------------------------------
// Intermediate seed structs (no id — generated at runtime)
// ----------------------------------------------------------------

#[derive(Deserialize)]
struct BankSeed {
    pub name: String,
    pub branch: Option<String>,
    pub note: Option<String>,
}

#[derive(Deserialize)]
struct CurrencySeed {
    pub code: String,
    pub symbol: String,
    pub decimal_places: u32,
    pub note: Option<String>,
}

#[derive(Deserialize)]
struct CategorySeed {
    pub name: String,
    pub note: Option<String>,
}

#[derive(Deserialize)]
struct CounterpartySeed {
    pub name: String,
    pub kind: CounterpartyKind,
    pub note: Option<String>,
}

// ----------------------------------------------------------------
// Public API
// ----------------------------------------------------------------

/// Returns the default bank list (language-independent).
pub fn default_banks() -> Vec<Bank> {
    let seeds: Vec<BankSeed> = serde_json::from_str(BANKS_JSON).expect("Invalid banks seed JSON");

    seeds
        .into_iter()
        .map(|s| Bank {
            id: Nulid::new().expect("Nulid generation failed"),
            name: s.name,
            branch: s.branch,
            note: s.note,
        })
        .collect()
}

/// Returns the default currency list (language-independent).
pub fn default_currencies() -> Vec<Currency> {
    let seeds: Vec<CurrencySeed> = serde_json::from_str(CURRENCIES_JSON).expect("Invalid currencies seed JSON");

    seeds
        .into_iter()
        .map(|s| Currency {
            id: Nulid::new().expect("Nulid generation failed"),
            code: s.code,
            symbol: s.symbol,
            decimal_places: s.decimal_places,
            note: s.note,
        })
        .collect()
}

/// Returns default categories for the given language code.
/// Falls back to English if language is not supported.
pub fn default_categories(language: &str) -> Vec<Category> {
    let json = match language {
        "fr" => CATEGORIES_FR_JSON,
        _ => CATEGORIES_EN_JSON,
    };

    let seeds: Vec<CategorySeed> = serde_json::from_str(json).expect("Invalid categories seed JSON");

    seeds
        .into_iter()
        .map(|s| Category {
            id: Nulid::new().expect("Nulid generation failed"),
            name: s.name,
            note: s.note,
            parent_id: None,
            terminated: None,
        })
        .collect()
}

/// Returns default counterparties for the given language code.
/// Falls back to English if language is not supported.
pub fn default_counterparties(language: &str) -> Vec<Counterparty> {
    let json = match language {
        "fr" => COUNTERPARTIES_FR_JSON,
        _ => COUNTERPARTIES_EN_JSON,
    };

    let seeds: Vec<CounterpartySeed> = serde_json::from_str(json).expect("Invalid counterparies seed JSON");

    seeds
        .into_iter()
        .map(|s| Counterparty {
            id: Nulid::new().expect("Nulid generation failed"),
            name: s.name,
            kind: s.kind,
            note: s.note,
            terminated: None,
        })
        .collect()
}
