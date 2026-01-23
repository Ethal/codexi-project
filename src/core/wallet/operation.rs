// src/core/wallet/operation.rs

use std::fmt;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use thousands::Separable;
use rust_decimal::Decimal;


use crate::core::wallet::operation_kind::OperationKind;
use crate::core::wallet::operation_flow::OperationFlow;

/// Error type for Operation
#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Invalid Operation Date format: {0}")]
    InvalidDate(#[from] chrono::ParseError),
}
/// Struct representing a wallet operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: usize,
    pub kind: OperationKind,
    pub flow: OperationFlow,
    pub date: NaiveDate,
    pub amount: Decimal,
    pub description: String,
    pub void_of: Option<usize>,
}
/// Methods for Operation
impl Operation {

    /// New operation
    pub fn new(
        id: usize,
        kind: OperationKind,
        flow: OperationFlow,
        dt: &str,
        amount: Decimal,
        desc: impl Into<String>,
        void_of: Option<usize>,
    ) -> Result<Self, OperationError>
    {

        let s: String = desc.into();
        let description = match s.trim() {
            "" => "no description".to_string(),
            t  => t.to_string(),
        };
        let naive_date = NaiveDate::parse_from_str(dt, "%Y-%m-%d")?;

        Ok(Self {
            id: id,
            kind: kind,
            flow: flow,
            date: naive_date,
            amount: amount,
            description:description,
            void_of: void_of,
        })
    }

    /// Check if an operation is voided
    pub fn is_voided(&self, all_ops: &[&Operation]) -> bool {
        all_ops.iter().any(|op| {
            op.kind.is_void() && op.void_of == Some(self.id)
        })
    }

    /// Date of the void Operation
    #[allow(dead_code)]
    pub fn void_date(&self, all_ops: &[Operation]) -> Option<NaiveDate> {
        all_ops.iter()
            .find(|op| op.kind.is_void() && op.void_of == Some(self.id))
            .map(|op| op.date)
    }

    /// Return the signed financial impact of the operation.
    /// Credit  -> +amount
    /// Debit   -> -amount
    /// VOID    -> reversed sign
    pub fn signed_amount(&self) -> Decimal {
        let base = match self.flow {
            OperationFlow::Credit => self.amount,
            OperationFlow::Debit  => -self.amount,
            OperationFlow::None   => Decimal::ZERO,
        };

        if self.kind.is_void() {
            -base
        } else {
            base
        }
    }
}

/// Implement Display for Operation
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {:.2} | {}",
            self.date.format("%Y-%m-%d"),
            self.kind,
            self.flow,
            self.amount.separate_with_commas(),
            self.description
        )
    }
}
