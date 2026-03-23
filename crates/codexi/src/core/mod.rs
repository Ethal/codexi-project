// src/core/mod.rs

mod error;
mod format;
mod fs;
mod parse;
mod paths;
pub mod serde_nulid;
mod validation;
mod warning;

pub const ID_MIN_SHORT_LEN: usize = 5;

pub use error::CoreError;
pub use format::{
    format_date, format_date_time_long, format_date_time_short, format_id, format_id_short,
    format_max_monthly_transactions, format_optional_date, format_optional_id_short,
    format_optional_text, format_text, format_time, yes_no,
};
pub use fs::{get_config_dir, get_data_dir};
pub use parse::{
    parse_date, parse_decimal, parse_flexible_date_range, parse_id, parse_optional_date,
    parse_optional_decimal, parse_optional_id, parse_optional_u32, parse_text, parse_u32,
};
pub use paths::DataPaths;
pub use validation::validate_text_rules;
pub use warning::{CoreWarning, CoreWarningKind};
