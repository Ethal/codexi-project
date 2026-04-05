// src/dto/category.rs

use crate::{
    core::{format_id, format_optional_date, format_optional_id},
    logic::category::{Category, CategoryList},
};

#[derive(Debug)]
pub struct CategoryItem {
    pub id: String,   // Nulid
    pub name: String, // ex: "Food"
    pub note: Option<String>,
    pub parent_id: Option<String>,
    pub terminated: Option<String>,
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
            parent_id: format_optional_id(category.parent_id),
            terminated: format_optional_date(category.terminated),
        }
    }
}

impl CategoryCollection {
    pub fn build(categories: &CategoryList) -> Self {
        let items: Vec<CategoryItem> = categories.list.iter().map(CategoryItem::from).collect();
        Self { items }
    }
}
