// src/core/wallet/file_management/snapshot.rs

use anyhow::Result;
use std::fs;
use chrono::Local;

use crate::core::wallet::codexi::Codexi;
use crate::core::helpers::get_data_dir;

impl Codexi {
    /// List snapshot files
    pub fn list_snapshot() -> Result<Vec<String>> {

        let data_dir = get_data_dir()?;
        let snapshot_dir = data_dir.join("snapshots");
        let mut files = Vec::new();

        if snapshot_dir.exists() {
            for entry in fs::read_dir(snapshot_dir)? {
                let entry = entry?;
                let file_name = entry.file_name().to_string_lossy().to_string();

                if file_name.starts_with("codexi_") && file_name.ends_with(".snp") {
                    files.push(file_name);
                }
            }
        }
        files.sort();
        Ok(files)
    }
    /// Create a snapshot of the current codexi state
    pub fn snapshot(&self) -> Result<()> {

        let data_dir =  get_data_dir()?;
        let snapshot_dir = data_dir.join("snapshots");
        fs::create_dir_all(&snapshot_dir)?;

        // Filename : codexi_YYYYMMDD_HHMMSS.snp
        let now = Local::now();
        let filename = format!("codexi_{}.snp", now.format("%Y%m%d_%H%M%S"));
        let snapshot_path = snapshot_dir.join(filename);

        Self::write_cbor(&snapshot_path, self)?;

        log::info!("snapshot done to {:?}", snapshot_path);
        Ok(())
    }
    /// Restore a snapshot file
    pub fn restore_snapshot(filename: &str) -> Result<Self> {
        let data_dir = get_data_dir()?;
        let file_path = data_dir.join("snapshots").join(filename);

        let codexi = Self::read_cbor(&file_path)?;

        log::info!("Snapshot {} restored", file_path.display());

        Ok(codexi)
    }
    /// Clean the snapshot , keep the latest 5 by default or as per a value provide
    pub fn clean_snapshot(files: &[String], keep: Option<usize>) -> Result<()> {
        let keep = keep.unwrap_or(5);

        if files.len() <= keep {
            log::info!("No snapshot deleted, {} snapshot(s),  request to keep {}. ", files.len(), keep);
            return Ok(());
        }

        let data_dir = get_data_dir()?;
        let snapshot_dir = data_dir.join("snapshots");

        let to_delete = &files[..files.len() - keep];

        for file in to_delete {
            let path = snapshot_dir.join(file);
            if path.exists() {
                fs::remove_file(&path)?;
                log::info!("Deleted old snapshot: {}", file);
            }
        }

        Ok(())
    }

}
