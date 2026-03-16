// src/logic/category/dto.rts

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryItem {
    pub id: String,   // Nulid
    pub name: String, // ex: "Food"
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryEntry {
    pub items: Vec<CategoryItem>,
}
