// src/logic/loan/mod.rs

mod error;
mod model;
mod settings;

pub use error::LoanError;
pub use model::{CompoundInterest, LinearInterest, Loan, LoanBase, LoanKind, LoanPolicy, LoanSummary};
pub use settings::LoanPolicySettings;
