// src/core/parse.rs

use chrono::NaiveDate;
use nulid::Nulid;
use rust_decimal::Decimal;
use std::{path::PathBuf, str::FromStr};

use crate::core::error::CoreError;

pub fn parse_optional<T, U, E>(opt: Option<T>, f: fn(T) -> Result<U, E>) -> Result<Option<U>, E> {
    opt.map(f).transpose()
}

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

/// Resolves an optional raw id string to a Nulid.
/// - Some(str) → parse validated id (panics if malformed — must be called after validation)
/// - None → generate a fresh Nulid
pub fn resolve_or_generate_id(raw_id: Option<&str>) -> Nulid {
    match raw_id {
        Some(s) => parse_id(s).expect("id already validated"),
        None => Nulid::new().expect("Nulid generation failed"),
    }
}

pub fn parse_id(id: &str) -> Result<Nulid, CoreError> {
    Ok(String::from(id).parse()?)
}

pub fn parse_optional_id(id: Option<&str>) -> Result<Option<Nulid>, CoreError> {
    id.map(parse_id).transpose()
}

pub fn parse_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}

pub fn parse_optional_path(path: Option<&str>) -> Option<PathBuf> {
    path.map(parse_path)
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
pub fn parse_optional_date(opt_s: Option<&str>) -> Result<Option<NaiveDate>, CoreError> {
    opt_s.map(parse_date).transpose()
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
