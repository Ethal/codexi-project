// src/logic/operation/operation.rs

use chrono::NaiveDate;
use derive_builder::Builder;
use nulid::Nulid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use thousands::Separable;

use crate::core::{format_date, validate_text_rules};
use crate::logic::operation::OperationError;
use crate::logic::operation::OperationFlow;
use crate::logic::operation::OperationKind;
use crate::logic::operation::RegularKind;
use crate::logic::utils::HasNulid;

// IMPORTANT use for exchange , could be deleted avec addition of the field account_id in operation
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountOperations {
    pub account_id: Nulid,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OperationLinks {
    pub void_of: Option<Nulid>,
    pub void_by: Option<Nulid>,
    pub transfer_id: Option<Nulid>,
    pub transfer_account_id: Option<Nulid>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OperationContext {
    pub category_id: Option<Nulid>,
    pub currency_id: Option<Nulid>,
    pub exchange_rate: Decimal,
    #[serde(default)]
    pub payee: Option<String>,
    pub reconciled: Option<NaiveDate>,
    pub counterparty_id: Option<Nulid>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OperationMeta {
    pub attachment_path: Option<PathBuf>,
    pub tags: Option<Vec<String>>,
    pub note: Option<String>,
}

/// Struct representing an operation
#[derive(Serialize, Deserialize, Clone, Builder, Debug)]
#[builder(setter(into, strip_option), build_fn(private, name = "fallible_build"))]
pub struct Operation {
    // --- Minimun field ---
    #[builder(setter(skip), default = "Nulid::default()")]
    pub id: Nulid,
    pub date: NaiveDate,
    pub kind: OperationKind,
    pub flow: OperationFlow,
    pub amount: Decimal,
    pub description: String,
    #[builder(default = "Decimal::ZERO")]
    pub balance: Decimal,

    // --- Relations ---
    #[serde(default)]
    pub account_id: Nulid,

    // --- Optional field ---
    #[builder(default, setter(into, strip_option = false))]
    pub links: OperationLinks,
    #[builder(default, setter(into, strip_option = false))]
    pub context: OperationContext,
    #[builder(default, setter(into, strip_option = false))]
    pub meta: OperationMeta,
}

/// Methods for Operation
impl Operation {
    pub fn update_description(&mut self, description: &str) {
        self.description = description.into();
    }
    pub fn update_category(&mut self, category_id: Nulid) {
        self.context.category_id = Some(category_id);
    }
    pub fn update_counterparty(&mut self, counterparty_id: Nulid) {
        self.context.counterparty_id = Some(counterparty_id);
    }
    pub fn update_exchange_rate(&mut self, from: Decimal, to: Decimal) -> Result<(), OperationError> {
        if !matches!(self.kind, OperationKind::Regular(RegularKind::Transaction)) {
            return Err(OperationError::InvalidUpdateRateOperationType(
                self.kind.as_str().to_string(),
            ));
        }
        if from <= Decimal::ZERO || to <= Decimal::ZERO {
            return Err(OperationError::InvalidRate);
        }
        self.context.exchange_rate = to / from;
        Ok(())
    }

    pub fn update_context(&mut self, context: &OperationContext) {
        self.context = context.clone();
    }
    pub fn update_meta(&mut self, meta: &OperationMeta) {
        self.meta = meta.clone();
    }

    pub fn is_void(&self) -> bool {
        self.kind.is_void()
    }
    pub fn is_voided(&self) -> bool {
        self.links.void_by.is_some()
    }

    pub fn is_adjust(&self) -> bool {
        self.kind.is_adjust()
    }
    pub fn is_transfer(&self) -> bool {
        self.kind.is_transfer()
    }

    pub fn is_legacy_account(&self) -> bool {
        self.account_id.is_nil()
    }
}

/// Operation Builder
impl OperationBuilder {
    pub fn build(&self) -> Result<Operation, OperationError> {
        // Create the operation
        let mut op = self
            .fallible_build()
            .map_err(|e| OperationError::OperationBuilder(e.to_string()))?;

        // Create and update the Id
        op.id = Nulid::new()?;

        // Check the description
        if validate_text_rules(&op.description, 2, 200).is_err() {
            op.description = "no description".to_string();
        }

        op.context.exchange_rate = Decimal::ONE;

        // Check the amount
        if op.amount < Decimal::ZERO {
            return Err(OperationError::InvalidAmount(op.id.to_string()));
        }

        Ok(op)
    }
}

impl HasNulid for Operation {
    fn id(&self) -> Nulid {
        self.id
    }
}

/// Implement Display for Operation
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {:.2} | {}",
            format_date(self.date),
            self.kind,
            self.flow,
            self.amount.separate_with_commas(),
            self.description
        )
    }
}
