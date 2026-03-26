// src/file_management/json.rs

use std::fs;
use std::path::Path;

use crate::{
    exchange::Exchangeable,
    file_management::{ExchangeSerdeFormat, FileExchangeError, FileManagement},
    logic::codexi::Codexi,
};

impl FileManagement {
    /// Export to Json
    pub fn export_json<T: Exchangeable>(data: &T, dir: &Path) -> Result<(), FileExchangeError> {
        ExchangeSerdeFormat::Json.export(data, dir)
    }
    /// Imort from Json
    pub fn import_json<T: Exchangeable>(
        dir: &Path,
    ) -> Result<(T, Vec<T::Warning>), FileExchangeError> {
        ExchangeSerdeFormat::Json.import(dir)
    }

    /// Export special from json
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
