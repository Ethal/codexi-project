// src/types/date_range.rs

use chrono::NaiveDate;

use crate::core::{CoreError, parse_flexible_date_range};

pub struct DateRange {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl DateRange {
    pub fn parse(from: Option<&str>, to: Option<&str>) -> Result<Self, CoreError> {
        Ok(Self {
            from: from
                .map(|d| parse_flexible_date_range(d, true))
                .transpose()?,
            to: to
                .map(|d| parse_flexible_date_range(d, false))
                .transpose()?,
        })
    }
}
