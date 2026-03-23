// src/logic/category/list

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::format_id;
use crate::logic::category::{Category, CategoryError};

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

    pub fn category_name_by_id(&self, id: &Nulid) -> Option<String> {
        self.categories
            .iter()
            .find(|c| &c.id == id)
            .map(|c| c.name.clone())
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
