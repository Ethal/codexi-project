// src/helpers.rs

use anyhow::{Result, anyhow};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use log::LevelFilter;
use chrono::{Local, NaiveDate, Datelike};
use directories::{ProjectDirs, UserDirs};
use rust_decimal::Decimal;

pub fn parse_decimal(s: &str, field: &str) -> Result<Decimal> {
    Decimal::from_str(s)
        .map_err(|_| anyhow!("Invalid {} '{}'", field, s))
}

pub fn parse_optional_decimal(opt_s: &Option<String>, field: &str) -> Result<Option<Decimal>> {
    opt_s.as_deref()
        .map(|s| parse_decimal(s, field))
        .transpose()
}

pub fn init_logger(lvl: bool) {

    // Configuration of the logger
    let log_level = if lvl {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp_millis()
        .init();
}

pub fn parse_flexible_date_range(
    date_str: &str,
    is_start_date: bool,
) -> Result<NaiveDate>
{
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
                .ok_or_else(|| anyhow!("Invalid start date"))?
        } else {
            NaiveDate::from_ymd_opt(year, 12, 31)
                .ok_or_else(|| anyhow!("Invalid end date"))?
        });
    }

    Err(anyhow!(
        "Invalid date format. Expected YYYY-MM-DD, YYYY-MM, or YYYY."
    ))
}

fn month_bounds(month_str: &str) -> Result<(NaiveDate, NaiveDate)> {
    let start = NaiveDate::parse_from_str(&format!("{}-01", month_str), "%Y-%m-%d")
        .map_err(|_| anyhow!("Invalid month format: expected YYYY-MM"))?;

    let (next_year, next_month) = if start.month() == 12 {
        (start.year() + 1, 1)
    } else {
        (start.year(), start.month() + 1)
    };

    let first_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .ok_or_else(|| anyhow!("Invalid intermediate date"))?;

    let end = first_next_month
        .pred_opt()
        .ok_or_else(|| anyhow!("Error computing end-of-month"))?;

    Ok((start, end))
}

const fn project_dirs_args() -> (&'static str, &'static str, &'static str) {
    ("fr", "ethal", "codexi")
}

pub fn get_data_dir() -> Result<PathBuf> {
    let (q, o, a) = project_dirs_args();
    if let Some(proj_dirs) = ProjectDirs::from(q, o, a) {
        let data_dir = proj_dirs.data_dir().to_path_buf();
        fs::create_dir_all(&data_dir)?;
        return Ok(data_dir);
    }
    Err(anyhow::anyhow!("Could not determine data directory for codexi."))
}

/// Determines the full path to the ZIP backup file.
/// Uses `target_dir_arg` (optional string) or the default user directory.
pub fn get_final_backup_path(target_dir_arg: Option<&str>) -> Result<PathBuf> {

    let now = Local::now();
    let default_filename = format!("codexi_backup_{}.zip", now.format("%Y%m%d_%H%M%S"));

    let target_dir: PathBuf;
    let final_filename: String;

    if let Some(path_str) = target_dir_arg {
        let path = PathBuf::from(path_str);

        if path.extension().map_or(false, |ext| ext.to_ascii_lowercase() == "zip") {

            final_filename = path.file_name()
                .ok_or_else(|| anyhow!("The path specified for the backup is invalid."))?
                .to_string_lossy()
                .into_owned();

            target_dir = path.parent()
                .map(|p| {
                    if p.as_os_str().is_empty() {
                        PathBuf::from(".")
                    } else {
                        p.to_path_buf()
                    }
                })
                .unwrap_or(PathBuf::from("."));
        } else {
            target_dir = path;
            final_filename = default_filename;
        }
    } else {
        let user_dirs = UserDirs::new().ok_or_else(|| anyhow!("Unable to find user directory (UserDirs)."))?;

        target_dir = user_dirs.document_dir()
            .unwrap_or_else(|| user_dirs.home_dir())
            .to_path_buf();

        final_filename = default_filename;
    };

    fs::create_dir_all(&target_dir)?;
    let final_path = target_dir.join(final_filename);

    Ok(final_path)
}
