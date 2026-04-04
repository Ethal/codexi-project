// src/logic/account/container.rs

use crate::logic::{
    operation::{Operation, OperationFlow},
    search::SearchOperation,
};
use rust_decimal::Decimal;

pub trait OperationContainer {
    fn operations(&self) -> &[Operation];

    fn get_operations_with_balance(&self) -> Vec<SearchOperation> {
        let mut cur_bal = Decimal::ZERO;

        self.operations()
            .iter()
            .map(|op| {
                cur_bal = match op.flow {
                    OperationFlow::Credit => cur_bal + op.amount,
                    OperationFlow::Debit => cur_bal - op.amount,
                    OperationFlow::None => cur_bal,
                };

                SearchOperation {
                    operation: op.clone(),
                    balance: cur_bal,
                }
            })
            .collect()
    }
}
