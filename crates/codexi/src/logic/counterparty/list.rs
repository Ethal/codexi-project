// src/logic/counterparty/list.rs

use chrono::Local;
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::{
    core::format_id,
    logic::counterparty::{Counterparty, CounterpartyError, CounterpartyKind},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CounterpartyList {
    pub list: Vec<Counterparty>,
}

impl CounterpartyList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, counterparty: Counterparty) -> Nulid {
        let id = counterparty.id;
        self.list.push(counterparty);
        id
    }

    pub fn create(
        &mut self,
        name: &str,
        kind: CounterpartyKind,
        note: Option<&str>,
    ) -> Result<Nulid, CounterpartyError> {
        let counterparty = Counterparty::new(name, kind, note)?;
        let id = self.add(counterparty);
        Ok(id)
    }

    pub fn update(
        &mut self,
        id: Nulid,
        name: &str,
        kind: CounterpartyKind,
        note: Option<&str>,
    ) -> Result<(), CounterpartyError> {
        let counterparty = self.get_by_id_mut(&id)?;
        counterparty.name = name.to_string();
        counterparty.kind = kind;
        counterparty.note = note.map(str::to_owned);
        Ok(())
    }

    pub fn get_by_id(&self, id: &Nulid) -> Result<&Counterparty, CounterpartyError> {
        self.list
            .iter()
            .find(|c| &c.id == id)
            .ok_or_else(|| CounterpartyError::CounterpartyNotFound(format_id(*id)))
    }

    pub fn get_by_id_mut(&mut self, id: &Nulid) -> Result<&mut Counterparty, CounterpartyError> {
        self.list
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or_else(|| CounterpartyError::CounterpartyNotFound(format_id(*id)))
    }

    pub fn counterparty_name_by_id(&self, id: &Nulid) -> Option<String> {
        self.list
            .iter()
            .find(|b| &b.id == id)
            .map(|b| b.name.clone())
    }

    pub fn terminate(&mut self, id: &Nulid) -> Result<(), CounterpartyError> {
        let today = Local::now().date_naive();
        let counterparty = self.get_by_id_mut(id)?;
        counterparty.terminated = Some(today);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Counterparty> {
        self.list.iter()
    }

    pub fn is_exist(&self, id: &Nulid) -> bool {
        self.list.iter().any(|c| &c.id == id)
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl Default for CounterpartyList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Counterparty>> for CounterpartyList {
    fn from(counterparties: Vec<Counterparty>) -> Self {
        Self {
            list: counterparties,
        }
    }
}
