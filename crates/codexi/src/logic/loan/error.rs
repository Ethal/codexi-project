// src/logic/loan/error.rs

use std::fmt;

/*------------ LOAN ERROR --------------*/

/// Impl loan error
#[derive(Debug, PartialEq)]
pub enum LoanError {
    FreeDaysPeriodBelowZero,
    RefundDateBelowStartDate,
    NegativeInterest,
    InterestCapOutOfBounds,
    DurationDaysOutOfBounds,
    CapitalOutOfBounds,
    PenalityOutOfBounds,
    CapitalBelowMinimum,
    DurationExceeded,
    PenalityExceeded,
}

/// Impl loan error
impl LoanError {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoanError::FreeDaysPeriodBelowZero => "The free day period shall be above zero",
            LoanError::RefundDateBelowStartDate => {
                "The refund date shall be equal or above the start date"
            }
            LoanError::NegativeInterest => "The interest shall be positive",
            LoanError::InterestCapOutOfBounds => "The max interest cap is out of bound (0-100%)",
            LoanError::DurationDaysOutOfBounds => "The max duration days is out of bound (>=0)",
            LoanError::CapitalOutOfBounds => "The min capital is out of bound (>=0)",
            LoanError::PenalityOutOfBounds => "The max penality is out of bound (0-100%)",
            LoanError::CapitalBelowMinimum => "The capital is below the one from policy",
            LoanError::DurationExceeded => "The duration exceeded the one from policy",
            LoanError::PenalityExceeded => "The penality exceeded the one from policy",
        }
    }
}
/// Implement Display for LinearInterestError
impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
