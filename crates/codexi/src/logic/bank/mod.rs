// src/logic/bank/mod.rs

mod dto;
mod error;
mod list;
mod model;

pub use dto::{BankEntry, BankItem};
pub use error::BankError;
pub use list::BankList;
pub use model::Bank;
