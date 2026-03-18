// src/core/parse.rs

use chrono::{Datelike, NaiveDate};
use nulid::Nulid;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::core::error::CoreError;

pub fn parse_text(parts: Vec<String>) -> String {
    let mut out = String::new();

    for part in parts {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(trimmed);
    }

    out
}

pub fn parse_id(id: &str) -> Result<Nulid, CoreError> {
    if id.is_empty() || id.len() != 26 {
        Ok(Nulid::nil())
    } else {
        Ok(String::from(id).parse()?)
    }
}

pub fn parse_optional_id(id: Option<&str>) -> Result<Option<Nulid>, CoreError> {
    id.map(parse_id).transpose()
}

pub fn parse_u32(s: &str, field: &str) -> Result<u32, CoreError> {
    s.parse::<u32>().map_err(|e| CoreError::Number {
        source: e,
        field: field.to_string(),
    })
}
pub fn parse_optional_u32(opt_s: &Option<String>, field: &str) -> Result<Option<u32>, CoreError> {
    opt_s.as_deref().map(|s| parse_u32(s, field)).transpose()
}

pub fn parse_date(s: &str) -> Result<NaiveDate, CoreError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| {
        CoreError::InvalidDate(format!(
            "Invalid date format: '{}', expected: YYYY-MM-DD",
            s
        ))
    })
}
pub fn parse_optional_date(opt_s: &Option<String>) -> Result<Option<NaiveDate>, CoreError> {
    opt_s.as_deref().map(parse_date).transpose()
}

pub fn parse_decimal(s: &str, field: &str) -> Result<Decimal, CoreError> {
    Decimal::from_str(s).map_err(|e| CoreError::Decimal {
        source: e,
        field: field.to_string(),
    })
}
pub fn parse_optional_decimal(
    opt_s: &Option<String>,
    field: &str,
) -> Result<Option<Decimal>, CoreError> {
    opt_s
        .as_deref()
        .map(|s| parse_decimal(s, field))
        .transpose()
}

pub fn parse_flexible_date_range(
    date_str: &str,
    is_start_date: bool,
) -> Result<NaiveDate, CoreError> {
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
    let start =
        NaiveDate::parse_from_str(&format!("{}-01", month_str), "%Y-%m-%d").map_err(|_| {
            CoreError::InvalidMonth("Invalid month format: expected YYYY-MM".to_string())
        })?;

    let (next_year, next_month) = if start.month() == 12 {
        (start.year() + 1, 1)
    } else {
        (start.year(), start.month() + 1)
    };

    let first_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).ok_or_else(|| {
        CoreError::InvalidIntermediateDate("Invalid intermediate date".to_string())
    })?;

    let end = first_next_month.pred_opt().ok_or_else(|| {
        CoreError::ErrorComputingEndOfMonth("Error computing end-of-month".to_string())
    })?;

    Ok((start, end))
}
