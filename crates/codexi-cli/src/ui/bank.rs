// src/ui/dictionnaries.rs

use codexi::logic::bank::BankEntry;

use crate::ui::TITLE_STYLE;

/// view to list of the bank
pub fn view_bank(datas: &BankEntry) {
    let title_text = TITLE_STYLE.apply_to("Banks - <id> <name> <branch> [note]");
    println!();
    println!("{}", title_text);
    if datas.items.is_empty() {
        println!(" No Bank");
    } else {
        for b in &datas.items {
            println!(
                " {} {} {} {}",
                b.id,
                b.name,
                b.branch.clone().unwrap_or_default(),
                b.note.clone().unwrap_or_default()
            );
        }
    }
}
