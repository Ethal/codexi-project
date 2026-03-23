// src/logic/account/mod.rs

mod account_type;
mod action;
mod anchors;
mod archive;
mod audit;
mod container;
mod dto;
mod entry;
mod error;
mod merge;
mod model;
mod policy;
mod reports;
mod search;

pub use account_type::AccountType;
pub use anchors::AccountAnchors;
pub use archive::{AccountArchive, CheckpointRef};
pub use container::OperationContainer;
pub use dto::{AccountAnchorsItem, OperationEntry, OperationItem, SummaryEntry};
pub use error::{AccountError, SearchError};
pub use model::{Account, AccountMeta};
pub use policy::{
    AccountContext, AccountContextItem, ComplianceAction, CompliancePolicy, ComplianceViolation,
    LifecycleViolation, TemporalAction, TemporalViolation,
};
pub use reports::StatsEntry;
pub use search::{SearchEntry, SearchItem, SearchParams, SearchParamsBuilder, search};
