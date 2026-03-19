// src/ui/dictionnaries.rs

use codexi::logic::category::CategoryEntry;

use crate::ui::{TITLE_STYLE, format_long_id_to_short};

/// view to list of the category
pub fn view_category(datas: &CategoryEntry) {
    let title_text = TITLE_STYLE.apply_to("Categories - <id> <short id> <name> [note]");
    println!();
    println!("{}", title_text);
    if datas.items.is_empty() {
        println!(" No Category");
    } else {
        for c in &datas.items {
            println!(
                " {} {} {:<20} {}",
                c.id,
                format_long_id_to_short(&c.id),
                c.name,
                c.note.clone().unwrap_or_default()
            );
        }
    }
}
