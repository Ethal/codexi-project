// src/ui/mod.rs

mod account;
mod balance;
mod bank;
mod category;
mod counterparty;
mod currency;
mod display;
mod loan;
mod operation;
mod report;

pub use account::*;
pub use balance::*;
pub use bank::view_bank;
pub use category::view_category;
pub use counterparty::view_counterparty;
pub use currency::view_currency;
pub use display::*;
pub use loan::*;
pub use operation::*;
pub use report::*;

use console::Style;
const TITLE_STYLE: Style = Style::new().cyan().bold();
const NOTE_STYLE: Style = Style::new().cyan().italic();
const STYLE_MUTED: Style = Style::new().dim();
const STYLE_NORMAL: Style = Style::new();
const STYLE_HIGHLIGHT: Style = Style::new().yellow();
const STYLE_DANGER: Style = Style::new().red();
const STYLE_CAUTION: Style = Style::new().magenta().bold();
const DEBIT_STYLE: Style = Style::new().red();
const CREDIT_STYLE: Style = Style::new().green();
const VALUE_STYLE: Style = Style::new().yellow().bold();

/// Truncate text for ui
pub(crate) fn truncate_text(desc: &str, max_width: usize) -> String {
    // If the visible length is already OK → simple formatting
    if desc.chars().count() <= max_width {
        return format!("{:<width$}", desc, width = max_width);
    }

    // Otherwise → truncate without ever breaking a UTF-8 character
    let visible = max_width.saturating_sub(3);

    let truncated: String = desc.chars().take(visible).collect();

    format!("{:<width$}", format!("{}...", truncated), width = max_width)
}

pub(crate) fn label(text: &str, width: usize) -> impl std::fmt::Display {
    STYLE_MUTED.apply_to(format!("{:<width$}", text, width = width))
}

use codexi::dto::{BankItem, CurrencyItem};
pub fn format_optional_currency_item(currency: &Option<CurrencyItem>) -> String {
    match currency {
        Some(c) => c.code.to_string(),
        None => "—".to_string(),
    }
}

pub fn format_optional_bank_item(bank: &Option<BankItem>) -> String {
    match bank {
        Some(b) => b.name.to_string(),
        None => "—".to_string(),
    }
}
