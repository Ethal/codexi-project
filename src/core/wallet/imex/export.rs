// src/core/wallest/imex/export.rs

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::operation::Operation;
use crate::core::wallet::imex::OperationExport;
use crate::core::wallet::imex::LedgerExport;
use crate::core::wallet::imex::EXPORT_VERSION;

impl Codexi {
    /// Single entry point for exporting a ledger (JSON / TOML / CSV)
    pub fn export_ledger(&self) -> LedgerExport {
        LedgerExport {
            version: EXPORT_VERSION,
            operations: self
                .operations
                .iter()
                .map(Self::from_operation)
                .collect(),
            next_op_id: self.next_op_id,
        }
    }

    ///Explicit domain mapping -> export
    fn from_operation(op: &Operation) -> OperationExport {
        OperationExport {
            id: op.id,
            kind: op.kind,
            flow: op.flow,
            date: op.date,
            amount: op.amount,
            description: op.description.clone(),
            void_of: op.void_of,
        }
    }
}
