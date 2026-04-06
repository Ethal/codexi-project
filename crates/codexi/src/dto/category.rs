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
    pub parent_name: Option<String>,
    pub parent_terminated: Option<String>,
    pub terminated: Option<String>,
}

impl CategoryItem {
    pub fn build(category: &Category, categories: &CategoryList) -> Self {
        let parent = category
            .parent_id
            .as_ref()
            .and_then(|pid| categories.get_by_id(pid).ok());
        Self {
            id: format_id(category.id),
            name: category.name.clone(),
            note: category.note.clone(),
            parent_id: format_optional_id(category.parent_id),
            parent_name: parent.map(|p| p.name.clone()),
            parent_terminated: parent.and_then(|p| format_optional_date(p.terminated)),
            terminated: format_optional_date(category.terminated),
        }
    }
}

#[derive(Debug)]
pub struct CategoryCollection {
    pub items: Vec<CategoryItem>,
}

impl CategoryCollection {
    pub fn build(categories: &CategoryList) -> Self {
        let items = categories
            .list
            .iter()
            .map(|c| CategoryItem::build(c, categories))
            .collect();
        Self { items }
    }
}
