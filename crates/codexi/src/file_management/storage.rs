// src/file_management/storage.rs

use serde::Serialize;
use std::fs;
use std::path::Path;

use crate::file_management::{FileEnvelope, FileManagement, StorageError, StorageFormat, StoreEntity, checksum};
use crate::{CODEXI_DATA_FORMAT_VERSION, CODEXI_MAGIC};

impl FileManagement {
    /// Write to a storage file using Ciborium format
    pub fn write_storage<T: Serialize>(value: &T, path: &Path) -> Result<(), StorageError> {
        let mut payload = Vec::new();
        ciborium::into_writer(value, &mut payload)?;

        let env = FileEnvelope {
            magic: CODEXI_MAGIC,
            version: CODEXI_DATA_FORMAT_VERSION,
            format: StorageFormat::Ciborium,
            checksum: checksum(&payload),
            payload,
        };

        let mut bytes = Vec::new();
        ciborium::into_writer(&env, &mut bytes)?;
        fs::write(path, bytes)?;
        Ok(())
    }

    /// Read a storage file — supports Cbor (legacy) and Ciborium formats
    pub fn read_storage(path: &Path) -> Result<StoreEntity, StorageError> {
        let bytes = fs::read(path)?;

        // Cbor en premier — anciens fichiers, fallback Ciborium pour nouveaux
        let env: FileEnvelope = serde_cbor::from_slice(&bytes)
            .or_else(|_| ciborium::from_reader(bytes.as_slice()).map_err(StorageError::CiboriumDe))?;

        // Magic
        if env.magic != CODEXI_MAGIC {
            return Err(StorageError::InvalidMagic);
        }

        // Version
        if env.version != CODEXI_DATA_FORMAT_VERSION {
            return Err(StorageError::InvalidVersion {
                found: env.version,
                expected: CODEXI_DATA_FORMAT_VERSION,
            });
        }

        // Checksum
        if checksum(&env.payload) != env.checksum {
            return Err(StorageError::InvalidChecksum);
        }

        // Désérialisation du payload selon le format déclaré
        match env.format {
            StorageFormat::Ciborium => {
                let entity: StoreEntity = ciborium::from_reader(env.payload.as_slice())?;
                Ok(entity)
            }
            StorageFormat::Cbor => {
                let entity: StoreEntity = serde_cbor::from_slice(&env.payload)?;
                Ok(entity)
            }
            _ => Err(StorageError::InvalidStorageFormat { format: env.format }),
        }
    }
}

/*------------------------ TEST ------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ciborium_envelope() {
        let env = FileEnvelope {
            magic: *b"CODEXI",
            version: 3,
            format: StorageFormat::Ciborium,
            checksum: [0u8; 32],
            payload: vec![1, 2, 3],
        };
        let mut bytes = Vec::new();
        ciborium::into_writer(&env, &mut bytes).unwrap();
        let decoded: FileEnvelope = ciborium::from_reader(bytes.as_slice()).unwrap();
        assert_eq!(decoded.magic, *b"CODEXI");
        assert_eq!(decoded.version, 3);
    }

    #[test]
    fn test_write_read_roundtrip() {
        use crate::file_management::StoreEntity;
        use crate::logic::codexi::{Codexi, CodexiSettings};
        use std::path::PathBuf;

        let path = PathBuf::from("/tmp/test_codexi_roundtrip.dat");
        let codexi = Codexi::new(CodexiSettings::default()).unwrap();
        let entity = StoreEntity::Codexi(codexi);

        FileManagement::write_storage(&entity, &path).unwrap();
        let result = FileManagement::read_storage(&path).unwrap();

        std::fs::remove_file(&path).ok();

        assert!(matches!(result, StoreEntity::Codexi(_)));
    }
}
