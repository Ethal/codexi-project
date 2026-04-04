// src/ui/bank.rs

use codexi::core::format_id_short;
use codexi::dto::BankCollection;

use crate::ui::TITLE_STYLE;

/// view to list of the bank
pub fn view_bank(datas: &BankCollection) {
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
                format_id_short(&b.id),
                b.name,
                b.branch.clone().unwrap_or_default(),
                b.note.clone().unwrap_or_default()
            );
        }
    }
}
