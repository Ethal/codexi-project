// src/file_management/json.rs

use std::fs;
use std::path::Path;

use crate::core::CoreWarning;
use crate::exchange::ExchangeData;
use crate::file_management::{FileExchangeError, FileManagement};
use crate::logic::account::Account;
use crate::logic::codexi::Codexi;

impl FileManagement {
    /// Export to json
    pub fn export_json(data: &Account, dir: &Path) -> Result<(), FileExchangeError> {
        let file_path = dir.join("codexi.json");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let export = ExchangeData::export_data(data);
        let json = serde_json::to_string_pretty(&export)?;
        fs::write(&file_path, json.as_bytes())?;

        Ok(())
    }
    /// Import from json
    pub fn import_json(dir: &Path) -> Result<(Account, Vec<CoreWarning>), FileExchangeError> {
        let file_path = dir.join("codexi.json");
        let content = fs::read_to_string(&file_path)?;
        let import: ExchangeData = serde_json::from_str(&content)?;
        let (mut account, warnings) = ExchangeData::import_data(&import)?;
        account.operations.sort_by_key(|o| o.date);

        Ok((account, warnings))
    }
    pub fn export_special_json(data: &Codexi, dir: &Path) -> Result<(), FileExchangeError> {
        let file_path = dir.join("codexi_all.json");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&data)?;
        fs::write(&file_path, json.as_bytes())?;

        Ok(())
    }
    /// Import special from json
    pub fn import_special_json(dir: &Path) -> Result<Codexi, FileExchangeError> {
        let file_path = dir.join("codexi_all.json");
        let content = fs::read_to_string(&file_path)?;
        let codexi: Codexi = serde_json::from_str(&content)?;

        Ok(codexi)
    }
}
