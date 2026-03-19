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
use crate::logic::utils::HasNulid;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OperationLinks {
    pub void_of: Option<Nulid>,
    pub void_by: Option<Nulid>,
    pub transfer_id: Option<Nulid>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OperationContext {
    pub category_id: Option<Nulid>,
    pub currency_id: Option<Nulid>,
    pub exchange_rate: Decimal,
    pub payee: Option<String>,
    pub reconciled: Option<NaiveDate>,
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
    /// Return the signed financial impact of the operation.
    /// Credit  -> +amount
    /// Debit   -> -amount
    /// Void    -> reversed sign
    pub fn signed_amount(&self) -> Decimal {
        let base = match self.flow {
            OperationFlow::Credit => self.amount,
            OperationFlow::Debit => -self.amount,
            OperationFlow::None => Decimal::ZERO,
        };

        if self.kind.is_void() { -base } else { base }
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
