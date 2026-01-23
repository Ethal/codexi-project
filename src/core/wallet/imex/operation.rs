// src/core/wallet/imex/operation.rs

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

use crate::core::wallet::{
    operation_kind::OperationKind,
    operation_flow::OperationFlow,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationExport {
    pub id: usize,
    pub kind: OperationKind,
    pub flow: OperationFlow,
    pub date: NaiveDate,
    pub amount: Decimal,
    pub description: String,
    pub void_of: Option<usize>,
}
