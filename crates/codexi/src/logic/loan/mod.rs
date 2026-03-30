// src/logic/loan/mod.rs

mod error;
mod model;

pub use error::LoanError;
pub use model::{CompoundInterest, LinearInterest, Loan};
