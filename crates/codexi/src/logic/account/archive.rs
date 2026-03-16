// src/logic/account/archive.rs

use chrono::NaiveDate;
use nulid::Nulid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::logic::{
    account::{Account, OperationContainer},
    operation::Operation,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointRef {
    pub checkpoint_date: NaiveDate,
    pub checkpoint_balance: Decimal,
    pub archive_file: PathBuf, // "<ID>_codexi_<YYY-MM-DD>.cld"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountArchive {
    pub account_id: Nulid, // Which Account
    pub operations: Vec<Operation>,
    pub checkpoint_date: NaiveDate,  // Close date
    pub checkpoint_balance: Decimal, // Closing Balance
}

impl AccountArchive {
    pub fn new(account: &Account, checkpoint_date: NaiveDate, checkpoint_balance: Decimal) -> Self {
        AccountArchive {
            account_id: account.id,
            operations: account.operations.clone(),
            checkpoint_date,
            checkpoint_balance,
        }
    }
    pub fn get_checkpoint(account: &Account, checkpoint_date: NaiveDate) -> Option<&CheckpointRef> {
        account
            .checkpoints
            .iter()
            .find(|cp| cp.checkpoint_date == checkpoint_date)
    }
}

impl OperationContainer for AccountArchive {
    fn operations(&self) -> &[Operation] {
        &self.operations
    }
}
