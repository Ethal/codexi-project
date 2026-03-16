// src/exchange/models/operation.rs

use chrono::NaiveDate;
use nulid::Nulid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::core::serde_nulid;
use crate::logic::operation::{
    Operation, OperationContext, OperationFlow, OperationKind, OperationLinks, OperationMeta,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeOperation {
    #[serde(with = "serde_nulid")]
    pub id: Nulid,
    pub date: NaiveDate,
    pub kind: OperationKind,
    pub flow: OperationFlow,
    pub amount: Decimal,
    pub description: String,

    pub balance: Decimal,

    #[serde(default)]
    pub links: OperationLinks,

    #[serde(default)]
    pub context: OperationContext,
    #[serde(default)]
    pub meta: OperationMeta,
}

impl From<&Operation> for ExchangeOperation {
    fn from(op: &Operation) -> Self {
        Self {
            id: op.id,
            date: op.date,
            kind: op.kind,
            flow: op.flow,
            amount: op.amount,
            description: op.description.clone(),

            balance: op.balance,
            links: op.links.clone(),
            context: op.context.clone(),
            meta: op.meta.clone(),
        }
    }
}
