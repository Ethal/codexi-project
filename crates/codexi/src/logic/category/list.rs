// src/logic/category/list

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::format_id;
use crate::logic::category::{Category, CategoryEntry, CategoryError, CategoryItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryList {
    pub categories: Vec<Category>,
}

impl CategoryList {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
        }
    }

    pub fn add(&mut self, category: Category) -> Nulid {
        let id = category.id;
        self.categories.push(category);
        id
    }

    pub fn create(&mut self, name: &str, note: Option<&str>) -> Result<Nulid, CategoryError> {
        let category = Category::new(name, note)?;
        let id = self.add(category);
        Ok(id)
    }

    pub fn update(
        &mut self,
        id: Nulid,
        name: &str,
        note: Option<&str>,
    ) -> Result<(), CategoryError> {
        let category = self.get_by_id_mut(&id)?;
        category.name = name.into();
        category.note = note.map(str::to_owned);
        Ok(())
    }

    pub fn get_by_id(&self, id: &Nulid) -> Result<&Category, CategoryError> {
        self.categories
            .iter()
            .find(|c| &c.id == id)
            .ok_or_else(|| CategoryError::CategoryNotFound(format_id(*id)))
    }

    pub fn get_by_id_mut(&mut self, id: &Nulid) -> Result<&mut Category, CategoryError> {
        self.categories
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or_else(|| CategoryError::CategoryNotFound(format_id(*id)))
    }

    pub fn count(&self) -> usize {
        self.categories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    pub fn is_exist(&self, id: &Nulid) -> bool {
        self.categories.iter().any(|c| &c.id == id)
    }

    pub fn category_entry(&self) -> CategoryEntry {
        let items: Vec<CategoryItem> = self.categories.iter().map(CategoryItem::from).collect();

        CategoryEntry { items }
    }
    pub fn category_item(&self, id: &Nulid) -> Result<CategoryItem, CategoryError> {
        let item = self.get_by_id(id).map(CategoryItem::from)?;
        Ok(item)
    }
}

impl Default for CategoryList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Category>> for CategoryList {
    fn from(categories: Vec<Category>) -> Self {
        Self { categories }
    }
}
