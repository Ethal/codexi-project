// src/core/wallet/file_management/backup.rs

use anyhow::{Result, anyhow};
use std::fs;
use std::io;
use std::fs::File;
use std::path::Path;

use zip::write::{FileOptions, ZipWriter};
use zip::ZipArchive;
use walkdir::WalkDir;

use crate::core::wallet::codexi::Codexi;
use crate::core::helpers::get_data_dir;

impl Codexi {
    /// Creates a complete ZIP backup of the application's data directory.
    /// The `target_path` is the FULL path where the ZIP file should be written.
    /// It includes all files except internal snapshots.
    pub fn backup(target_path: &Path) -> Result<()> {
        let data_dir = get_data_dir()?;
        let internal_snapshot_dir = data_dir.join("snapshots");

        // The data directory SHALL exist and contain at least the codexi.dat file
        if !data_dir.exists() {
            return Err(anyhow!("The data directory ({}) does not exist.", data_dir.display()));
        }

        // 2. Create the ZIP file
        let file = File::create(target_path)?;
        let mut zip = ZipWriter::new(file);

        // Standard options for compression (Deflated)
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755); // Standard Unix permissions if necessary

        // 3. Iterate the data directory (including codexi.dat and archives/, exclude snapshot)
        for entry in WalkDir::new(&data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.starts_with(&internal_snapshot_dir) && path != internal_snapshot_dir {
                continue;
            }

            // Paths in the ZIP to be relative to the data_dir, not absolute.
            let name_in_zip = path.strip_prefix(&data_dir)
                .map_err(|_| anyhow!("Failure to calculate relative path for archive."))?
                .to_path_buf();

            if path.is_file() {
                // Add teh ZIP file
                let name_in_zip_str = name_in_zip.to_str().ok_or_else(|| anyhow!("Path invalid (non-UTF8)."))?;

                // Avoid adding temporary or locked files if present (non-standard)
                if name_in_zip_str.contains(".temp") { continue; }

                zip.start_file(name_in_zip_str, options)?;
                io::copy(&mut File::open(path)?, &mut zip)?;

            } else if path.is_dir() && name_in_zip.as_os_str().len() != 0 {
                // Add the directory (only if it is not the root directory itself)
                let name_in_zip_str = name_in_zip.to_str().ok_or_else(|| anyhow!("Path invalid (non-UTF8)."))?;
                zip.add_directory(name_in_zip_str, options)?;
            }
        }

        zip.finish()?;
        log::info!("Full backup successful to: {}", target_path.display());
        Ok(())
    }
    /// Restores the contents of a full ZIP backup to the application's data directory.
    /// The `zip_path` is the FULL path to the backup ZIP file.
    /// Existing files in the data directory will be overwritten.
    pub fn restore(zip_path: &Path) -> Result<()> {

        let data_dir = get_data_dir()?;
        let file = File::open(zip_path)?;

        // Attempting to create the ZIP archive
        let mut archive = ZipArchive::new(file)?;

        log::warn!("Restoration in progress. Existing files in {} will be overwritten.", data_dir.display());

        // Iterate over all files in the archive
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            // The destination path is data_dir + the path to the file in the ZIP archive
            let outpath = data_dir.join(file.mangled_name());

            if file.is_dir() {
                // Create the directories (e.g., 'archives/')
                fs::create_dir_all(&outpath)?;
            } else if file.is_file() {

                // Ensure that the parent directory exists (in the case of files in 'archives/')
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }

                // Write the contents of the file
                let mut outfile = File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;

                log::info!("Restore : {}", outpath.file_name().unwrap_or_default().to_string_lossy());
            }
        }

        log::info!("Complete restore successful. The codexi has been reloaded from the backup.");
        Ok(())
    }
}
