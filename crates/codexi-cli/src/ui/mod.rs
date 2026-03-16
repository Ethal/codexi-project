// src/ui/mod.rs

mod bank;
mod category;
mod currency;
mod ui;

pub use bank::view_bank;
pub use category::view_category;
pub use currency::view_currency;
pub use ui::*;

use console::Style;
const TITLE_STYLE: Style = Style::new().cyan().bold();
const NOTE_STYLE: Style = Style::new().blue().italic();
const LABEL_STYLE: Style = Style::new().dim();
const DEBIT_STYLE: Style = Style::new().red();
const CREDIT_STYLE: Style = Style::new().green();
const VALUE_STYLE: Style = Style::new().yellow().bold();
