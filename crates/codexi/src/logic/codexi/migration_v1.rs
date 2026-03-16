// src/logic/codexi/migration_v1.rs

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

use crate::CODEXI_MAGIC;
use crate::core::DataPaths;
use crate::file_management::{
    FileCodexiError, FileEnvelope, StorageError, StorageFormat, checksum,
};
use crate::logic::codexi::migration_v2::CodexiV2;
use crate::logic::codexi::migration_v2::OperationKindV2;
use crate::logic::codexi::migration_v2::OperationV2;
use crate::logic::codexi::migration_v2::SystemKindV2;
use crate::logic::operation::OperationFlow; // no change from V1
use crate::logic::operation::RegularKind; // no change from V1

#[derive(Debug, Error)]
pub enum MigrationV1Error {
    #[error("SYS_IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("SYS_BINCODE: {0}")]
    Bincode(#[from] bincode::Error),
    #[error("SYS_CODEXI: {0}")]
    FileCodexi(#[from] FileCodexiError),
    #[error("SYS_STORAGE_FORMAT: {0}")]
    InvalidStorageFormat(#[from] StorageError),
    #[error("SYS_IO: No codexi.dat file")]
    NoFile,
}

#[derive(Serialize, Deserialize)]
pub struct CodexiV1 {
    pub operations: Vec<OperationV1>,
}

#[derive(Serialize, Deserialize)]
pub struct OperationV1 {
    pub kind: OperationKindV1,
    pub flow: OperationFlow,
    pub date: NaiveDate,
    pub amount: f64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationKindV1 {
    System(SystemKindV1),
    Regular(RegularKind),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SystemKindV1 {
    Init,
    Adjust,
    Close,
    Void,
}

/// Migrate current codexi and associated archives
pub fn migrate_v1(paths: &DataPaths) -> Result<(), MigrationV1Error> {
    let file_path = &paths.main_file;
    let old_codexi = load_format_v1(file_path)?;
    let codexi: CodexiV2 = migrate_v1_to_v2(old_codexi);
    save_format_v2(&paths.root, &codexi)?;

    let archives = list_archive_v1(paths)?;

    for arch in &archives {
        let file_path = paths.archives_dir.join(arch);
        let old_codexi = load_format_v1(&file_path)?;
        let codexi: CodexiV2 = migrate_v1_to_v2(old_codexi);
        save_format_v2(&file_path, &codexi)?;
    }
    Ok(())
}

/// load codexi v1
fn load_format_v1(file_path: &Path) -> Result<CodexiV1, MigrationV1Error> {
    if !file_path.exists() {
        return Err(MigrationV1Error::NoFile);
    }

    let bytes = std::fs::read(file_path)?;
    let old = bincode::deserialize(&bytes)?;

    Ok(old)
}

/// save codexi v2
fn save_format_v2<T: Serialize>(path: &Path, value: &T) -> Result<(), StorageError> {
    let payload = serde_cbor::to_vec(value)?;

    let env = FileEnvelope {
        magic: CODEXI_MAGIC,
        version: 2,
        format: StorageFormat::Cbor,
        checksum: checksum(&payload),
        payload,
    };

    let bytes = serde_cbor::to_vec(&env)?;
    fs::write(path, bytes)?;
    Ok(())
}

/// Migrate structure from V1 to V2
fn migrate_v1_to_v2(old: CodexiV1) -> CodexiV2 {
    let operations: Vec<OperationV2> = old
        .operations
        .into_iter()
        .enumerate()
        .map(|(idx, op)| {
            let amount = Decimal::from_f64(op.amount).expect("Invalid f64 amount during migration");
            let kind = match op.kind {
                OperationKindV1::System(SystemKindV1::Close) => {
                    OperationKindV2::System(SystemKindV2::Close)
                }
                OperationKindV1::System(SystemKindV1::Init) => {
                    OperationKindV2::System(SystemKindV2::Init)
                }
                OperationKindV1::System(SystemKindV1::Adjust) => {
                    OperationKindV2::System(SystemKindV2::Adjust)
                }
                OperationKindV1::System(SystemKindV1::Void) => {
                    OperationKindV2::System(SystemKindV2::Void)
                }
                OperationKindV1::Regular(RegularKind::Transaction) => {
                    OperationKindV2::Regular(RegularKind::Transaction)
                }
                OperationKindV1::Regular(RegularKind::Transfer) => {
                    OperationKindV2::Regular(RegularKind::Transfer)
                }
                OperationKindV1::Regular(RegularKind::Fee) => {
                    OperationKindV2::Regular(RegularKind::Fee)
                }
                OperationKindV1::Regular(RegularKind::Refund) => {
                    OperationKindV2::Regular(RegularKind::Refund)
                }
            };

            OperationV2 {
                id: idx,
                kind,
                flow: op.flow,
                date: op.date,
                amount,
                description: op.description,
                void_of: None,
            }
        })
        .collect();

    let next_op_id = operations
        .iter()
        .map(|op| op.id)
        .max()
        .map(|id| id + 1)
        .unwrap_or(0);

    CodexiV2 {
        operations,
        next_op_id,
    }
}

/// Old function to get the archive list
fn list_archive_v1(paths: &DataPaths) -> Result<Vec<String>, MigrationV1Error> {
    let archive_dir = &paths.archives_dir;
    let mut files = Vec::new();

    if archive_dir.exists() {
        for entry in fs::read_dir(archive_dir)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            if file_name.starts_with("codexi_") && file_name.ends_with(".cld") {
                files.push(file_name);
            }
        }
    }
    files.sort();
    Ok(files)
}
