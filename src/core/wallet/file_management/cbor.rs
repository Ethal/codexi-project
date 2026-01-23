// src/core/wallet/file_management/cbor.rs

use thiserror::Error;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_cbor;
use sha2::{Sha256, Digest};

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::file_management::storage_format::StorageFormat;

/// Error type for Storage
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid format, Try a `codexi maintenance migrate <VERSION>` Error: {0}")]
    InvalidFormat(#[from] serde_cbor::Error),

    #[error("Not a codexi data, Try a `codexi maintenance migrate <VERSION>`")]
    InvalidMagic,

    #[error("Invalid version: {version}, Try a `codexi maintenance migrate <VERSION>`: ")]
    InvalidVersion { version: u16 },

    #[error("Invalid checksum (file corrupted), Try a `codexi maintenance migrate <VERSION>`")]
    InvalidChecksum,

    #[error("Unsupported storage format {format}, Try a `codexi maintenance migrate <VERSION>`")]
    InvalidStorageFormat { format: StorageFormat },

}

const MAGIC: [u8; 6] = *b"CODEXI";
pub const CURRENT_VERSION: u16 = 2;


#[derive(Serialize, Deserialize)]
pub struct FileEnvelope {
    pub magic: [u8; 6],          // b"CODEXI"
    pub version: u16,            // 2
    pub format: StorageFormat,   // Cbor,
    pub checksum: [u8; 32],      // SHA-256 du payload
    pub payload: Vec<u8>,
}

fn checksum(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

impl Codexi {
    /// Write to a storage file
    pub fn write_cbor<T: Serialize>(path: &Path, value: &T) -> Result<(), StorageError > {
        let payload = serde_cbor::to_vec(value)?;

        let env = FileEnvelope {
            magic: MAGIC,
            version: CURRENT_VERSION,
            format: StorageFormat::Cbor,
            checksum: checksum(&payload),
            payload,
        };

        let bytes = serde_cbor::to_vec(&env)?;
        fs::write(path, bytes)?;
        Ok(())
    }
    /// read a storage file
    pub fn read_cbor(path: &Path) -> Result<Self, StorageError> {
        let bytes = fs::read(path)?;
        let env: FileEnvelope = serde_cbor::from_slice(&bytes)?;

        if env.magic != MAGIC {
            return Err(StorageError::InvalidMagic);
        }
        if env.version != CURRENT_VERSION {
            return Err(StorageError::InvalidVersion { version: env.version });
        }

        if checksum(&env.payload) != env.checksum {
            return Err(StorageError::InvalidChecksum);
        }

        match env.format {
            StorageFormat::Cbor => Ok(serde_cbor::from_slice(&env.payload)?),
            _ => return Err(StorageError::InvalidStorageFormat { format: env.format } ),
        }

    }
}
