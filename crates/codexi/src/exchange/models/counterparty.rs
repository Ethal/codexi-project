// src/exchange/models/counterparty.rs

use serde::{Deserialize, Serialize};

use crate::{
    core::{format_id, format_optional_date},
    logic::counterparty::Counterparty,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCounterparty {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub terminated: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCounterpartyList {
    pub version: u16,
    #[serde(rename = "counterparties")]
    pub list: Vec<ExchangeCounterparty>,
}

impl From<&Counterparty> for ExchangeCounterparty {
    fn from(c: &Counterparty) -> Self {
        Self {
            id: Some(format_id(c.id)),
            name: c.name.clone(),
            kind: c.kind.as_str().to_string(),
            terminated: format_optional_date(c.terminated),
            note: c.note.clone(),
        }
    }
}
