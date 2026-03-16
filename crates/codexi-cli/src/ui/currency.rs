// src/ui/dictionnaries.rs

use codexi::logic::currency::CurrencyEntry;

use crate::ui::TITLE_STYLE;

/// view to list the snapshot file
pub fn view_currency(datas: &CurrencyEntry) {
    let title_text = TITLE_STYLE.apply_to("Currencies - <id> <name> <code> <symbol> [note]");
    println!();
    println!("{}", title_text);
    if datas.items.is_empty() {
        println!(" No Currency");
    } else {
        for c in &datas.items {
            println!(
                " {} {} {:<3} {}",
                c.id,
                c.code,
                c.symbol,
                c.note.clone().unwrap_or_default()
            );
        }
    }
}
