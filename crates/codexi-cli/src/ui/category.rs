// src/ui/category.rs

use codexi::core::format_id_short;
use codexi::dto::CategoryCollection;

use crate::ui::TITLE_STYLE;

/// view to list of the category
pub fn view_category(datas: &CategoryCollection) {
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
                format_id_short(&c.id),
                c.name,
                c.note.clone().unwrap_or_default()
            );
        }
    }
}
