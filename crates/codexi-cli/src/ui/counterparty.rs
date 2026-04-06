// src/ui/counterparty.rs

use codexi::core::format_id_short;
use codexi::dto::CounterpartyCollection;

use crate::ui::TITLE_STYLE;

/// view to list of the counterparties
pub fn view_counterparty(datas: &CounterpartyCollection) {
    let title_text = TITLE_STYLE.apply_to("Counterparties - <id> <short id> <name> <kind> [terminated] [note]");
    println!();
    println!("{}", title_text);
    if datas.items.is_empty() {
        println!(" No Counterparty");
    } else {
        for cp in &datas.items {
            println!(
                " {} {} {} {} {} {}",
                cp.id,
                format_id_short(&cp.id),
                cp.name,
                cp.kind,
                cp.terminated.clone().unwrap_or_default(),
                cp.note.clone().unwrap_or_default(),
            );
        }
    }
}
