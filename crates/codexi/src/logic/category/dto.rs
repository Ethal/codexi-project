// src/logic/category/dto.rts

use crate::{core::format_id, logic::category::Category};

#[derive(Debug)]
pub struct CategoryItem {
    pub id: String,   // Nulid
    pub name: String, // ex: "Food"
    pub note: Option<String>,
}

#[derive(Debug)]
pub struct CategoryEntry {
    pub items: Vec<CategoryItem>,
}

impl From<&Category> for CategoryItem {
    fn from(category: &Category) -> Self {
        Self {
            id: format_id(category.id),
            name: category.name.clone(),
            note: category.note.clone(),
        }
    }
}
