// src/exchange/validator/operation.rs

use nulid::Nulid;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;
use crate::core::{CoreWarning, CoreWarningKind, parse_decimal, parse_id, parse_optional_id};
use crate::exchange::{ExchangeAccountOperations, ExchangeError, ExchangeOperation};
use crate::logic::operation::{OperationError, OperationKind, OperationKindError, RegularKind};

/// Exchange operations validation (structural + Void + Transfer + Amount consistency)
pub fn validate_import_operations(
    import: &ExchangeAccountOperations,
) -> Result<Vec<CoreWarning>, ExchangeError> {
    // Version
    if import.version != CODEXI_EXCHANGE_FORMAT_VERSION {
        return Err(ExchangeError::InvalidVersion(format!(
            "Unsupported import version {}, expected {}",
            import.version, CODEXI_EXCHANGE_FORMAT_VERSION
        )));
    }

    // Check if the provided account id is a valid Nulid
    parse_id(&import.account_id)?;

    let mut warnings = Vec::new();
    let mut seen_ids = HashSet::new();
    let mut ops_by_id: HashMap<Nulid, &ExchangeOperation> = HashMap::new();

    // --- First pass: id uniqueness, amount, transfer links ---
    for op in &import.operations {
        // New operation (no id) — only simple kinds allowed, skip id-based validations
        if op.id.is_none() {
            if op.links.void_of.is_some()
                || op.links.void_by.is_some()
                || op.links.transfer_id.is_some()
                || op.links.transfer_account_id.is_some()
            {
                return Err(ExchangeError::InvalidData(
                    "New operation (no id) cannot have void or transfer links".into(),
                ));
            }
            // Amount still validated for new operations
            let amount = parse_decimal(&op.amount, "amount")?;
            if amount <= Decimal::ZERO {
                return Err(ExchangeError::InvalidAmount(
                    "New operation has invalid amount — must be strictly positive".into(),
                ));
            }
            continue;
        }

        // Operation with id — full validation
        let raw_id = op.id.as_ref().unwrap();
        let id = parse_id(raw_id)?;
        let id_s = id.to_string();

        if !seen_ids.insert(id) {
            return Err(ExchangeError::DuplicateOperation(format!(
                "Duplicate operation id {}",
                id_s
            )));
        }
        ops_by_id.insert(id, op);

        // Amount must be strictly positive
        let amount = parse_decimal(&op.amount, "amount")?;
        if amount <= Decimal::ZERO {
            return Err(ExchangeError::InvalidAmount(format!(
                "Operation {} has invalid amount {} — must be strictly positive",
                id_s, amount
            )));
        }

        // Both transfer fields must be set together
        let has_transfer_id = op.links.transfer_id.is_some();
        let has_transfer_acc = op.links.transfer_account_id.is_some();
        match (has_transfer_id, has_transfer_acc) {
            (true, false) => {
                return Err(ExchangeError::BrokenTransferLink(format!(
                    "Operation {} has transfer_id but no transfer_account_id",
                    id_s
                )));
            }
            (false, true) => {
                return Err(ExchangeError::BrokenTransferLink(format!(
                    "Operation {} has transfer_account_id but no transfer_id",
                    id_s
                )));
            }
            _ => {}
        }

        // A transfer operation must use Regular::Transfer kind
        let kind = parse_kind(op)?;
        if has_transfer_id && !matches!(kind, OperationKind::Regular(RegularKind::Transfer)) {
            return Err(ExchangeError::BrokenTransferLink(format!(
                "Operation {} has transfer_id but kind is not Regular::Transfer (found {})",
                id_s, kind
            )));
        }

        // Transfer twin lives in another file — warning only
        let transfer_id = parse_optional_id(op.links.transfer_id.as_deref())?;
        if let Some(twin_op_id) = transfer_id
            && !ops_by_id.contains_key(&twin_op_id)
        {
            warnings.push(CoreWarning {
                kind: CoreWarningKind::TransferAccountNotFound,
                message: format!(
                    "Operation {} references transfer_id {} — twin not found in this export",
                    id_s, twin_op_id
                ),
            });
        }
    }

    // --- Second pass: void link validation (requires ops_by_id fully built) ---
    for (id, op) in &ops_by_id {
        let void_of = parse_optional_id(op.links.void_of.as_deref())?;
        if let Some(void_id) = void_of
            && !ops_by_id.contains_key(&void_id)
        {
            warnings.push(CoreWarning {
                kind: CoreWarningKind::VoidOfNotFound,
                message: format!(
                    "Operation {} references void_of {} not in current file",
                    id, void_id
                ),
            });
        }

        // A Void operation must have a void_of reference
        let kind = parse_kind(op)?;
        if kind.is_void() && op.links.void_of.is_none() {
            return Err(ExchangeError::UnknowVoidOf(format!(
                "Void operation {} must reference a void_of operation",
                id
            )));
        }
    }

    // --- Third pass: void consistency ---
    let mut voided_targets = HashSet::new();
    for (id, op) in &ops_by_id {
        let kind = parse_kind(op)?;

        // An operation cannot be voided more than once
        if kind.is_void() {
            let target = op.links.void_of.clone().unwrap();
            if !voided_targets.insert(target.clone()) {
                return Err(ExchangeError::MoreThanOnceVoided(format!(
                    "Operation {} is voided more than once",
                    target
                )));
            }
        }

        // A Void operation cannot void another Void operation
        let void_of = parse_optional_id(op.links.void_of.as_deref())?;
        if let Some(void_id) = void_of
            && let Some(target) = ops_by_id.get(&void_id)
        {
            let target_kind = parse_kind(target)?;
            if target_kind.is_void() {
                return Err(ExchangeError::VoidToVoid(format!(
                    "Operation {} attempts to void another Void operation {}",
                    id, void_id
                )));
            }
        }
    }

    Ok(warnings)
}

/// Parse OperationKind from an ExchangeOperation — avoids repeating the error mapping.
fn parse_kind(op: &ExchangeOperation) -> Result<OperationKind, ExchangeError> {
    OperationKind::try_from(op.kind.as_str())
        .map_err(|e| OperationError::Kind(OperationKindError::Unknown(e.to_string())).into())
}
