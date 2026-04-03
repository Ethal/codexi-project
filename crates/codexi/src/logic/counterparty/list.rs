// src/logic/counterparty/list.rs

use chrono::NaiveDate;
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::{
    core::format_id,
    logic::counterparty::{Counterparty, CounterpartyError, CounterpartyKind},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CounterpartyList {
    pub counterparties: Vec<Counterparty>,
}

impl CounterpartyList {
    pub fn new() -> Self {
        Self {
            counterparties: Vec::new(),
        }
    }

    pub fn add(&mut self, counterparty: Counterparty) -> Nulid {
        let id = counterparty.id;
        self.counterparties.push(counterparty);
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
        self.counterparties
            .iter()
            .find(|c| &c.id == id)
            .ok_or_else(|| CounterpartyError::CounterpartyNotFound(format_id(*id)))
    }

    pub fn get_by_id_mut(&mut self, id: &Nulid) -> Result<&mut Counterparty, CounterpartyError> {
        self.counterparties
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or_else(|| CounterpartyError::CounterpartyNotFound(format_id(*id)))
    }

    pub fn counterparty_name_by_id(&self, id: &Nulid) -> Option<String> {
        self.counterparties
            .iter()
            .find(|b| &b.id == id)
            .map(|b| b.name.clone())
    }

    pub fn terminate(&mut self, id: &Nulid, date: &NaiveDate) -> Result<(), CounterpartyError> {
        let counterparty = self.get_by_id_mut(id)?;

        counterparty.terminated = Some(*date);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Counterparty> {
        self.counterparties.iter()
    }

    pub fn is_exist(&self, id: &Nulid) -> bool {
        self.counterparties.iter().any(|c| &c.id == id)
    }

    pub fn count(&self) -> usize {
        self.counterparties.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counterparties.is_empty()
    }
}

impl Default for CounterpartyList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Counterparty>> for CounterpartyList {
    fn from(counterparties: Vec<Counterparty>) -> Self {
        Self { counterparties }
    }
}
