// src/logic/codexi/migration_v2.rs

use chrono::NaiveDate;
use nulid::Nulid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

use crate::CODEXI_MAGIC;
use crate::core::CoreWarning;
use crate::core::{CoreError, DataPaths, parse_date};
use crate::file_management::{
    FileCodexiError, FileEnvelope, FileExchangeError, FileManagement, StorageError, StorageFormat,
    checksum,
};
use crate::logic::account::{Account, AccountError, AccountType}; // new structure
use crate::logic::codexi::{Codexi, CodexiError, CodexiSettings}; // new structure
use crate::logic::operation::{Operation, OperationContext}; // new structure
use crate::logic::operation::{
    OperationFlow, OperationKind, OperationLinks, OperationMeta, RegularKind, SystemKind,
};

#[derive(Debug, Error)]
pub enum MigrationV2Error {
    #[error("SYS_IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("VAL_CBOR: {0}")]
    Cbor(#[from] serde_cbor::Error),
    #[error("SYS_ID: {0}")]
    Nulid(#[from] nulid::Error),

    #[error("SYS_FILE_CODEXI: {0}")]
    FileCodexi(#[from] FileCodexiError),
    #[error("SYS_STORAGE: {0}")]
    Storage(#[from] StorageError),
    #[error("SYS_EXCHANGE: {0}")]
    Exchange(#[from] FileExchangeError),
    #[error("SYS_COMMON: {0}")]
    Common(#[from] CoreError),

    #[error("SYS_CODEXI: {0}")]
    Codexi(#[from] CodexiError),
    #[error("SYS_ACCOUNT: {0}")]
    Account(#[from] AccountError),

    #[error("SYS_IO: No codexi.dat file")]
    NoFile,
    #[error("VAL_MIGRATION_BALANCE: {0}")]
    InvalidData(String),
}

/// Structure V2
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CodexiV2 {
    pub operations: Vec<OperationV2>,
    pub next_op_id: usize,
}

/// Structure V2
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationV2 {
    pub id: usize,
    pub kind: OperationKindV2,
    pub flow: OperationFlow,
    pub date: NaiveDate,
    pub amount: Decimal,
    pub description: String,
    pub void_of: Option<usize>,
}

/// Structure V2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationKindV2 {
    System(SystemKindV2),
    Regular(RegularKind),
}

/// Structure V2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemKindV2 {
    Init,
    Adjust,
    Close, // change to Checkpoint
    Void,
}

/// Migrate current codexi and associated archives
pub fn migrate_v2(paths: &DataPaths) -> Result<Vec<CoreWarning>, MigrationV2Error> {
    let mut old_codexi_full = CodexiV2::default();

    let file_path = &paths.main_file;
    let old_codexi_dat = load_format_v2(file_path)?;
    let old_bal = balance_v2(&old_codexi_dat.operations);

    old_codexi_full.operations.extend(old_codexi_dat.operations);

    let archives = list_archive_v2(paths)?;
    for arch in &archives {
        let file_path = paths.archives_dir.join(arch);
        let old_codexi_arch = load_format_v2(&file_path)?;
        old_codexi_full
            .operations
            .extend(old_codexi_arch.operations);
    }

    // full new codexi(ledger) with all operations migrated
    let (codexi, warnings) = migrate_v2_to_v3(old_codexi_full, &old_bal)?;

    FileManagement::save_current_state(&codexi, paths)?;
    FileManagement::export_special_json(&codexi, &paths.tmp_dir)?;

    // TODO find a solution to avoid many disk acces load/save
    for arch in &archives {
        let date_str = &arch[7..17];
        let date = parse_date(date_str)?;
        let mut codexi = FileManagement::load_current_state(paths)?;
        let account = codexi.get_current_account_mut()?;
        let description: String = "".to_string();
        account.checkpoint(date, description, paths)?;
        FileManagement::save_current_state(&codexi, paths)?;
        let old_file = paths.archives_dir.join(arch);
        fs::remove_file(&old_file)?;
    }

    let mut codexi = FileManagement::load_current_state(paths)?;
    let account = codexi.get_current_account_mut()?;
    let new_bal = balance_v3(&account.operations);
    if old_bal.credit != new_bal.credit
        || old_bal.debit != new_bal.debit
        || old_bal.total != new_bal.total
    {
        return Err(MigrationV2Error::InvalidData(format!(
            "Calculated New: debit:{},credit:{}, balance:{},
               expected: debit:{},credit:{},balance:{}",
            new_bal.debit,
            new_bal.credit,
            new_bal.total,
            old_bal.debit,
            old_bal.credit,
            old_bal.total,
        )));
    }

    Ok(warnings)
}

/// Migrate structure from V2 to V3
fn migrate_v2_to_v3(
    mut old: CodexiV2,
    cur_bal: &Balance,
) -> Result<(Codexi, Vec<CoreWarning>), MigrationV2Error> {
    old.operations.sort_by_key(|o| (o.date, o.id));

    // remove the close operation and validate the balance.
    let op_wo_close_op: Vec<OperationV2> = old
        .operations
        .into_iter()
        .filter(|o| !matches!(o.kind, OperationKindV2::System(SystemKindV2::Close)))
        .collect();

    old.operations = op_wo_close_op; // op without close operation
    old.operations.sort_by_key(|o| (o.date, o.id));

    let bal = balance_v2(&old.operations);
    if cur_bal.total != bal.total {
        return Err(MigrationV2Error::InvalidData(format!(
            "Calculated w/o close: balance:{}, expected: balance:{}",
            bal.total, cur_bal.total
        )));
    }
    // get the initial balance
    let mut init_iter = old
        .operations
        .iter()
        .filter(|o| matches!(o.kind, OperationKindV2::System(SystemKindV2::Init)));

    let init_op = init_iter
        .next()
        .ok_or_else(|| MigrationV2Error::InvalidData("Missing initial operation".into()))?;

    if init_iter.next().is_some() {
        return Err(MigrationV2Error::InvalidData(
            "More than one initial operation".into(),
        ));
    }
    let initial_balance = init_op.amount * init_op.flow.to_sign();
    let open_date = init_op.date;

    let new_ops: Vec<Operation> = migrate_op_to_v3(&old.operations)?;

    let new_bal = balance_v3(&new_ops);
    if bal.credit != new_bal.credit || bal.debit != new_bal.debit || bal.total != new_bal.total {
        return Err(MigrationV2Error::InvalidData(format!(
            "Calculated New: debit:{},credit:{}, balance:{},
               expected: debit:{},credit:{},balance:{}",
            new_bal.debit, new_bal.credit, new_bal.total, bal.debit, bal.credit, bal.total,
        )));
    }

    let settings = CodexiSettings::load_or_create()?;
    let mut codexi = Codexi::new(settings)?;

    let mut account = Account::new(
        open_date,
        "legacy".to_string(),
        AccountType::Current,
        Some(Nulid::nil()),
        Some(Nulid::nil()),
    )?;
    account.carry_forward_balance = initial_balance;
    account.operations = new_ops;
    account.current_balance = new_bal.total;

    // refresh the anchor
    account.refresh_anchors();
    // audit the account
    let warnings = account.audit()?;

    // add the acccount
    codexi.add_account(account);

    Ok((codexi, warnings))
}

fn migrate_op_to_v3(old_ops: &Vec<OperationV2>) -> Result<Vec<Operation>, MigrationV2Error> {
    let mut new_ops = Vec::new();
    // Mapping : Old ID (usize) -> New ID (Nulid)
    let mut id_map: HashMap<usize, Nulid> = HashMap::new();
    let mut running_balance = Decimal::ZERO;

    // --- STEP 1 : Create and mapping ---
    for old in old_ops {
        let new_id = Nulid::new()?;
        // IMPORTANT : using the old ID (the original usize) as key
        id_map.insert(old.id, new_id);

        let kind = match old.kind {
            OperationKindV2::System(SystemKindV2::Close) => {
                OperationKind::System(SystemKind::Checkpoint)
            }
            OperationKindV2::System(SystemKindV2::Init) => OperationKind::System(SystemKind::Init),
            OperationKindV2::System(SystemKindV2::Adjust) => {
                OperationKind::System(SystemKind::Adjust)
            }
            OperationKindV2::System(SystemKindV2::Void) => OperationKind::System(SystemKind::Void),
            OperationKindV2::Regular(RegularKind::Transaction) => {
                OperationKind::Regular(RegularKind::Transaction)
            }
            OperationKindV2::Regular(RegularKind::Transfer) => {
                OperationKind::Regular(RegularKind::Transfer)
            }
            OperationKindV2::Regular(RegularKind::Fee) => OperationKind::Regular(RegularKind::Fee),
            OperationKindV2::Regular(RegularKind::Refund) => {
                OperationKind::Regular(RegularKind::Refund)
            }
        };

        let new_op = Operation {
            id: new_id,
            date: old.date,
            kind,
            flow: old.flow,
            amount: old.amount,
            description: old.description.clone(),

            balance: Decimal::ZERO,

            links: OperationLinks::default(),
            context: OperationContext::default(),
            meta: OperationMeta::default(),
        };
        new_ops.push(new_op);
    }

    // --- STEP 2 : Reconciliation of the Voids ---
    // browses old and new operations in parallel
    for i in 0..old_ops.len() {
        if let Some(old_target_id) = old_ops[i].void_of
            && let Some(&new_target_nulid) = id_map.get(&old_target_id)
        {
            // ID of the Void operation
            let voiding_nulid = new_ops[i].id;

            // 1. Operation Void is pointing towards its target
            new_ops[i].links.void_of = Some(new_target_nulid);

            // 2. The target is marked as voided by the void
            if let Some(target_op) = new_ops.iter_mut().find(|o| o.id == new_target_nulid) {
                target_op.links.void_by = Some(voiding_nulid);
            }
        }
    }
    // --- Step 3 : Build the Balance ---
    for op in new_ops.iter_mut() {
        match op.flow {
            OperationFlow::Credit => running_balance += op.amount,
            OperationFlow::Debit => running_balance -= op.amount,
            OperationFlow::None => {}
        }
        op.balance = running_balance;
    }

    Ok(new_ops)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Balance {
    pub credit: Decimal,
    pub debit: Decimal,
    pub total: Decimal,
}

fn balance_v2(items: &[OperationV2]) -> Balance {
    let mut credit = Decimal::ZERO;
    let mut debit = Decimal::ZERO;

    for item in items {
        match item.flow {
            OperationFlow::Credit => credit += item.amount,
            OperationFlow::Debit => debit += item.amount,
            OperationFlow::None => {}
        }
    }

    Balance {
        credit,
        debit,
        total: credit - debit,
    }
}

fn balance_v3(items: &[Operation]) -> Balance {
    let mut credit = Decimal::ZERO;
    let mut debit = Decimal::ZERO;

    for item in items {
        match item.flow {
            OperationFlow::Credit => credit += item.amount,
            OperationFlow::Debit => debit += item.amount,
            OperationFlow::None => {}
        }
    }

    Balance {
        credit,
        debit,
        total: credit - debit,
    }
}

/// load codexi v2
fn load_format_v2(file_path: &Path) -> Result<CodexiV2, MigrationV2Error> {
    if !file_path.exists() {
        return Err(MigrationV2Error::NoFile);
    }
    let bytes = fs::read(file_path)?;
    let env: FileEnvelope = serde_cbor::from_slice(&bytes)?;

    if env.magic != CODEXI_MAGIC {
        return Err(MigrationV2Error::Storage(StorageError::InvalidMagic));
    }
    if env.version != 2 {
        return Err(MigrationV2Error::Storage(StorageError::InvalidVersion {
            found: env.version,
            expected: 2,
        }));
    }

    if checksum(&env.payload) != env.checksum {
        return Err(MigrationV2Error::Storage(StorageError::InvalidChecksum));
    }

    match env.format {
        StorageFormat::Cbor => {
            let codexi: CodexiV2 = serde_cbor::from_slice(&env.payload)?;
            Ok(codexi)
        }
        _ => Err(MigrationV2Error::Storage(
            StorageError::InvalidStorageFormat { format: env.format },
        )),
    }
}

/// Old function to get the archive list
fn list_archive_v2(paths: &DataPaths) -> Result<Vec<String>, MigrationV2Error> {
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
