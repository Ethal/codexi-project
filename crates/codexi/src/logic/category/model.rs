// src/logic/category/category.rs

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::{format_id, validate_text_rules};
use crate::logic::category::{CategoryError, CategoryItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: Nulid,    // Nulid
    pub name: String, // ex: "Food"
    pub note: Option<String>,
}

impl Category {
    pub fn new(name: &str, note: Option<&str>) -> Result<Self, CategoryError> {
        let id = Nulid::new()?;
        let min = 3;
        let max = 10;
        if let Err(e) = validate_text_rules(name, min, max) {
            return Err(CategoryError::InvalidName(e));
        }

        Ok(Self {
            id,
            name: name.to_string(),
            note: note.map(str::to_owned),
        })
    }
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
