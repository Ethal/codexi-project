// src/file_management/snapshot.rs

use std::fs;

use crate::core::DataPaths;
use crate::file_management::{FileManagement, FileSnapshotError, StorageError, StoreEntity};
use crate::logic::codexi::Codexi;

impl FileManagement {
    /// List snapshot files
    pub fn list_snapshot(paths: &DataPaths) -> Result<Vec<String>, FileSnapshotError> {
        let snapshot_dir = &paths.snapshots_dir;
        let mut files = Vec::new();

        if snapshot_dir.exists() {
            for entry in fs::read_dir(snapshot_dir)? {
                let entry = entry?;
                let path = entry.path();
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                if stem.starts_with(DataPaths::APP_NAME)
                    && ext == DataPaths::EXT_SNAPSHOT
                    && let Some(name) = path.file_name()
                {
                    files.push(name.to_string_lossy().to_string());
                }
            }
        }
        files.sort();
        files.reverse();
        Ok(files)
    }
    /// Create a snapshot of the current codexi state
    pub fn create_snapshot(data: &Codexi, paths: &DataPaths) -> Result<String, FileSnapshotError> {
        // check id there some operation in the codexi if no -> Error
        if data.accounts.is_empty() {
            return Err(FileSnapshotError::NoAccount);
        }

        fs::create_dir_all(&paths.snapshots_dir)?;

        let snapshot = paths.snapshot_path();
        Self::write_storage(&StoreEntity::Codexi(data.clone()), &snapshot.path)?;

        Ok(snapshot.filename)
    }
    /// Restore a snapshot file
    pub fn restore_snapshot(paths: &DataPaths, filename: &str) -> Result<Codexi, FileSnapshotError> {
        let file_path = paths.snapshots_dir.join(filename);

        match Self::read_storage(&file_path)? {
            StoreEntity::Codexi(codexi) => Ok(codexi),
            _ => Err(FileSnapshotError::Storage(StorageError::InvalidStoreEntity {
                expected: "Codexi".to_string(),
            })),
        }
    }
    /// Clean the snapshot, keep the latest 5 by default or as per a value provide
    pub fn clean_snapshot(paths: &DataPaths, keep: Option<usize>) -> Result<(), FileSnapshotError> {
        let keep = keep.unwrap_or(5);

        let mut files = Self::list_snapshot(paths)?;
        files.reverse();

        if files.len() <= keep {
            return Ok(());
        }
        let snapshot_dir = &paths.snapshots_dir;
        let to_delete = &files[..files.len() - keep];

        for file in to_delete {
            let path = snapshot_dir.join(file);
            if path.exists() {
                fs::remove_file(&path)?;
            }
        }

        Ok(())
    }
}
