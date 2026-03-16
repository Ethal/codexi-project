// src/logic/account/container.rs

use crate::logic::{
    account::SearchItem,
    operation::{Operation, OperationFlow},
};
use rust_decimal::Decimal;

pub trait OperationContainer {
    fn operations(&self) -> &[Operation];

    fn get_operations_with_balance(&self) -> Vec<SearchItem> {
        let mut cur_bal = Decimal::ZERO;

        self.operations()
            .iter()
            .map(|op| {
                cur_bal = match op.flow {
                    OperationFlow::Credit => cur_bal + op.amount,
                    OperationFlow::Debit => cur_bal - op.amount,
                    OperationFlow::None => cur_bal,
                };

                SearchItem {
                    operation: op.clone(),
                    balance: cur_bal,
                }
            })
            .collect()
    }
}
