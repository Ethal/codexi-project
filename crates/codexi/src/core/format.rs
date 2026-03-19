// src/core/format.rs

use chrono::{NaiveDate, NaiveDateTime};
use nulid::Nulid;

use crate::core::ID_MIN_SHORT_LEN;

pub fn format_id(id: Nulid) -> String {
    id.to_string()
}
pub fn format_id_short(id: &str) -> String {
    let len = id.len();
    let start = len.saturating_sub(ID_MIN_SHORT_LEN);
    id[start..].to_string()
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
