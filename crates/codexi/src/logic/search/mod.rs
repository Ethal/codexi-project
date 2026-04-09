// src/logic/search/mod.rs

mod error;
mod operation;

pub use error::SearchError;
pub use operation::{
    CategoryGroup, CategorySubGroup, CounterpartyCategoryGroup, CounterpartyGroup, NulidSearchFilter, SearchOperation,
    SearchOperationList, SearchParams, SearchParamsBuilder, search,
};
