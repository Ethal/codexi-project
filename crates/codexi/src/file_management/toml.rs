// src/file_management/toml.rs

use std::fs;
use std::path::Path;

use crate::core::CoreWarning;
use crate::exchange::ExchangeData;
use crate::file_management::{FileExchangeError, FileManagement};
use crate::logic::account::Account;

impl FileManagement {
    /// Export to toml
    pub fn export_toml(data: &Account, dir: &Path) -> Result<(), FileExchangeError> {
        let file_path = dir.join("codexi.toml");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let export = ExchangeData::export_data(data);
        let toml_str = toml::to_string_pretty(&export)?;
        fs::write(&file_path, toml_str)?;

        Ok(())
    }
    /// Import from toml
    pub fn import_toml(dir: &Path) -> Result<(Account, Vec<CoreWarning>), FileExchangeError> {
        let file_path = dir.join("codexi.toml");
        let content = fs::read_to_string(&file_path)?;
        let import: ExchangeData = toml::from_str(&content)?;
        let (mut account, warnings) = ExchangeData::import_data(&import)?;
        account.operations.sort_by_key(|o| o.date);

        Ok((account, warnings))
    }
}
