// src/logic/codexi/mod.rs

mod dto;
mod error;
mod init_data;
mod migration_v1;
mod migration_v2;
mod model;
mod settings;
mod transfer;

pub use dto::{AccountEntry, AccountItem, CodexiContext};
pub use error::CodexiError;
pub use init_data::{default_banks, default_categories, default_currencies};
pub use migration_v1::migrate_v1;
pub use migration_v2::migrate_v2;
pub use model::Codexi;
pub use settings::CodexiSettings;
