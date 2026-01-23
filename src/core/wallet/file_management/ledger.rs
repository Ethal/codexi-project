// src/core/wallet/file_management/ledger.rs

use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::core::wallet::codexi::Codexi;

impl Codexi {
    /// Save codexi.dat
    pub fn save_current_ledger(&self, dir: &Path) -> Result<()> {
        let file_path = dir.join("codexi.dat");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Self::write_cbor(&file_path, self)?;
        Ok(())
    }
    /// Load codexi.dat
    pub fn load_current_ledger(dir: &Path) -> Result<Self> {
        let file_path = dir.join("codexi.dat");
        if !file_path.exists() {
            log::warn!("No codexi file, considered to perform a system init command (codexi init YYYY-MM-DD AMOUNT)");
            return Ok(Self::default());
        }

        let codexi = Self::read_cbor(&file_path)?;

        Ok(codexi)
    }
}
