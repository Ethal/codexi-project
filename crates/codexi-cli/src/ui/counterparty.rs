// src/ui/counterparty.rs

use thousands::Separable;

use codexi::{
    core::format_id_short,
    dto::{CounterpartyCollection, CounterpartyStatsCollection},
};

use crate::ui::{CREDIT_STYLE, DEBIT_STYLE, STYLE_DANGER, STYLE_MUTED, TITLE_STYLE, VALUE_STYLE, truncate_text};

/// view to list of the counterparties
pub fn view_counterparty(datas: &CounterpartyCollection) {
    let title_text = TITLE_STYLE.apply_to("Counterparties:");
    let header_text = TITLE_STYLE.apply_to(format!(
        " {:<26} {:<7} {:<20} {:<15} {}",
        "<id>", "<sh id>", "<name>", "<kind>", "[note]"
    ));
    println!();
    println!("{}", title_text);
    println!("{}", header_text);
    for cp in &datas.items {
        let id_style = match &cp.terminated {
            Some(_) => STYLE_DANGER,
            None => STYLE_MUTED,
        };
        let id = id_style.apply_to(format!("{}", cp.id));
        let id_short = id_style.apply_to(format!("#{}", format_id_short(&cp.id)));
        println!(
            " {} {:<7} {:<20} {:<15} {}",
            id,
            id_short,
            cp.name,
            cp.kind,
            cp.note.clone().unwrap_or_default(),
        );
    }
}

pub fn view_counterparty_stats(data: &CounterpartyStatsCollection) {
    println!();
    println!("{}", TITLE_STYLE.apply_to("Counterparty stats"));
    println!(
        "┌───────┬──────────────────┬──────────────┬─────┬──────────────────┬───────┬──────────────────┬───────┬──────────────────┬──────────┐"
    );
    println!(
        "│{:<7}│{:<18}│{:<14}│{:>5}│{:>18}│{:>7}│{:>18}│{:>7}│{:>18}│{:<10}│",
        "Id", "Name", "Kind", "Ops", "Debit", "%", "Credit", "%", "Avg/op", "Last date"
    );
    println!(
        "├───────┼──────────────────┼──────────────┼─────┼──────────────────┼───────┼──────────────────┼───────┼──────────────────┼──────────┤"
    );

    for item in &data.items {
        println!(
            "│{:<7}│{:<18}│{:<14}│{:>5}│{:>18}│{:>7}│{:>18}│{:>7}│{:>18}│{:<10}│",
            STYLE_MUTED.apply_to(format!("#{}", format_id_short(&item.id.clone().unwrap_or_default()))),
            truncate_text(&item.name, 17),
            item.kind,
            VALUE_STYLE.apply_to(item.op_count),
            DEBIT_STYLE.apply_to(format!("{:.2}", item.total_debit).separate_with_commas()),
            VALUE_STYLE.apply_to(format!("{:.2}%", item.debit_percentage)),
            CREDIT_STYLE.apply_to(format!("{:.2}", item.total_credit).separate_with_commas()),
            VALUE_STYLE.apply_to(format!("{:.2}%", item.credit_percentage)),
            VALUE_STYLE.apply_to(format!("{:.2}", item.average_amount).separate_with_commas()),
            STYLE_MUTED.apply_to(item.last_date.as_deref().unwrap_or("—")),
        );
    }

    println!(
        "└───────┴──────────────────┴──────────────┴─────┴──────────────────┴───────┴──────────────────┴───────┴──────────────────┴──────────┘"
    );
    println!();
}
