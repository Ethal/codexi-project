// src/logic/account/anchors.rs

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::logic::operation::{Operation, OperationKind, SystemKind};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountAnchors {
    pub last_regular: Option<NaiveDate>,
    pub last_init: Option<NaiveDate>,
    pub last_adjust: Option<NaiveDate>,
    pub last_void: Option<NaiveDate>,
    pub last_checkpoint: Option<NaiveDate>,
}

impl AccountAnchors {
    pub fn latest(&self) -> Option<NaiveDate> {
        [
            self.last_regular,
            self.last_init,
            self.last_adjust,
            self.last_void,
            self.last_checkpoint,
        ]
        .into_iter()
        .flatten()
        .max()
    }
    pub fn earliest(&self) -> Option<NaiveDate> {
        [
            self.last_regular,
            self.last_init,
            self.last_adjust,
            self.last_void,
            self.last_checkpoint,
        ]
        .into_iter()
        .flatten()
        .min()
    }
    pub fn update(&mut self, op: &Operation) {
        let field = match op.kind {
            OperationKind::System(SystemKind::Init) => &mut self.last_init,
            OperationKind::System(SystemKind::Checkpoint) => &mut self.last_checkpoint,
            OperationKind::System(SystemKind::Adjust) => &mut self.last_adjust,
            OperationKind::System(SystemKind::Void) => &mut self.last_void,
            OperationKind::Regular(_) => &mut self.last_regular,
        };
        if field.is_none_or(|d| op.date > d) {
            *field = Some(op.date);
        }
    }
    pub fn rebuild_from(&mut self, operations: &[Operation]) {
        let mut fresh = AccountAnchors::default();
        for op in operations {
            fresh.update(op); // update already keeps the maximum because iterate chronologically
        }
        *self = fresh;
    }
}
