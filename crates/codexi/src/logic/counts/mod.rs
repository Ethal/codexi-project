// src/logic/counts/mod.rs

use crate::logic::{
    account::SearchEntry,
    operation::{OperationKind, SystemKind},
};

#[derive(Debug, Default, Clone)]
pub struct Counts {
    pub regular: usize,
    pub init: usize,
    pub adjust: usize,
    pub void: usize,
    pub checkpoint: usize,
}

impl Counts {
    pub fn new(items: &SearchEntry) -> Self {
        let mut counts = Counts::default();

        for item in items.iter() {
            match item.operation.kind {
                OperationKind::Regular(_) => counts.regular += 1,
                OperationKind::System(SystemKind::Init) => counts.init += 1,
                OperationKind::System(SystemKind::Adjust) => counts.adjust += 1,
                OperationKind::System(SystemKind::Void) => counts.void += 1,
                OperationKind::System(SystemKind::Checkpoint) => counts.checkpoint += 1,
            }
        }

        counts
    }

    pub fn total(&self) -> usize {
        self.regular + self.init + self.adjust + self.void + self.checkpoint
    }
}
