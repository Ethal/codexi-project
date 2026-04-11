// src/exchange/models/operation.rs

use serde::{Deserialize, Serialize};

use crate::core::{
    format_date, format_decimal, format_id, format_optional_date, format_optional_id, format_optional_path, parse_date,
    parse_decimal, parse_id, parse_optional_date, parse_optional_id, parse_optional_path, resolve_or_generate_id,
};
use crate::logic::operation::{
    Operation, OperationContext, OperationError, OperationFlow, OperationKind, OperationLinks, OperationMeta,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeAccountOperations {
    pub version: u16,
    pub account_id: String,
    pub operations: Vec<ExchangeOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeOperation {
    #[serde(default)]
    pub id: Option<String>,
    pub date: String,
    pub kind: String,
    pub flow: String,
    pub amount: String,
    pub description: String,

    #[serde(default)]
    pub balance: String,

    #[serde(default)]
    pub account_id: String,

    #[serde(default)]
    pub links: ExchangeOperationLinks,

    #[serde(default)]
    pub context: ExchangeOperationContext,
    #[serde(default)]
    pub meta: ExchangeOperationMeta,
}

impl From<&Operation> for ExchangeOperation {
    fn from(op: &Operation) -> Self {
        Self {
            id: Some(format_id(op.id)),
            date: format_date(op.date),
            kind: op.kind.as_str().to_string(),
            flow: op.flow.as_str().to_string(),
            amount: format_decimal(op.amount),
            description: op.description.clone(),

            balance: format_decimal(op.balance),
            account_id: format_id(op.account_id),
            links: ExchangeOperationLinks::from(&op.links),
            context: ExchangeOperationContext::from(&op.context),
            meta: ExchangeOperationMeta::from(&op.meta),
        }
    }
}

impl TryFrom<&ExchangeOperation> for Operation {
    type Error = OperationError;
    fn try_from(op: &ExchangeOperation) -> Result<Self, Self::Error> {
        Ok(Self {
            id: resolve_or_generate_id(op.id.as_deref()),
            date: parse_date(&op.date)?,
            kind: OperationKind::try_from(op.kind.as_str())?,
            flow: OperationFlow::try_from_str(&op.flow)?,
            amount: parse_decimal(&op.amount, "amount")?,
            description: op.description.clone(),
            balance: parse_decimal(&op.balance, "balance.into")?,
            account_id: parse_id(&op.account_id)?,
            links: OperationLinks::try_from(&op.links)?,
            context: OperationContext::try_from(&op.context)?,
            meta: OperationMeta::try_from(&op.meta)?,
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeOperationLinks {
    pub void_of: Option<String>,
    pub void_by: Option<String>,
    pub transfer_id: Option<String>,
    pub transfer_account_id: Option<String>,
}

impl From<&OperationLinks> for ExchangeOperationLinks {
    fn from(ol: &OperationLinks) -> Self {
        Self {
            void_of: format_optional_id(ol.void_of),
            void_by: format_optional_id(ol.void_by),
            transfer_id: format_optional_id(ol.transfer_id),
            transfer_account_id: format_optional_id(ol.transfer_account_id),
        }
    }
}

impl TryFrom<&ExchangeOperationLinks> for OperationLinks {
    type Error = OperationError;
    fn try_from(ol: &ExchangeOperationLinks) -> Result<Self, Self::Error> {
        Ok(Self {
            void_of: parse_optional_id(ol.void_of.as_deref())?,
            void_by: parse_optional_id(ol.void_by.as_deref())?,
            transfer_id: parse_optional_id(ol.transfer_id.as_deref())?,
            transfer_account_id: parse_optional_id(ol.transfer_account_id.as_deref())?,
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeOperationContext {
    pub category_id: Option<String>,
    pub currency_id: Option<String>,
    pub exchange_rate: String,
    pub payee: Option<String>,
    pub reconciled: Option<String>,
    pub counterparty_id: Option<String>,
}

impl From<&OperationContext> for ExchangeOperationContext {
    fn from(oc: &OperationContext) -> Self {
        Self {
            category_id: format_optional_id(oc.category_id),
            currency_id: format_optional_id(oc.currency_id),
            exchange_rate: format_decimal(oc.exchange_rate),
            payee: oc.payee.clone(),
            reconciled: format_optional_date(oc.reconciled),
            counterparty_id: format_optional_id(oc.counterparty_id),
        }
    }
}

impl TryFrom<&ExchangeOperationContext> for OperationContext {
    type Error = OperationError;
    fn try_from(oc: &ExchangeOperationContext) -> Result<Self, Self::Error> {
        Ok(Self {
            category_id: parse_optional_id(oc.category_id.as_deref())?,
            currency_id: parse_optional_id(oc.currency_id.as_deref())?,
            exchange_rate: parse_decimal(&oc.exchange_rate, "exchange rate")?,
            payee: oc.payee.clone(),
            reconciled: parse_optional_date(oc.reconciled.as_deref())?,
            counterparty_id: parse_optional_id(oc.counterparty_id.as_deref())?,
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeOperationMeta {
    pub attachment_path: Option<String>,
    pub tags: Option<Vec<String>>,
    pub note: Option<String>,
}

impl From<&OperationMeta> for ExchangeOperationMeta {
    fn from(om: &OperationMeta) -> Self {
        Self {
            attachment_path: format_optional_path(om.attachment_path.as_deref()),
            tags: om.tags.clone(),
            note: om.note.clone(),
        }
    }
}

impl TryFrom<&ExchangeOperationMeta> for OperationMeta {
    type Error = OperationError;
    fn try_from(om: &ExchangeOperationMeta) -> Result<Self, Self::Error> {
        Ok(Self {
            attachment_path: parse_optional_path(om.attachment_path.as_deref()),
            tags: om.tags.clone(),
            note: om.note.clone(),
        })
    }
}
