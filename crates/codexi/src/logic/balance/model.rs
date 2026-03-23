// src/logic/balance/model.rs

use rust_decimal::Decimal;

use crate::logic::{account::SearchEntry, operation::OperationFlow};

#[derive(Debug, Default, Clone)]
pub struct Balance {
    pub credit: Decimal,
    pub debit: Decimal,
}

impl Balance {
    /// Calculates the total of credits, debits,
    /// with several date filters (from/to/day/month/year).
    /// Returns a Balance struct.
    pub fn new(items: &SearchEntry) -> Self {
        let mut credit = Decimal::ZERO;
        let mut debit = Decimal::ZERO;

        for item in items.iter() {
            match item.operation.flow {
                OperationFlow::Credit => credit += item.operation.amount,
                OperationFlow::Debit => debit += item.operation.amount,
                OperationFlow::None => {}
            }
        }
        Self { credit, debit }
    }
    pub fn total(&self) -> Decimal {
        self.credit - self.debit
    }
}
