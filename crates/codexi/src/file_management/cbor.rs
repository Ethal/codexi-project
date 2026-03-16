// src/file_management/cbor.rs

use serde::Serialize;
use serde_cbor;
use std::fs;
use std::path::Path;

use crate::{CODEXI_DATA_FORMAT_VERSION, CODEXI_MAGIC};

use crate::file_management::{
    FileEnvelope, FileManagement, StorageError, StorageFormat, StoreEntity, checksum,
};

impl FileManagement {
    /// Write to a storage file
    pub fn write_cbor<T: Serialize>(value: &T, path: &Path) -> Result<(), StorageError> {
        let payload = serde_cbor::to_vec(value)?;

        let env = FileEnvelope {
            magic: CODEXI_MAGIC,
            version: CODEXI_DATA_FORMAT_VERSION,
            format: StorageFormat::Cbor,
            checksum: checksum(&payload),
            payload,
        };

        let bytes = serde_cbor::to_vec(&env)?;
        fs::write(path, bytes)?;
        Ok(())
    }
    /// read a storage file
    pub fn read_cbor(path: &Path) -> Result<StoreEntity, StorageError> {
        let bytes = fs::read(path)?;
        let env: FileEnvelope = serde_cbor::from_slice(&bytes)?;

        if env.magic != CODEXI_MAGIC {
            return Err(StorageError::InvalidMagic);
        }
        if env.version != CODEXI_DATA_FORMAT_VERSION {
            return Err(StorageError::InvalidVersion {
                found: env.version,
                expected: CODEXI_DATA_FORMAT_VERSION,
            });
        }

        if checksum(&env.payload) != env.checksum {
            return Err(StorageError::InvalidChecksum);
        }

        match env.format {
            StorageFormat::Cbor => {
                let entity: StoreEntity = serde_cbor::from_slice(&env.payload)?;
                Ok(entity)
            }
            _ => Err(StorageError::InvalidStorageFormat { format: env.format }),
        }
    }
}
