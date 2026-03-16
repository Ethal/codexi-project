// src/logic/category/mod.rs

mod dto;
mod error;
mod list;
mod model;

pub use dto::{CategoryEntry, CategoryItem};
pub use error::CategoryError;
pub use list::CategoryList;
pub use model::Category;
