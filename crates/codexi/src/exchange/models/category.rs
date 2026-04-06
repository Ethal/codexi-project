// src/exchange/models/category.rs

use serde::{Deserialize, Serialize};

use crate::{
    core::{format_id, format_optional_date},
    logic::category::Category,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCategory {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub terminated: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCategoryList {
    pub version: u16,
    #[serde(rename = "categories")]
    pub list: Vec<ExchangeCategory>,
}

impl From<&Category> for ExchangeCategory {
    fn from(c: &Category) -> Self {
        Self {
            id: Some(format_id(c.id)),
            name: c.name.clone(),
            parent_id: c.parent_id.map(format_id),
            terminated: format_optional_date(c.terminated),
            note: c.note.clone(),
        }
    }
}
