// src/file_management/mod.rs

mod archive;
mod backup;
mod envelope;
mod error;
mod html;
mod json;
mod maintenance;
mod snapshot;
mod state;
mod storage;
mod toml;

pub struct FileManagement;

pub use error::{
    FileArchiveError, FileBackupError, FileCodexiError, FileExchangeError, FileMaintenanceError,
    FileManagementError, FileSnapshotError, StorageError,
};

pub use envelope::{FileEnvelope, StorageFormat, StoreEntity, checksum};
pub use maintenance::CodexiInfos;
