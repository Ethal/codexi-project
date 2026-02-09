// src/core/wallet/file_management/archive.rs

use anyhow::Result;
use std::fs;

use crate::core::wallet::codexi::Codexi;
use crate::core::helpers::get_data_dir;

impl Codexi {
    /// List archive files
    /// The archive files are stored in the "archives" subdirectory of the data directory.
    pub fn list_archives() -> Result<Vec<String>> {
        let data_dir = get_data_dir()?;
        let archive_dir = data_dir.join("archives");
        let mut files = Vec::new();

        if archive_dir.exists() {
            for entry in fs::read_dir(archive_dir)? {
                let entry = entry?;
                let file_name = entry.file_name().to_string_lossy().to_string();

                if file_name.starts_with("codexi_") && file_name.ends_with(".cld") {
                    files.push(file_name);
                }
            }
        }
        files.sort();
        Ok(files)
    }
    /// Save an archive file
    pub fn save_archive(&self, data: &Codexi, close_date_str: &str ) -> Result<()> {

        let data_dir =  get_data_dir()?;
        let archive_dir = data_dir.join("archives");
        fs::create_dir_all(&archive_dir)?;

        // Filename : codexi_YYYYMMDD.cld
        let filename = format!("codexi_{}.cld", close_date_str);
        let archive_path = archive_dir.join(filename);

        Self::write_cbor(&archive_path, &data)?;

        log::info!("Archived to {:?}", archive_path);
        Ok(())
    }
    /// Load an archive file (for view only)
    pub fn load_archive(filename: &str) -> Result<Self> {
        let data_dir = get_data_dir()?;
        let file_path = data_dir.join("archives").join(filename);

        let codexi = Self::read_cbor(&file_path)?;

        Ok(codexi)
    }
}
