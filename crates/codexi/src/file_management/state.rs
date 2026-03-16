// src/file_management/codexi.rs

use std::fs;

use crate::core::DataPaths;
use crate::file_management::{FileCodexiError, FileManagement, StorageError, StoreEntity};
use crate::logic::codexi::{Codexi, CodexiSettings};

impl FileManagement {
    /// Save codexi.dat (ledger)
    pub fn save_current_state(data: &Codexi, paths: &DataPaths) -> Result<(), FileCodexiError> {
        let file_path = &paths.main_file;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        Self::write_cbor(&StoreEntity::Codexi(data.clone()), file_path)?;

        Ok(())
    }
    /// Load codexi.dat (ledger)
    pub fn load_current_state(paths: &DataPaths) -> Result<Codexi, FileCodexiError> {
        let file_path = &paths.main_file;
        if !file_path.exists() {
            let settings = CodexiSettings::load_or_create()?;
            return Ok(Codexi::new(settings)?);
        }

        match Self::read_cbor(file_path)? {
            StoreEntity::Codexi(mut codexi) => {
                for account in codexi.accounts.iter_mut() {
                    account.refresh_anchors();
                    account.audit()?;
                }
                Ok(codexi)
            }
            _ => Err(FileCodexiError::Storage(StorageError::InvalidStoreEntity {
                expected: "Codexi".into(),
            })),
        }
    }
}
