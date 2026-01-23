// src/core/wallet/imex/import.rs

use anyhow::{Result, bail};
use std::collections::{HashMap, HashSet};

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::operation::Operation;
use crate::core::wallet::imex::LedgerExport;
use crate::core::wallet::imex::OperationExport;
use crate::core::wallet::imex::EXPORT_VERSION;

impl Codexi {
    /// Single entry point for importing a ledger (JSON / TOML / CSV)
    pub fn import_ledger(import: &LedgerExport) -> Result<Self> {
        Self::validate_import(import)?;
        Ok(Self::build_from_export(import))
    }

    /// IMEX validation (structural + Void consistency)
    fn validate_import(import: &LedgerExport) -> Result<()> {
        // Version
        if import.version != EXPORT_VERSION {
            bail!(
                "Unsupported import version {}, expected {}",
                import.version,
                EXPORT_VERSION
            );
        }

        // IDs uniques
        let mut seen_ids = HashSet::new();
        for op in &import.operations {
            if !seen_ids.insert(op.id) {
                bail!("Duplicate operation id {}", op.id);
            }
        }

        // Preparation : index of operations
        let ops_by_id: HashMap<usize, &OperationExport> =
            import.operations.iter().map(|op| (op.id, op)).collect();

        // void_of → existing reférence
        for op in &import.operations {
            if let Some(void_id) = op.void_of {
                if !ops_by_id.contains_key(&void_id) {
                    bail!(
                        "Operation {} references unknown void_of {}",
                        op.id,
                        void_id
                    );
                }
            }
        }

        // Void / void_of consistency
        // 1. An operation Void SHALL have a void_of
        for op in &import.operations {
            if op.kind.is_void() && op.void_of.is_none() {
                bail!(
                    "Void operation {} must reference a void_of operation",
                    op.id
                );
            }
        }

        // 2. An operation cannot be voided only once
        let mut voided_targets = HashSet::new();
        for op in &import.operations {
            if op.kind.is_void() {
                let target = op.void_of.unwrap();
                if !voided_targets.insert(target) {
                    bail!(
                        "Operation {} is voided more than once",
                        target
                    );
                }
            }
        }

        // 3. An operation Void cannot voided a Void
        for op in &import.operations {
            if let Some(void_id) = op.void_of {
                let target = ops_by_id.get(&void_id).unwrap();
                if target.kind.is_void() {
                    bail!(
                        "Operation {} attempts to void another Void operation {}",
                        op.id,
                        void_id
                    );
                }
            }
        }

        // next_op_id consistency
        let max_id = import.operations.iter().map(|op| op.id).max().unwrap_or(0);
        if import.next_op_id <= max_id {
            bail!(
                "next_op_id {} is not greater than max operation id {}",
                import.next_op_id,
                max_id
            );
        }

        Ok(())
    }

    /// internal build after validation
    fn build_from_export(import: &LedgerExport) -> Self {
        let operations: Vec<Operation> = import
            .operations
            .clone()
            .into_iter()
            .map(Self::map_operation)
            .collect();

        Codexi {
            operations,
            next_op_id: import.next_op_id,
        }
    }

    /// Mapping strict Export → Doamin (without alteration)
    fn map_operation(op: OperationExport) -> Operation {
        Operation {
            id: op.id,
            kind: op.kind,
            flow: op.flow,
            date: op.date,
            amount: op.amount,
            description: op.description,
            void_of: op.void_of,
        }
    }
}
