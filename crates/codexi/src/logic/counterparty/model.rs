// src/logic/counterparty/model.rs

use chrono::NaiveDate;
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::{
    core::validate_text_rules,
    logic::{
        counterparty::{CounterpartyError, CounterpartyKind},
        utils::{HasName, HasNulid},
    },
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Counterparty {
    pub id: Nulid,
    pub name: String,
    pub kind: CounterpartyKind,
    pub terminated: Option<NaiveDate>,
    pub note: Option<String>,
}

impl Counterparty {
    pub fn new(name: &str, kind: CounterpartyKind, note: Option<&str>) -> Result<Self, CounterpartyError> {
        let id = Nulid::new()?;

        let min = 3;
        let max = 20;
        if let Err(e) = validate_text_rules(name, min, max) {
            return Err(CounterpartyError::InvalidName(e));
        }

        Ok(Self {
            id,
            name: name.to_string(),
            kind,
            terminated: None,
            note: note.map(str::to_owned),
        })
    }

    pub fn is_active(&self) -> bool {
        self.terminated.is_none()
    }
}

impl HasNulid for Counterparty {
    fn id(&self) -> Nulid {
        self.id
    }
}

impl HasName for Counterparty {
    fn name(&self) -> &str {
        &self.name
    }
}
