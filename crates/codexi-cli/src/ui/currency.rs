// src/ui/dictionnaries.rs

use codexi::core::format_id_short;
use codexi::dto::CurrencyCollection;

use crate::ui::TITLE_STYLE;

/// view to list the currency
pub fn view_currency(datas: &CurrencyCollection) {
    let title_text =
        TITLE_STYLE.apply_to("Currencies - <id> <short id> <name> <code> <symbol> [note]");
    println!();
    println!("{}", title_text);
    if datas.items.is_empty() {
        println!(" No Currency");
    } else {
        for c in &datas.items {
            println!(
                " {} {} {} {:<3} {}",
                c.id,
                format_id_short(&c.id),
                c.code,
                c.symbol,
                c.note.clone().unwrap_or_default()
            );
        }
    }
}
