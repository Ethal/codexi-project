// src/ui/dictionnaries.rs

use codexi::logic::bank::BankEntry;

use crate::ui::{TITLE_STYLE, format_long_id_to_short};

/// view to list of the bank
pub fn view_bank(datas: &BankEntry) {
    let title_text = TITLE_STYLE.apply_to("Banks - <id> <short id> <name> <branch> [note]");
    println!();
    println!("{}", title_text);
    if datas.items.is_empty() {
        println!(" No Bank");
    } else {
        for b in &datas.items {
            println!(
                " {} {} {} {} {}",
                b.id,
                format_long_id_to_short(&b.id),
                b.name,
                b.branch.clone().unwrap_or_default(),
                b.note.clone().unwrap_or_default()
            );
        }
    }
}
