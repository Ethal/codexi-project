// src/exchange/validator.rs

use nulid::Nulid;
use std::collections::{HashMap, HashSet};

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;
use crate::core::{CoreWarning, CoreWarningKind};
use crate::exchange::{ExchangeData, ExchangeError, ExchangeOperation};
/// Exchange data validation (structural + Void consistency)
pub fn validate_import(import: &ExchangeData) -> Result<Vec<CoreWarning>, ExchangeError> {
    // Version
    if import.version != CODEXI_EXCHANGE_FORMAT_VERSION {
        return Err(ExchangeError::InvalidVersion(format!(
            "Unsupported import version {}, expected {}",
            import.version, CODEXI_EXCHANGE_FORMAT_VERSION
        )));
    }

    let mut warnings = Vec::new();

    // unique IDs
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

    // void_of → existing reference
    for op in &import.operations {
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

    // Void / void_of consistency
    // 1. An operation Void SHALL have a void_of
    for op in &import.operations {
        if op.kind.is_void() && op.links.void_of.is_none() {
            return Err(ExchangeError::UnknowVoidOf(format!(
                "Void operation {} must reference a void_of operation",
                op.id
            )));
        }
    }

    // 2. An operation cannot be voided only once
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

    // 3. An operation Void cannot voided a Void
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
