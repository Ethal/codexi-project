// src/core/wallet/file_management/toml.rs

use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::imex::LedgerExport;


impl Codexi {
    /// Export to toml
    pub fn export_toml(&self, dir: &Path) -> Result<()> {
        let file_path = dir.join("codexi.toml");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let export = self.export_ledger();
        let toml_str = toml::to_string_pretty(&export)
            .map_err(|e| anyhow!("{}", e))?;

        fs::write(&file_path, toml_str)?;
        log::info!("Export toml saved to {:?}", file_path);
        Ok(())
    }
    /// Import from toml
    pub fn import_toml(dir: &Path) -> Result<Self> {
        let file_path = dir.join("codexi.toml");

        let content = fs::read_to_string(&file_path)?;

        let import: LedgerExport = toml::from_str(&content)
            .map_err(|e| anyhow!("Import TOML: {}", e))?;
        let mut codexi = Self::import_ledger(&import)?;

        codexi.operations.sort_by_key(|o| o.date);
        log::info!("Import toml: {:?} loaded.", file_path);
        Ok(codexi)
    }
}
