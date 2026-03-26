// src/command/mod.rs

mod account;
mod admin;
mod bank;
mod category;
mod currency;
mod data;
mod history;
mod operation;
mod report;
mod root;

pub use account::AccountCommand;
pub use admin::{AdminCommand, TrashCommand};
pub use bank::BankCommand;
pub use category::CategoryCommand;
pub use currency::CurrencyCommand;
pub use data::{DataCommand, ExchangeFormat, ExchangeTypeCommand, SnapshotCommand};
pub use history::{ArchiveCommand, HistoryCommand};
pub use operation::OperationCommand;
pub use report::ReportCommand;
pub use root::{Cli, RootCommand};
