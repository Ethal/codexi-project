// src/ui/category.rs

use thousands::Separable;

use codexi::{
    core::format_id_short,
    dto::{CategoryCollection, CategoryStatsCollection},
};

use crate::ui::{CREDIT_STYLE, DEBIT_STYLE, STYLE_DANGER, STYLE_MUTED, TITLE_STYLE, VALUE_STYLE, truncate_text};

/// view to list the category
pub fn view_category(datas: &CategoryCollection) {
    let title_text = TITLE_STYLE.apply_to("Categories:");
    let header_text = TITLE_STYLE.apply_to(format!(
        " {:<26} {:<7} {:<17} {:<20} {}",
        "<id>", "<sh id>", "<name>", "[Parent]", "[note]"
    ));
    println!();
    println!("{}", title_text);
    println!("{}", header_text);
    for c in &datas.items {
        let id_style = match &c.terminated {
            Some(_) => STYLE_DANGER,
            None => STYLE_MUTED,
        };
        let parent_style = match &c.parent_terminated {
            Some(_) => STYLE_DANGER,
            None => STYLE_MUTED,
        };
        let id = id_style.apply_to(format!("{}", c.id));
        let id_short = id_style.apply_to(format!("#{}", format_id_short(&c.id)));
        let parent = match (&c.parent_name, &c.parent_id) {
            (Some(name), Some(pid)) => {
                let styled_pid = parent_style.apply_to(format!("({})", format_id_short(pid)));
                let name_tr = truncate_text(&name, 17);
                format!("{}{}", name_tr, styled_pid)
            }
            _ => "─(—)".to_string(),
        };
        println!(
            " {} {:<7} {:<17} {:<20} {}",
            id,
            id_short,
            truncate_text(&c.name, 17),
            parent,
            c.note.clone().unwrap_or("─".to_string()),
        );
    }
}

pub fn view_category_stats(data: &CategoryStatsCollection) {
    println!();
    println!("{}", TITLE_STYLE.apply_to("Category stats"));
    println!(
        "┌───────┬──────────────────┬─────┬──────────────────┬───────┬──────────────────┬───────┬──────────────────┬──────────┐"
    );
    println!(
        "│{:<7}│{:<18}│{:>5}│{:>18}│{:>7}│{:>18}│{:>7}│{:>18}│{:<10}│",
        "Id", "Name", "Ops", "Debit", "%", "Credit", "%", "Avg/op", "Last date"
    );
    println!(
        "├───────┼──────────────────┼─────┼──────────────────┼───────┼──────────────────┼───────┼──────────────────┼──────────┤"
    );

    for item in &data.items {
        println!(
            "│{:<7}│{:<18}│{:>5}│{:>18}│{:>7}│{:>18}│{:>7}│{:>18}│{:<10}│",
            STYLE_MUTED.apply_to(format!("#{}", format_id_short(&item.id.clone().unwrap_or_default()))),
            truncate_text(&item.name, 17),
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
        "└───────┴──────────────────┴─────┴──────────────────┴───────┴──────────────────┴───────┴──────────────────┴──────────┘"
    );
    println!();
}
