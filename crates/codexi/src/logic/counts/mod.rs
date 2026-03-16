// src/logic/counts/mod.rs

use crate::logic::{
    account::SearchEntry,
    operation::{OperationKind, SystemKind},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Counts {
    pub regular: usize,
    pub init: usize,
    pub adjust: usize,
    pub void: usize,
    pub checkpoint: usize,
}

impl Counts {
    pub fn counts(items: &SearchEntry) -> Counts {
        let counts = items.iter().fold(Counts::default(), |mut acc, item| {
            match item.operation.kind {
                OperationKind::Regular(_) => acc.regular += 1,
                OperationKind::System(SystemKind::Init) => acc.init += 1,
                OperationKind::System(SystemKind::Adjust) => acc.adjust += 1,
                OperationKind::System(SystemKind::Void) => acc.void += 1,
                OperationKind::System(SystemKind::Checkpoint) => acc.checkpoint += 1,
            }
            acc
        });

        counts
    }

    pub fn total(&self) -> usize {
        self.regular + self.init + self.adjust + self.void + self.checkpoint
    }
}
