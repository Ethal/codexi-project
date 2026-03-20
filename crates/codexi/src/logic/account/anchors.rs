// src/logic/account/anchors.rs

use chrono::NaiveDate;
use nulid::Nulid;
use serde::{Deserialize, Serialize};

use crate::logic::operation::{Operation, OperationKind, SystemKind};

/// Represents the last known occurrence of a specific operation type.
/// Stores both the date and the operation id to allow precise ordering
/// when two operations share the same date (ordered by Nulid — chronological).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastAnchor {
    pub id: Nulid,
    pub date: NaiveDate,
}

impl LastAnchor {
    pub fn new(id: Nulid, date: NaiveDate) -> Self {
        Self { id, date }
    }

    /// Returns true if this anchor is more recent than another,
    /// comparing date first then id (Nulid is chronological).
    pub fn is_more_recent_than(&self, other: &LastAnchor) -> bool {
        self.date > other.date || (self.date == other.date && self.id > other.id)
    }
}

/// Cached anchors for the account — last known date and id for each operation type.
/// Updated incrementally via update() on each commit_operation().
/// Can be fully rebuilt from scratch via rebuild_from().
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountAnchors {
    pub last_regular: Option<LastAnchor>,
    pub last_init: Option<LastAnchor>,
    pub last_adjust: Option<LastAnchor>,
    pub last_void: Option<LastAnchor>,
    pub last_checkpoint: Option<LastAnchor>,
}

impl AccountAnchors {
    /// Returns the most recent anchor across all operation types.
    /// Used by lifecycle policy to determine if any operation exists.
    pub fn latest(&self) -> Option<NaiveDate> {
        [
            self.last_regular.as_ref().map(|a| a.date),
            self.last_init.as_ref().map(|a| a.date),
            self.last_adjust.as_ref().map(|a| a.date),
            self.last_void.as_ref().map(|a| a.date),
            self.last_checkpoint.as_ref().map(|a| a.date),
        ]
        .into_iter()
        .flatten()
        .max()
    }

    /// Returns the earliest anchor date across all operation types.
    pub fn earliest(&self) -> Option<NaiveDate> {
        [
            self.last_regular.as_ref().map(|a| a.date),
            self.last_init.as_ref().map(|a| a.date),
            self.last_adjust.as_ref().map(|a| a.date),
            self.last_void.as_ref().map(|a| a.date),
            self.last_checkpoint.as_ref().map(|a| a.date),
        ]
        .into_iter()
        .flatten()
        .min()
    }

    /// Updates the relevant anchor for the given operation.
    /// Only updates if the operation is more recent than the current anchor
    /// (date first, then id for same-day ordering).
    pub fn update(&mut self, op: &Operation) {
        let field = match op.kind {
            OperationKind::System(SystemKind::Init) => &mut self.last_init,
            OperationKind::System(SystemKind::Checkpoint) => &mut self.last_checkpoint,
            OperationKind::System(SystemKind::Adjust) => &mut self.last_adjust,
            OperationKind::System(SystemKind::Void) => &mut self.last_void,
            OperationKind::Regular(_) => &mut self.last_regular,
        };

        let candidate = LastAnchor::new(op.id, op.date);
        match field {
            None => *field = Some(candidate),
            Some(current) => {
                if candidate.is_more_recent_than(current) {
                    *field = Some(candidate);
                }
            }
        }
    }

    /// Fully rebuilds all anchors from the given operation slice.
    /// Used after deserialization or manual operation insertion.
    pub fn rebuild_from(&mut self, operations: &[Operation]) {
        let mut fresh = AccountAnchors::default();
        for op in operations {
            fresh.update(op);
        }
        *self = fresh;
    }
}
