// src/logic/loan/error.rs

use thiserror::Error;

/*------------ LOAN ERROR --------------*/

#[derive(Debug, PartialEq, Error)]
pub enum LoanError {
    #[error("The free day period shall be above zero")]
    FreeDaysPeriodBelowZero,
    #[error("The refund date shall be equal or above the start date")]
    RefundDateBelowStartDate,
    #[error("The interest shall be positive")]
    NegativeInterest,
    #[error("The max interest cap is out of bound (0-100%)")]
    InterestCapOutOfBounds,
    #[error("The max duration days is out of bound (>=0)")]
    DurationDaysOutOfBounds,
    #[error("The min capital is out of bound (>=0)")]
    CapitalOutOfBounds,
    #[error("The max penality is out of bound (0-100%)")]
    PenalityOutOfBounds,
    #[error("The capital is below the one from policy")]
    CapitalBelowMinimum,
    #[error("The duration exceeded the one from policy")]
    DurationExceeded,
    #[error("The penality exceeded the one from policy")]
    PenalityExceeded,
    #[error("Unknown interest type, expected 'linear' or 'compound'")]
    UnknownInterestType,
    #[error("Failed to serialize loan policy")]
    PolicySerialize,
    #[error("Failed to write loan policy to disk")]
    PolicyWrite,
    #[error("Failed to parse policy field: {0}")]
    PolicyParse(String),
}
