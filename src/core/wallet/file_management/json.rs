// src/core/wallet/file_management/json.rs

use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::imex::LedgerExport;

impl Codexi {
    /// Export to json
    pub fn export_json(&self, dir: &Path) -> Result<()> {
        let file_path = dir.join("codexi.json");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let export = self.export_ledger();

        let json = serde_json::to_string_pretty(&export)
            .map_err(|e| anyhow!("{}", e))?;

        fs::write(&file_path, json.as_bytes())?;
        log::info!("Export json saved to {:?}", file_path);
        Ok(())
    }
    /// Import from json
    pub fn import_json(dir: &Path) -> Result<Self> {
        let file_path = dir.join("codexi.json");

        let content = fs::read_to_string(&file_path)?;

        let import: LedgerExport = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Import JSON: {}", e))?;

        let mut codexi = Self::import_ledger(&import)?;

        codexi.operations.sort_by_key(|o| o.date);
        log::info!("Import json: {:?} loaded.", file_path);
        Ok(codexi)
    }
}
