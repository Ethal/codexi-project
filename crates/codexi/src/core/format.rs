// src/core/format.rs

use chrono::{NaiveDate, NaiveDateTime};
use nulid::Nulid;

use crate::core::ID_MIN_SHORT_LEN;

pub fn yes_no(b: bool) -> String {
    if b { "YES".into() } else { "NO".into() }
}

pub fn format_text(txt: &str) -> String {
    txt.to_string()
}

pub fn format_optional_text(txt: Option<&str>) -> String {
    match txt {
        Some(v) => format_text(v),
        None => "—".into(),
    }
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

pub fn format_max_monthly_transactions(value: Option<u32>) -> String {
    match value {
        None => "unlimited".to_string(),
        Some(x) => format!("{}/month", x),
    }
}

pub fn format_optional_date(value: Option<NaiveDate>) -> String {
    match value {
        Some(d) => format_date(d),
        None => "—".to_string(),
    }
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
