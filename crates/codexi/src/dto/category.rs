// src/dto/category.rs

use crate::{
    core::format_id,
    logic::category::{Category, CategoryList},
};

#[derive(Debug)]
pub struct CategoryItem {
    pub id: String,   // Nulid
    pub name: String, // ex: "Food"
    pub note: Option<String>,
}

#[derive(Debug)]
pub struct CategoryCollection {
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

impl CategoryCollection {
    pub fn build(categories: &CategoryList) -> Self {
        let items: Vec<CategoryItem> = categories
            .categories
            .iter()
            .map(CategoryItem::from)
            .collect();
        Self { items }
    }
}
