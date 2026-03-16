// src/logic/bank/list.rs

use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::core::format_id;
use crate::logic::bank::{Bank, BankEntry, BankError, BankItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankList {
    pub banks: Vec<Bank>,
}

impl BankList {
    pub fn new() -> Self {
        Self { banks: Vec::new() }
    }

    pub fn add(&mut self, bank: Bank) -> Nulid {
        let id = bank.id;
        self.banks.push(bank);
        id
    }

    pub fn create(
        &mut self,
        name: &str,
        branch: Option<&str>,
        note: Option<&str>,
    ) -> Result<Nulid, BankError> {
        let bank = Bank::new(name, branch, note)?;
        let id = self.add(bank);
        Ok(id)
    }

    pub fn update(&mut self, id: Nulid, note: Option<&str>) -> Result<(), BankError> {
        let bank = self.get_by_id_mut(&id)?;
        bank.note = note.map(str::to_owned);
        Ok(())
    }

    pub fn get_by_id(&self, id: &Nulid) -> Result<&Bank, BankError> {
        self.banks
            .iter()
            .find(|c| &c.id == id)
            .ok_or_else(|| BankError::BankNotFound(format_id(*id)))
    }

    pub fn get_by_id_mut(&mut self, id: &Nulid) -> Result<&mut Bank, BankError> {
        self.banks
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or_else(|| BankError::BankNotFound(format_id(*id)))
    }

    pub fn is_exist(&self, id: &Nulid) -> bool {
        self.banks.iter().any(|c| &c.id == id)
    }

    pub fn count(&self) -> usize {
        self.banks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.banks.is_empty()
    }

    pub fn bank_entry(&self) -> BankEntry {
        let items: Vec<BankItem> = self.banks.iter().map(BankItem::from).collect();
        BankEntry { items }
    }

    pub fn bank_item(&self, id: &Nulid) -> Result<BankItem, BankError> {
        let item = self.get_by_id(id).map(BankItem::from)?;
        Ok(item)
    }
}

impl Default for BankList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Bank>> for BankList {
    fn from(banks: Vec<Bank>) -> Self {
        Self { banks }
    }
}
