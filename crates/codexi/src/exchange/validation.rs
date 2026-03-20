// src/exchange/validator.rs

use nulid::Nulid;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;
use crate::core::{CoreWarning, CoreWarningKind};
use crate::exchange::{ExchangeData, ExchangeError, ExchangeOperation};
use crate::logic::operation::{OperationKind, RegularKind};

/// Exchange data validation (structural + Void + Transfer + Amount consistency)
pub fn validate_import(import: &ExchangeData) -> Result<Vec<CoreWarning>, ExchangeError> {
    // Version
    if import.version != CODEXI_EXCHANGE_FORMAT_VERSION {
        return Err(ExchangeError::InvalidVersion(format!(
            "Unsupported import version {}, expected {}",
            import.version, CODEXI_EXCHANGE_FORMAT_VERSION
        )));
    }

    let mut warnings = Vec::new();

    // Unique IDs
    let mut seen_ids = HashSet::new();
    for op in &import.operations {
        if !seen_ids.insert(op.id) {
            return Err(ExchangeError::DuplicateOperation(format!(
                "Duplicate operation id {}",
                op.id
            )));
        }
    }

    // Preparation : index of operations
    let ops_by_id: HashMap<Nulid, &ExchangeOperation> =
        import.operations.iter().map(|op| (op.id, op)).collect();

    for op in &import.operations {
        // --- Amount validation ---
        // Amount must be strictly positive — negative or zero amounts indicate corruption
        if op.amount <= Decimal::ZERO {
            return Err(ExchangeError::InvalidAmount(format!(
                "Operation {} has invalid amount {} — must be strictly positive",
                op.id, op.amount
            )));
        }

        // --- Transfer link validation ---
        let has_transfer_id = op.links.transfer_id.is_some();
        let has_transfer_acc = op.links.transfer_account_id.is_some();

        // Both transfer fields must be set together
        match (has_transfer_id, has_transfer_acc) {
            (true, false) => {
                return Err(ExchangeError::BrokenTransferLink(format!(
                    "Operation {} has transfer_id but no transfer_account_id",
                    op.id
                )));
            }
            (false, true) => {
                return Err(ExchangeError::BrokenTransferLink(format!(
                    "Operation {} has transfer_account_id but no transfer_id",
                    op.id
                )));
            }
            _ => {}
        }

        // A transfer operation must use Regular::Transfer kind
        if has_transfer_id && !matches!(op.kind, OperationKind::Regular(RegularKind::Transfer)) {
            return Err(ExchangeError::BrokenTransferLink(format!(
                "Operation {} has transfer_id but kind is not Regular::Transfer (found {})",
                op.id,
                op.kind.as_str()
            )));
        }

        // transfer_account_id points to an account outside this file — warning only
        // (normal — the twin account lives in a separate file)
        if let Some(twin_op_id) = op.links.transfer_id
            && !ops_by_id.contains_key(&twin_op_id)
        {
            warnings.push(CoreWarning {
                    kind: CoreWarningKind::TransferAccountNotFound,
                    message: format!(
                        "Operation {} references transfer_id {} — twin operation not found in this export, transfer link cannot be verified",
                        op.id, twin_op_id
                    ),
                });
        }

        // --- Void link validation ---

        // void_of → existing reference
        if let Some(void_id) = op.links.void_of
            && !ops_by_id.contains_key(&void_id)
        {
            warnings.push(CoreWarning {
                kind: CoreWarningKind::VoidOfNotFound,
                message: format!(
                    "Operation {} references void_of {} not in current file",
                    op.id, void_id
                ),
            });
        }
    }

    // --- Void consistency ---

    // 1. A Void operation must have a void_of reference
    for op in &import.operations {
        if op.kind.is_void() && op.links.void_of.is_none() {
            return Err(ExchangeError::UnknowVoidOf(format!(
                "Void operation {} must reference a void_of operation",
                op.id
            )));
        }
    }

    // 2. An operation cannot be voided more than once
    let mut voided_targets = HashSet::new();
    for op in &import.operations {
        if op.kind.is_void() {
            let target = op.links.void_of.unwrap();
            if !voided_targets.insert(target) {
                return Err(ExchangeError::MoreThanOnceVoided(format!(
                    "Operation {} is voided more than once",
                    target
                )));
            }
        }
    }

    // 3. A Void operation cannot void another Void operation
    for op in &import.operations {
        if let Some(void_id) = op.links.void_of
            && let Some(target) = ops_by_id.get(&void_id)
            && target.kind.is_void()
        {
            return Err(ExchangeError::VoidToVoid(format!(
                "Operation {} attempts to void another Void operation {}",
                op.id, void_id
            )));
        }
    }

    Ok(warnings)
}
