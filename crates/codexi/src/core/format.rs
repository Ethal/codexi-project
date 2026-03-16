// src/core/format.rs

use nulid::Nulid;
use chrono::{NaiveDate, NaiveDateTime};

pub fn format_id(id: Nulid) -> String {
    id.to_string()
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
