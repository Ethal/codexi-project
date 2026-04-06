// src/logic/loan/model.rs

use chrono::{Duration, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::logic::loan::LoanError;

/*------------ HELPERS --------------*/

fn pow(val: Decimal, exp: i64) -> Decimal {
    if exp <= 0 {
        return Decimal::ONE;
    }

    let mut res = Decimal::ONE;

    for _ in 0..exp {
        res *= val;
    }
    res
}

/*----------------- LOAN POLICY -----------------*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoanPolicy {
    pub max_interest_cap: Option<Decimal>,
    pub max_duration_days: Option<Duration>,
    pub min_capital: Option<Decimal>,
    pub max_penality: Option<Decimal>,
}

impl Default for LoanPolicy {
    fn default() -> Self {
        Self {
            max_interest_cap: Some(Decimal::ONE_HUNDRED / Decimal::TWO),
            max_duration_days: Some(Duration::days(30_i64)),
            min_capital: Some(Decimal::ONE_HUNDRED),
            max_penality: None,
        }
    }
}

/*----------------- LOAN BASE -----------------*/

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct LoanBase {
    free_period: Duration,
    capital: Decimal,
    interest_rate: Decimal,
    policy: LoanPolicy,
}

impl LoanBase {
    fn new(
        free_period: Duration,
        capital: Decimal,
        interest_rate: Decimal,
        policy: LoanPolicy,
    ) -> Result<Self, LoanError> {
        if free_period.num_days() < 0 {
            return Err(LoanError::FreeDaysPeriodBelowZero);
        }
        if interest_rate < Decimal::ZERO {
            return Err(LoanError::NegativeInterest);
        }

        // validate interest cap (%)
        if let Some(cap) = policy.max_interest_cap
            && (cap < Decimal::ZERO || cap > Decimal::ONE_HUNDRED)
        {
            return Err(LoanError::InterestCapOutOfBounds);
        }

        // validate max duration days
        if let Some(days) = policy.max_duration_days
            && days < Duration::days(0_i64)
        {
            return Err(LoanError::DurationDaysOutOfBounds);
        }

        // validate min capital
        if let Some(capital) = policy.min_capital
            && capital < Decimal::ZERO
        {
            return Err(LoanError::CapitalOutOfBounds);
        }

        // validate max penality
        if let Some(penality) = policy.max_penality
            && penality < Decimal::ZERO
        {
            return Err(LoanError::PenalityOutOfBounds);
        }

        if let Some(min) = policy.min_capital
            && capital < min
        {
            return Err(LoanError::CapitalBelowMinimum);
        }

        Ok(Self {
            free_period,
            capital,
            interest_rate,
            policy,
        })
    }

    fn compute_late_days(&self, start_date: NaiveDate, refund_date: NaiveDate) -> Result<i64, LoanError> {
        if refund_date < start_date {
            return Err(LoanError::RefundDateBelowStartDate);
        }
        if let Some(max) = self.policy.max_duration_days
            && (refund_date - start_date) > max
        {
            return Err(LoanError::DurationExceeded);
        }

        let delay = (refund_date - start_date).num_days();
        Ok((delay - self.free_period.num_days()).max(0))
    }

    // max x% of the capital
    fn apply_interest_cap(&self, total_interest: Decimal) -> Decimal {
        if let Some(cap_pct) = self.policy.max_interest_cap {
            let max_interest = self.capital * (cap_pct / Decimal::ONE_HUNDRED);
            return total_interest.min(max_interest);
        }
        total_interest
    }
    // % max of the capital
    fn apply_penality(&self) -> Decimal {
        if let Some(penality_pct) = self.policy.max_penality {
            return self.capital * (penality_pct / Decimal::ONE_HUNDRED);
        }
        Decimal::ZERO
    }
}

/*------------ LOAN --------------*/

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoanKind {
    #[default]
    Linear,
    Compound,
}

impl LoanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compound => "compound",
            Self::Linear => "linear",
        }
    }
}

