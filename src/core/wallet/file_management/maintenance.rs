// src/core/wallet/file_management/maintenance.rs

use anyhow::Result;
use std::fs;

use crate::core::wallet::codexi::Codexi;
use crate::core::helpers::get_data_dir;
use crate::core::wallet::file_management::cbor::CURRENT_VERSION;
use crate::core::wallet::file_management::storage_format::StorageFormat;

#[derive(Debug, Clone)]
pub struct ArchiveInfos{
    pub name: String,
    pub nb_op: usize,
}

#[derive(Debug, Clone)]
pub struct LedgerInfos {
    pub nb_archive_file: usize,
    pub archive_infos: Vec<ArchiveInfos>,
    pub nb_current_op: usize,
    pub version: String,
    pub storage_format: StorageFormat,
}

impl Codexi {
    /// Clear all data file related to codexi
    pub fn clear(snap: &[String], arch: &[String] ) -> Result<()> {

        let data_dir = get_data_dir()?;

        // delete the snaphot file
        if !snap.is_empty() {
            let snapshot_dir = data_dir.join("snapshots");
            for file in snap {
                let path = snapshot_dir.join(file);
                if path.exists() {
                    fs::remove_file(&path)?;
                    log::warn!("Deleted snapshot: {}", file);
                }
            }
        } else {
            log::info!("No snapshot file");
        }
        // delete the archive file
        if !arch.is_empty() {
            let archive_dir = data_dir.join("archives");
            for file in arch {
                let path = archive_dir.join(file);
                if path.exists() {
                    fs::remove_file(&path)?;
                    log::warn!("Deleted arch: {}", file);
                }
            }
        } else {
            log::info!("No archive file");
        }
        // Delete the codexi.dat file
        let codexi_file = data_dir.join("codexi.dat");
        if codexi_file.exists() {
            fs::remove_file(&codexi_file)?;
            log::warn!("Deleted dat file: {:?}", codexi_file);
        } else {
            log::info!("No codexi.dat");
        }
        Ok(())
    }
    /// Get ledger infos
    pub fn ledger_infos(&self, arch: &[String] ) -> Result<LedgerInfos> {

        let mut archive_infos = Vec::new();
        let nb_archive_file = arch.len();

        for f in arch {
            let data = Self::load_archive(&f)?;
            let nb_op = data.operations.len();
            let infos = ArchiveInfos { name: f.into(), nb_op: nb_op };
            archive_infos.push(infos);
        }

        let nb_current_op = self.operations.len();
        let result = LedgerInfos {
            nb_archive_file: nb_archive_file,
            archive_infos: archive_infos,
            nb_current_op: nb_current_op,
            version: CURRENT_VERSION.to_string(),
            storage_format: StorageFormat::Cbor,
        };
        Ok(result)
    }
}
