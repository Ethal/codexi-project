// src/core/wallet/file_management/storage_format.rs

use std::fmt;
use serde::{Serialize, Deserialize};

/// Storage format
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageFormat {
    Cbor,

    #[serde(other)]
    Unknown,
}

/// Implement Display for StorageFormat
impl fmt::Display for StorageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageFormat::Cbor  => write!(f, "Cbor"),
            StorageFormat::Unknown  => write!(f, "Unknown"),
        }
    }
}
