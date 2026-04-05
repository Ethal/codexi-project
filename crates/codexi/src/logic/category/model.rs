// src/logic/category/category.rs

use chrono::NaiveDate;
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::validate_text_rules;
use crate::logic::category::CategoryError;
use crate::logic::utils::{HasName, HasNulid};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: Nulid,    // Nulid
    pub name: String, // ex: "Food"
    pub note: Option<String>,
    pub parent_id: Option<Nulid>,
    pub terminated: Option<NaiveDate>,
}

impl Category {
    pub fn new(
        name: &str,
        parent: Option<Nulid>,
        note: Option<&str>,
    ) -> Result<Self, CategoryError> {
        let id = Nulid::new()?;
        let min = 3;
        let max = 20;
        if let Err(e) = validate_text_rules(name, min, max) {
            return Err(CategoryError::InvalidName(e));
        }

        Ok(Self {
            id,
            name: name.to_string(),
            note: note.map(str::to_owned),
            parent_id: parent,
            terminated: None,
        })
    }

    pub fn is_active(&self) -> bool {
        self.terminated.is_none()
    }
}

impl HasNulid for Category {
    fn id(&self) -> Nulid {
        self.id
    }
}

impl HasName for Category {
    fn name(&self) -> &str {
        &self.name
    }
}