impl FromStr for LoanKind {
    type Err = LoanError;
    fn from_str(s: &str) -> Result<Self, LoanError> {
        match s.to_ascii_lowercase().as_str() {
            "linear" => Ok(Self::Linear),
            "compound" => Ok(Self::Compound),
            _ => Err(LoanError::UnknownInterestType),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Loan {
    Linear(LinearInterest),
    Compound(CompoundInterest),
}

#[derive(Debug, PartialEq)]
pub struct LoanSummary {
    pub final_due: Decimal,
    pub cumulative_interest: Vec<Decimal>,
    pub total_interest: Decimal,
    pub start_date: NaiveDate,
    pub first_interest_date: NaiveDate, // start_date + free_period + 1 jour
}

impl Loan {
    pub fn amount_due(&self, start_date: NaiveDate, refund_date: NaiveDate) -> Result<LoanSummary, LoanError> {
        match self {
            Loan::Linear(l) => l.amount_due(start_date, refund_date),
            Loan::Compound(c) => c.amount_due(start_date, refund_date),
        }
    }
}

/*------------ LINEAR INTEREST --------------*/

#[derive(Debug, PartialEq)]
pub struct LinearInterest {
    base: LoanBase,
}

impl LinearInterest {
    pub fn new(
        free_period: Duration,
        capital: Decimal,
        interest_rate: Decimal,
        policy: LoanPolicy,
    ) -> Result<Self, LoanError> {
        Ok(Self {
            base: LoanBase::new(free_period, capital, interest_rate, policy)?,
        })
    }
    fn amount_due(&self, start_date: NaiveDate, refund_date: NaiveDate) -> Result<LoanSummary, LoanError> {
        let late_days = self.base.compute_late_days(start_date, refund_date)?;

        if late_days == 0 {
            return Ok(LoanSummary {
                final_due: self.base.capital,
                cumulative_interest: Vec::new(),
                total_interest: Decimal::ZERO,
                start_date,
                first_interest_date: start_date + self.base.free_period + Duration::days(1),
            });
        }

        let rate = self.base.interest_rate / Decimal::ONE_HUNDRED;
        let daily_interest = self.base.capital * rate;
        let interest_day = vec![daily_interest; late_days as usize];

        let total_interest = daily_interest * Decimal::from(late_days);

        let interest_capped = self.base.apply_interest_cap(total_interest);
        let penality = self.base.apply_penality();
        let due = self.base.capital + interest_capped + penality;

        Ok(LoanSummary {
            final_due: due,
            cumulative_interest: interest_day,
            total_interest: interest_capped,
            start_date,
            first_interest_date: start_date + self.base.free_period + Duration::days(1),
        })
    }
}

/*------------ COMPOUND INTEREST --------------*/

#[derive(Debug, PartialEq)]
pub struct CompoundInterest {
    base: LoanBase,
}

/// Implement for compound interest
impl CompoundInterest {
    pub fn new(
        free_period: Duration,
        capital: Decimal,
        interest_rate: Decimal,
        policy: LoanPolicy,
    ) -> Result<Self, LoanError> {
        Ok(Self {
            base: LoanBase::new(free_period, capital, interest_rate, policy)?,
        })
    }
    // +x% on the total every day
    fn amount_due(&self, start_date: NaiveDate, refund_date: NaiveDate) -> Result<LoanSummary, LoanError> {
        let late_days = self.base.compute_late_days(start_date, refund_date)?;
        if late_days == 0 {
            return Ok(LoanSummary {
                final_due: self.base.capital,
                cumulative_interest: Vec::new(),
                total_interest: Decimal::ZERO,
                start_date,
                first_interest_date: start_date + self.base.free_period + Duration::days(1),
            });
        }

        let rate = self.base.interest_rate / Decimal::ONE_HUNDRED;
        let mut int_days = Vec::new();
        let mut total_interest = Decimal::ZERO;

        for d in 1..=late_days {
            let cum = self.interest_calculation(rate, d);
            let prev = self.interest_calculation(rate, d - 1);
            let daily = cum - prev;

            int_days.push(daily);
            total_interest += daily;
        }

        let interest_capped = self.base.apply_interest_cap(total_interest);
        let penality = self.base.apply_penality();
        let due = self.base.capital + interest_capped + penality;
        Ok(LoanSummary {
            final_due: due,
            cumulative_interest: int_days,
            total_interest: interest_capped,
            start_date,
            first_interest_date: start_date + self.base.free_period + Duration::days(1),
        })
    }
    fn interest_calculation(&self, rate: Decimal, day: i64) -> Decimal {
        self.base.capital * (pow(Decimal::ONE + rate, day) - Decimal::ONE)
    }
}

/*--------------- TEST --------------- */

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::{parse_date, parse_decimal};
    use chrono::Duration;
    use rust_decimal_macros::dec;

    /*----------------- COMMON -----------------*/
    #[test]
    fn applicatif_no_error() {
        /*-------------------------- Policies Setup --------------------------*/
        // Defaut policy:
        // max_interest_cap: 50% of the capital,
        // max_duration_days: 30 days,
        // min_capital: 100,
        // max_penality: None,

        let policy_standard = LoanPolicy::default();
        let mut policy_risky = LoanPolicy::default();
        policy_risky.min_capital = Some(dec!(500));
        policy_risky.max_duration_days = Some(Duration::days(60));
        policy_risky.max_interest_cap = Some(dec!(70));
        policy_risky.max_penality = Some(dec!(10));

        /*-------------------------- Loan --------------------------*/
        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-10").unwrap();
        let loan_to_bob = LinearInterest::new(Duration::days(7), dec!(100), dec!(1), policy_standard).unwrap();
        let due = loan_to_bob.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(102));
        assert_eq!(due.cumulative_interest, [dec!(1), dec!(1)]);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-10").unwrap();
        let loan_to_mike = LinearInterest::new(Duration::days(7), dec!(600), dec!(4), policy_risky).unwrap();

        // capital: 600  interest: 24+24, max interest 420=>No interest cap, penality: 60
        // expexted 600 + 48 => 648+60 => 708
        let due = loan_to_mike.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(708));
        assert_eq!(due.cumulative_interest, [dec!(24), dec!(24)])
    }

