// src/exchannge/models/checkpoint.rs

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::logic::account::CheckpointRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCheckpointRef {
    pub checkpoint_date: NaiveDate,
    pub checkpoint_balance: Decimal,
    pub archive_file: PathBuf, // "<ID>_codexi_<YYY-MM-DD>.cld"
}

impl From<&CheckpointRef> for ExchangeCheckpointRef {
    fn from(checkpoint: &CheckpointRef) -> Self {
        Self {
            checkpoint_date: checkpoint.checkpoint_date,
            checkpoint_balance: checkpoint.checkpoint_balance,
            archive_file: checkpoint.archive_file.clone(),
        }
    }
}
