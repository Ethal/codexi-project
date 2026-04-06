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

pub use account::*;
pub use balance::*;
pub use bank::view_bank;
pub use category::view_category;
pub use counterparty::view_counterparty;
pub use currency::view_currency;
pub use display::*;
pub use loan::*;
pub use operation::*;

use console::Style;
const TITLE_STYLE: Style = Style::new().cyan().bold();
const NOTE_STYLE: Style = Style::new().blue().italic();
const LABEL_STYLE: Style = Style::new().dim();
const TERMINATED_STYLE: Style = Style::new().red().dim();
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
