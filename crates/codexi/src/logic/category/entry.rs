// src/logic/category/entry.rs

use nulid::Nulid;

use crate::logic::category::{CategoryEntry, CategoryItem, CategoryList};

impl CategoryList {
    pub fn category_entry(&self) -> CategoryEntry {
        let items: Vec<CategoryItem> = self.categories.iter().map(CategoryItem::from).collect();
        CategoryEntry { items }
    }

    pub fn category_item(&self, id: &Nulid) -> Option<CategoryItem> {
        self.get_by_id(id).map(CategoryItem::from).ok()
    }
}
