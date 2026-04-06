// src/logic/category/list

use chrono::{Local, NaiveDate};
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::format_id;
use crate::logic::category::{Category, CategoryError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryList {
    pub list: Vec<Category>,
}

impl CategoryList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, category: Category) -> Nulid {
        let id = category.id;
        self.list.push(category);
        id
    }

    pub fn create(&mut self, name: &str, parent_id: Option<Nulid>, note: Option<&str>) -> Result<Nulid, CategoryError> {
        if let Some(v) = parent_id {
            let _ = self.get_by_id(&v)?;
        }
        let category = Category::new(name, parent_id, note)?;
        let id = self.add(category);
        Ok(id)
    }

    pub fn update(
        &mut self,
        id: Nulid,
        name: &str,
        parent_id: Option<Nulid>,
        terminated: Option<NaiveDate>,
        note: Option<&str>,
    ) -> Result<(), CategoryError> {
        if let Some(v) = parent_id {
            let _ = self.get_by_id(&v)?;
        }
        let category = self.get_by_id_mut(&id)?;
        category.name = name.into();
        category.parent_id = parent_id;
        category.terminated = terminated;
        category.note = note.map(str::to_owned);
        Ok(())
    }

    pub fn has_active_children(&self, id: &Nulid) -> bool {
        self.list
            .iter()
            .any(|c| c.parent_id.as_ref() == Some(id) && c.terminated.is_none())
    }

    pub fn terminate(&mut self, id: &Nulid) -> Result<(), CategoryError> {
        if self.has_active_children(id) {
            return Err(CategoryError::HasActiveChildren(format_id(*id)));
        }
        let today = Local::now().date_naive();
        let category = self.get_by_id_mut(id)?;
        category.terminated = Some(today);
        Ok(())
    }

    pub fn get_by_id(&self, id: &Nulid) -> Result<&Category, CategoryError> {
        self.list
            .iter()
            .find(|c| &c.id == id)
            .ok_or_else(|| CategoryError::CategoryNotFound(format_id(*id)))
    }

    pub fn get_by_id_mut(&mut self, id: &Nulid) -> Result<&mut Category, CategoryError> {
        self.list
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or_else(|| CategoryError::CategoryNotFound(format_id(*id)))
    }

    pub fn get_name_by_id(&self, id: &Nulid) -> Option<String> {
        self.list.iter().find(|c| &c.id == id).map(|c| c.name.clone())
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn is_exist(&self, id: &Nulid) -> bool {
        self.list.iter().any(|c| &c.id == id)
    }
}

impl Default for CategoryList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Category>> for CategoryList {
    fn from(categories: Vec<Category>) -> Self {
        Self { list: categories }
    }
}
