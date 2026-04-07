// src/logic/search/mod.rs

mod error;
mod operation;

pub use error::SearchError;
pub use operation::{
    CategoryGroup, CounterpartyGroup, SearchOperation, SearchOperationList, SearchParams, SearchParamsBuilder, search,
};
