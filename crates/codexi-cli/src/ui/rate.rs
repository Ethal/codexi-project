// src/ui/rate.rs

use thousands::Separable;

use codexi::{core::format_id_short, dto::ExchangeRateCollection};

use crate::ui::{
    CREDIT_STYLE, DEBIT_STYLE, NOTE_STYLE, STYLE_MUTED, STYLE_NORMAL, TITLE_STYLE, VALUE_STYLE, truncate_text,
};

pub fn view_exchange_rate(data: &ExchangeRateCollection) {
    let cost_label = data.cost_currency.as_deref().unwrap_or("Cost");

    println!();
    println!(
        "{}",
        TITLE_STYLE.apply_to(format!(
            "Exchange rate report — {} ({})",
            data.account_name, data.account_currency
        ))
    );

    if data.items.is_empty() {
        println!();
        println!("{}", NOTE_STYLE.apply_to("No operations with exchange rate found."));
        println!();
        return;
    }

    println!(
        "┌───────┬──────────┬──────────────────┬──────────────────┬──────────────────┬───────────────────────────────┐"
    );
    println!(
        "│{:<7}│{:<10}│{:>18}│{:>18}│{:>18}│{:<31}│",
        "Id", "Date", data.account_currency, "Rate", cost_label, "Description"
    );
    println!(
        "├───────┼──────────┼──────────────────┼──────────────────┼──────────────────┼───────────────────────────────┤"
    );

    for item in &data.items {
        let id_short = format!("#{}", format_id_short(&item.op_id));
        println!(
            "│{:<7}│{:<10}│{:>18}│{:>18}│{:>18}│{:<31}│",
            STYLE_MUTED.apply_to(id_short),
            STYLE_NORMAL.apply_to(&item.date),
            DEBIT_STYLE.apply_to(format!("{:.2}", item.amount).separate_with_commas()),
            VALUE_STYLE.apply_to(format!("{:.8}", item.rate).separate_with_commas()),
            CREDIT_STYLE.apply_to(format!("{:.2}", item.cost).separate_with_commas()),
            truncate_text(&item.description, 31),
        );
    }

    println!(
        "├───────┴──────────┴──────────────────┼──────────────────┼──────────────────┼───────────────────────────────┤"
    );
    println!(
        "│{:<37}│{:>18}│{:>18}│{:<31}│",
        STYLE_NORMAL.apply_to(" avg rate"),
        VALUE_STYLE.apply_to(format!("{:.8}", data.avg_rate).separate_with_commas()),
        "",
        "",
    );
    println!(
        "│{:<37}│{:>18}│{:>18}│{:<31}│",
        STYLE_NORMAL.apply_to(" best rate"),
        VALUE_STYLE.apply_to(format!("{:.8}", data.best_rate).separate_with_commas()),
        "",
        "",
    );
    println!(
        "│{:<37}│{:>18}│{:>18}│{:<31}│",
        STYLE_NORMAL.apply_to(" worst rate"),
        VALUE_STYLE.apply_to(format!("{:.8}", data.worst_rate).separate_with_commas()),
        "",
        "",
    );
    println!(
        "└─────────────────────────────────────┴──────────────────┴──────────────────┴───────────────────────────────┘"
    );
    println!();
}
