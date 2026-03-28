// src/ui/mod.rs

mod account;
mod balance;
mod bank;
mod category;
mod currency;
mod display;
mod operation;

pub use account::*;
pub use balance::*;
pub use bank::view_bank;
pub use category::view_category;
pub use currency::view_currency;
pub use display::*;
pub use operation::*;

use console::Style;
const TITLE_STYLE: Style = Style::new().cyan().bold();
const NOTE_STYLE: Style = Style::new().blue().italic();
const LABEL_STYLE: Style = Style::new().dim();
const DEBIT_STYLE: Style = Style::new().red();
const CREDIT_STYLE: Style = Style::new().green();
const VALUE_STYLE: Style = Style::new().yellow().bold();

/// Truncate text for ui
pub fn truncate_text(desc: &str, max_width: usize) -> String {
    // If the visible length is already OK → simple formatting
    if desc.chars().count() <= max_width {
        return format!("{:<width$}", desc, width = max_width);
    }

    // Otherwise → truncate without ever breaking a UTF-8 character
    let visible = max_width.saturating_sub(3);

    let truncated: String = desc.chars().take(visible).collect();

    format!("{:<width$}", format!("{}...", truncated), width = max_width)
}
