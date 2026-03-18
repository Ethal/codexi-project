// src/file_management/envelope.rs

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

use crate::logic::account::AccountArchive;
use crate::logic::codexi::Codexi;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEnvelope {
    pub magic: [u8; 6],        // b"CODEXI"
    pub version: u16,          // 3
    pub format: StorageFormat, // Cbor,
    pub checksum: [u8; 32],    // SHA-256 du payload
    pub payload: Vec<u8>,
}

pub fn checksum(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize)]
pub enum StoreEntity {
    Codexi(Codexi),
    AccountArchive(AccountArchive),
}

/// Storage format
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum StorageFormat {
    Cbor,
    Ciborium,
    Unknown,
}

impl StorageFormat {
    /// Get the string representation of the specific storage format
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageFormat::Cbor => "Cbor",
            StorageFormat::Ciborium => "Ciborium",
            StorageFormat::Unknown => "Unknown",
        }
    }
}

/// Implement From<StorageFormat> for &'static str
impl From<StorageFormat> for &'static str {
    fn from(t: StorageFormat) -> Self {
        t.as_str()
    }
}

/// Implement Display for StorageFormat
impl fmt::Display for StorageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
