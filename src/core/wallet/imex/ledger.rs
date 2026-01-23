// src/core/wallet/imex/ledger.rs

use serde::{Serialize, Deserialize};
use crate::core::wallet::imex::OperationExport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerExport {
    pub version: u16,
    pub operations: Vec<OperationExport>,
    pub next_op_id: usize,
}
