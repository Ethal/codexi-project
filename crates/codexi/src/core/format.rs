// src/core/format.rs

use chrono::{NaiveDate, NaiveDateTime};
use nulid::Nulid;
use rust_decimal::Decimal;
use std::path::Path;

use crate::core::ID_MIN_SHORT_LEN;

pub fn format_optional<T, U>(opt: Option<T>, f: fn(T) -> U) -> Option<U> {
    opt.map(f)
}

pub fn default_zero() -> String {
    "0".to_owned()
}

pub fn yes_no(b: bool) -> String {
    if b { "YES".into() } else { "NO".into() }
}

pub fn format_decimal(val: Decimal) -> String {
    format!("{:.2}", val)
}
pub fn format_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
pub fn format_text(txt: &str) -> String {
    txt.to_string()
}

pub fn format_optional_text(txt: Option<&str>) -> Option<String> {
    txt.map(format_text)
}

pub fn format_id(id: Nulid) -> String {
    id.to_string()
}

pub fn format_id_short(id: &str) -> String {
    let len = id.len();
    let start = len.saturating_sub(ID_MIN_SHORT_LEN);
    id[start..].to_string()
}

pub fn format_optional_id_short(id: Option<&str>) -> String {
    match id {
        Some(v) => format_id_short(v),
        None => "—".into(),
    }
}

pub fn format_optional_id(id: Option<Nulid>) -> Option<String> {
    id.map(format_id)
}

pub fn format_max_monthly_transactions(value: Option<u32>) -> String {
    match value {
        None => "unlimited".to_string(),
        Some(x) => format!("{}/month", x),
    }
}

pub fn format_optional_date(date: Option<NaiveDate>) -> Option<String> {
    date.map(format_date)
}

pub fn format_date(d: NaiveDate) -> String {
    d.format("%Y-%m-%d").to_string()
}
pub fn format_time(d: NaiveDateTime) -> String {
    d.format("%H-%M-%S").to_string()
}
pub fn format_date_time_long(d: NaiveDateTime) -> String {
    d.format("%Y-%m-%d_%H-%M-%S").to_string()
}
pub fn format_date_time_short(d: NaiveDateTime) -> String {
    d.format("%Y%m%d_%H%M%S").to_string()
}
