// src/file_management/archive.rs

use chrono::NaiveDate;
use nulid::Nulid;
use std::fs;

use crate::core::DataPaths;
use crate::file_management::{FileArchiveError, FileManagement, StorageError, StoreEntity};
use crate::logic::account::AccountArchive;

impl FileManagement {
    /// List archive files
    /// The archive files are stored in the "archives" subdirectory of the data directory.
    pub fn list_archive(paths: &DataPaths, account_id: Nulid) -> Result<Vec<String>, FileArchiveError> {
        let archive_dir = paths.archive_dir(&account_id);

        let mut files = Vec::new();

        if archive_dir.exists() {
            for entry in fs::read_dir(archive_dir)? {
                let entry = entry?;
                let path = entry.path();
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                if stem.contains(&format!("_{}_", DataPaths::APP_NAME))
                    && ext == DataPaths::EXT_ARCHIVE
                    && let Some(name) = path.file_name()
                {
                    files.push(name.to_string_lossy().to_string());
                }
            }
        }
        files.sort();
        Ok(files)
    }
    /// Save an archive file
    pub fn save_archive(data: &AccountArchive, paths: &DataPaths) -> Result<String, FileArchiveError> {
        let account_id = data.account_id;
        let checkpoint_date = data.checkpoint_date;

        fs::create_dir_all(paths.archive_dir(&account_id))?;

        let archive = paths.archive_path(&account_id, &checkpoint_date);
        Self::write_storage(&StoreEntity::AccountArchive(data.clone()), &archive.path)?;

        Ok(archive.filename)
    }
    /// Load an archive file (for view only)
    pub fn load_archive(
        account_id: Nulid,
        checkpoint_date: NaiveDate,
        paths: &DataPaths,
    ) -> Result<AccountArchive, FileArchiveError> {
        let archive = paths.archive_path(&account_id, &checkpoint_date);

        match Self::read_storage(&archive.path)? {
            StoreEntity::AccountArchive(archive) => Ok(archive),
            _ => Err(FileArchiveError::Storage(StorageError::InvalidStoreEntity {
                expected: "AccountArchive".to_string(),
            })),
        }
    }
}
