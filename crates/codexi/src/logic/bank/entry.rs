// src/logic/bank/entry.rs

use nulid::Nulid;

use crate::logic::bank::{BankEntry, BankItem, BankList};

impl BankList {
    pub fn bank_entry(&self) -> BankEntry {
        let items: Vec<BankItem> = self.banks.iter().map(BankItem::from).collect();
        BankEntry { items }
    }

    pub fn bank_item(&self, id: &Nulid) -> Option<BankItem> {
        self.get_by_id(id).map(BankItem::from).ok()
    }
}