    #[test]
    fn interest_cap_out_of_bounds() {
        let mut policy_linear = LoanPolicy::default();
        policy_linear.max_interest_cap = Some(dec!(101));
        let linear = LinearInterest::new(Duration::days(7), dec!(100), dec!(1), policy_linear);
        let mut policy_compound = LoanPolicy::default();
        policy_compound.max_interest_cap = Some(dec!(-1));
        let compound = CompoundInterest::new(Duration::days(7), dec!(100), dec!(1), policy_compound);
        assert_eq!(linear.unwrap_err(), LoanError::InterestCapOutOfBounds);
        assert_eq!(compound.unwrap_err(), LoanError::InterestCapOutOfBounds);
    }
    #[test]
    fn invalid_free_period() {
        let policy = LoanPolicy::default();

        let linear = LinearInterest::new(Duration::days(-1), dec!(100), dec!(1), policy.clone());
        let compound = CompoundInterest::new(Duration::days(-1), dec!(100), dec!(-1), policy.clone());
        assert_eq!(linear.unwrap_err(), LoanError::FreeDaysPeriodBelowZero);
        assert_eq!(compound.unwrap_err(), LoanError::FreeDaysPeriodBelowZero);
    }

    /*----------------- LINEAR -----------------*/
    #[test]
    fn linear_late_free_period_zero_no_refund_limit() {
        let policy = LoanPolicy::default();
        let linear = LinearInterest::new(
            Duration::days(0),
            parse_decimal("100", "capital").unwrap(),
            dec!(2),
            policy,
        )
        .unwrap();

        let loan = Loan::Linear(linear);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-05").unwrap();
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(108));
        assert_eq!(due.cumulative_interest, [dec!(2), dec!(2), dec!(2), dec!(2)]);
    }
    /*----------------- LINEAR -----------------*/
    #[test]
    fn linear_late_free_period_one_no_refund_limit() {
        let policy = LoanPolicy::default();
        let linear = LinearInterest::new(
            Duration::days(1),
            parse_decimal("100", "capital").unwrap(),
            dec!(1),
            policy,
        )
        .unwrap();

        let loan = Loan::Linear(linear);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-05").unwrap();
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(103));
        assert_eq!(due.cumulative_interest, [dec!(1), dec!(1), dec!(1)]);
    }
    #[test]
    fn linear_ontime_no_refund_limit() {
        let policy = LoanPolicy::default();
        let linear = LinearInterest::new(
            Duration::days(7),
            parse_decimal("1_000_000", "capital").unwrap(),
            dec!(1),
            policy,
        )
        .unwrap();

        let loan = Loan::Linear(linear);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-05").unwrap();
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(1_000_000));
        assert_eq!(due.cumulative_interest, []);
    }

    #[test]
    fn linear_late_no_refund_limit() {
        let policy = LoanPolicy::default();
        let linear = LinearInterest::new(
            Duration::days(7),
            parse_decimal("1_000_000", "capital").unwrap(),
            dec!(1),
            policy,
        )
        .unwrap();
        let loan = Loan::Linear(linear);

        let start = parse_date("2026-03-17").unwrap();
        let refund = parse_date("2026-03-29").unwrap();
        let due = loan.amount_due(start, refund).unwrap();

        assert_eq!(due.final_due, dec!(1_050_000));
        assert_eq!(
            due.cumulative_interest,
            [dec!(10_000), dec!(10_000), dec!(10_000), dec!(10_000), dec!(10_000)]
        );
    }

    #[test]
    fn linear_on_time_exceed_refund_limit() {
        let mut policy = LoanPolicy::default();
        policy.max_interest_cap = Some(dec!(20));
        let linear_loan = LinearInterest::new(
            Duration::days(0),
            parse_decimal("100", "capital").unwrap(),
            dec!(20),
            policy,
        )
        .unwrap();
        let loan = Loan::Linear(linear_loan);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-05").unwrap();
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(120));
    }

    #[test]
    fn linear_late_exceed_refund_limit() {
        let mut policy = LoanPolicy::default();
        policy.max_interest_cap = Some(dec!(20));
        let linear_loan = LinearInterest::new(
            Duration::days(0),
            parse_decimal("100", "capital").unwrap(),
            dec!(20),
            policy,
        )
        .unwrap();
        let loan = Loan::Linear(linear_loan);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-10").unwrap();
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(120));
    }

    /*----------------- COMPOUND -----------------*/
    #[test]
    fn compound_late_no_refund_limit() {
        let policy = LoanPolicy::default();
        let compound = CompoundInterest::new(
            Duration::days(7),
            parse_decimal("100", "capital").unwrap(),
            dec!(1),
            policy,
        )
        .unwrap();
        let loan = Loan::Compound(compound);

        let start = parse_date("2026-03-01").unwrap();
        let refund = parse_date("2026-03-11").unwrap();
        let due = loan.amount_due(start, refund).unwrap();

        assert_eq!(due.final_due, dec!(103.0301));
        assert_eq!(due.cumulative_interest, [dec!(1), dec!(1.01), dec!(1.0201)]);
    }

    #[test]
    fn compound_late_exceed_refund_limit() {
        let mut policy = LoanPolicy::default();
        policy.max_interest_cap = Some(dec!(20));
        let compound_loan = CompoundInterest::new(
            Duration::days(0),
            parse_decimal("100", "capital").unwrap(),
            dec!(20),
            policy,
        )
        .unwrap();
        let loan = Loan::Compound(compound_loan);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-10").unwrap();
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.final_due, dec!(120));
    }

    #[test]
    fn linear_late_with_penalty() {
        let mut policy = LoanPolicy::default();
        policy.max_penality = Some(dec!(10)); // 10% du capital
        policy.max_interest_cap = None; // pas de cap pour isoler la pénalité

        let linear = LinearInterest::new(Duration::days(0), dec!(1_000), dec!(1), policy).unwrap();
        let loan = Loan::Linear(linear);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-03").unwrap();

        // capital=1000, rate=1%, 2 late days
        // interest = 1000 * 0.01 * 2 = 20
        // penalty  = 1000 * 0.10    = 100
        // final    = 1000 + 20 + 100 = 1120
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.total_interest, dec!(20));
        assert_eq!(due.final_due, dec!(1_120));
    }

    #[test]
    fn linear_late_with_cap_and_penalty() {
        let mut policy = LoanPolicy::default();
        policy.max_interest_cap = Some(dec!(10)); // cap intérêts à 10% du capital
        policy.max_penality = Some(dec!(5)); // pénalité 5% du capital

        let linear = LinearInterest::new(
            Duration::days(0),
            dec!(1_000),
            dec!(5), // rate élevé pour déclencher le cap
            policy,
        )
        .unwrap();
        let loan = Loan::Linear(linear);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-10").unwrap();

        // capital=1000, rate=5%, 9 late days
        // interest brut = 1000 * 0.05 * 9 = 450 → cappé à 1000 * 10% = 100
        // penalty       = 1000 * 5%        = 50
        // final         = 1000 + 100 + 50  = 1150
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.total_interest, dec!(100));
        assert_eq!(due.final_due, dec!(1_150));
    }

    #[test]
    fn compound_late_with_penalty() {
        let mut policy = LoanPolicy::default();
        policy.max_penality = Some(dec!(10));
        policy.max_interest_cap = None;

        let compound = CompoundInterest::new(Duration::days(0), dec!(1_000), dec!(1), policy).unwrap();
        let loan = Loan::Compound(compound);

        let start = parse_date("2026-01-01").unwrap();
        let refund = parse_date("2026-01-04").unwrap();

        // capital=1000, rate=1%, 3 late days composé
        // interest = 1000 * ((1.01)^3 - 1) = 1000 * 0.030301 = 30.301
        // penalty  = 1000 * 10% = 100
        // final    = 1000 + 30.301 + 100 = 1130.301
        let due = loan.amount_due(start, refund).unwrap();
        assert_eq!(due.total_interest, dec!(30.301));
        assert_eq!(due.final_due, dec!(1_130.301));
    }
}
