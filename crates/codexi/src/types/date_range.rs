// src/types/date_range.rs

use chrono::{Datelike, NaiveDate};

use crate::core::{CoreError, format_date};
use crate::logic::search::SearchOperationList;

pub struct DateRange {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl DateRange {
    pub fn parse(from: Option<&str>, to: Option<&str>) -> Result<Self, CoreError> {
        Ok(Self {
            from: from.map(|d| parse_flexible_date_range(d, true)).transpose()?,
            to: to.map(|d| parse_flexible_date_range(d, false)).transpose()?,
        })
    }

    pub fn compute(items: &SearchOperationList, from: Option<NaiveDate>, to: Option<NaiveDate>) -> Self {
        let mut iter = items.iter().map(|i| i.operation.date);
        let range = iter
            .next()
            .map(|first| iter.fold((first, first), |(min, max), d| (min.min(d), max.max(d))));

        let range_min = range.map(|(min, _)| min);
        let range_max = range.map(|(_, max)| max);

        Self {
            from: from.or(range_min),
            to: to.or(range_max),
        }
    }
    pub fn formatted(&self) -> (Option<String>, Option<String>) {
        (self.from.map(format_date), self.to.map(format_date))
    }

    /// Generate month periods between from and to
    pub fn month_periods(from: NaiveDate, to: NaiveDate) -> Vec<(NaiveDate, NaiveDate, String)> {
        let mut result = Vec::new();
        let mut current = NaiveDate::from_ymd_opt(from.year(), from.month(), 1).unwrap();
        loop {
            let (next_year, next_month) = if current.month() == 12 {
                (current.year() + 1, 1)
            } else {
                (current.year(), current.month() + 1)
            };
            let month_end = NaiveDate::from_ymd_opt(next_year, next_month, 1)
                .unwrap()
                .pred_opt()
                .unwrap();
            let label = format!("{}-{:02}", current.year(), current.month());
            result.push((current, month_end, label));
            if current.year() == to.year() && current.month() == to.month() {
                break;
            }
            current = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
        }
        result
    }
}

fn parse_flexible_date_range(date_str: &str, is_start_date: bool) -> Result<NaiveDate, CoreError> {
    // 1. Full format: YYYY-MM-DD
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(date);
    }

    // 2. Monthly format: YYYY-MM
    if let Ok((start, end)) = month_bounds(date_str) {
        return Ok(if is_start_date { start } else { end });
    }

    // 3. Year format: YYYY
    if let Ok(year) = date_str.parse::<i32>() {
        return Ok(if is_start_date {
            NaiveDate::from_ymd_opt(year, 1, 1)
                .ok_or_else(|| CoreError::InvalidStartDate("Invalid start date".to_string()))?
        } else {
            NaiveDate::from_ymd_opt(year, 12, 31)
                .ok_or_else(|| CoreError::InvalidEndDate("Invalid end date".to_string()))?
        });
    }

    Err(CoreError::InvalidDate(
        "Invalid date format. Expected YYYY-MM-DD, YYYY-MM, or YYYY.".to_string(),
    ))
}

fn month_bounds(month_str: &str) -> Result<(NaiveDate, NaiveDate), CoreError> {
    let start = NaiveDate::parse_from_str(&format!("{}-01", month_str), "%Y-%m-%d")
        .map_err(|_| CoreError::InvalidMonth("Invalid month format: expected YYYY-MM".to_string()))?;

    let (next_year, next_month) = if start.month() == 12 {
        (start.year() + 1, 1)
    } else {
        (start.year(), start.month() + 1)
    };

    let first_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .ok_or_else(|| CoreError::InvalidIntermediateDate("Invalid intermediate date".to_string()))?;

    let end = first_next_month
        .pred_opt()
        .ok_or_else(|| CoreError::ErrorComputingEndOfMonth("Error computing end-of-month".to_string()))?;

    Ok((start, end))
}
