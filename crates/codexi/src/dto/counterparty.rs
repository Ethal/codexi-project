// src/dto/bank.rs

use crate::{
    core::{format_id, format_optional_date},
    logic::counterparty::{Counterparty, CounterpartyList},
};

#[derive(Debug)]
pub struct CounterpartyItem {
    pub id: String, // Nulid
    pub name: String,
    pub kind: String,
    pub terminated: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug)]
pub struct CounterpartyCollection {
    pub items: Vec<CounterpartyItem>,
}

impl From<&Counterparty> for CounterpartyItem {
    fn from(cp: &Counterparty) -> Self {
        Self {
            id: format_id(cp.id),
            name: cp.name.clone(),
            kind: cp.kind.as_str().to_string(),
            terminated: format_optional_date(cp.terminated),
            note: cp.note.clone(),
        }
    }
}

impl CounterpartyCollection {
    pub fn build(cps: &CounterpartyList) -> Self {
        let items: Vec<CounterpartyItem> = cps.list.iter().map(CounterpartyItem::from).collect();
        Self { items }
    }
}
