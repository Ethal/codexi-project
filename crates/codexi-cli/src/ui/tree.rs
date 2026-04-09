// src/ui/tree.rs

use thousands::Separable;

use codexi::{core::format_id_short, dto::CounterpartyTreeCollection};

use crate::ui::{CREDIT_STYLE, DEBIT_STYLE, STYLE_MUTED, TITLE_STYLE, VALUE_STYLE, truncate_text};

pub fn view_tree(data: &CounterpartyTreeCollection) {
    println!();
    println!("{}", TITLE_STYLE.apply_to("Tree: Counterparty → Category → Operations"));
    println!();

    for cp in &data.nodes {
        // Counterparty header line
        let cp_id = cp.id.as_deref().unwrap_or("");
        let cp_id_short = if cp_id.is_empty() {
            "—".to_string()
        } else {
            format!("#{}", format_id_short(cp_id))
        };
        println!(
            "{} {} {}  DR: {}  CR: {}",
            TITLE_STYLE.apply_to("Counterparty:"),
            TITLE_STYLE.apply_to(&cp.name),
            STYLE_MUTED.apply_to(format!("({})", cp_id_short)),
            DEBIT_STYLE.apply_to(format!("{:.2}", cp.total_debit).separate_with_commas()),
            CREDIT_STYLE.apply_to(format!("{:.2}", cp.total_credit).separate_with_commas()),
        );

        let cat_count = cp.categories.len();
        for (cat_idx, cat) in cp.categories.iter().enumerate() {
            let is_last_cat = cat_idx == cat_count - 1;
            let cat_prefix = if is_last_cat { "  └─" } else { "  ├─" };
            let op_indent = if is_last_cat { "      " } else { "  │   " };

            // Category line
            println!(
                "{} {} {}  DR: {}  CR: {}",
                STYLE_MUTED.apply_to(cat_prefix),
                VALUE_STYLE.apply_to(&cat.name),
                STYLE_MUTED.apply_to(
                    cat.id
                        .as_deref()
                        .map(|id| format!("(#{})", format_id_short(id)))
                        .unwrap_or_else(|| "( — )".to_string())
                ),
                DEBIT_STYLE.apply_to(format!("{:.2}", cat.total_debit).separate_with_commas()),
                CREDIT_STYLE.apply_to(format!("{:.2}", cat.total_credit).separate_with_commas()),
            );

            // Operation lines
            let op_count = cat.operations.len();
            for (op_idx, op) in cat.operations.iter().enumerate() {
                let is_last_op = op_idx == op_count - 1;
                let op_connector = if is_last_op { "└─" } else { "├─" };
                let id_short = format!("#{}", format_id_short(&op.id));
                let flow_styled = if op.flow == "Debit" {
                    DEBIT_STYLE.apply_to(format!("{:<6}", "DR"))
                } else {
                    CREDIT_STYLE.apply_to(format!("{:<6}", "CR"))
                };
                let amount_styled = if op.flow == "Debit" {
                    DEBIT_STYLE.apply_to(format!("{:>14}", format!("{:.2}", op.amount).separate_with_commas()))
                } else {
                    CREDIT_STYLE.apply_to(format!("{:>14}", format!("{:.2}", op.amount).separate_with_commas()))
                };
                println!(
                    "{}{} {}  {}  {}  {}  {}",
                    STYLE_MUTED.apply_to(op_indent),
                    STYLE_MUTED.apply_to(op_connector),
                    STYLE_MUTED.apply_to(&id_short),
                    STYLE_MUTED.apply_to(&op.date),
                    flow_styled,
                    amount_styled,
                    truncate_text(&op.description, 35),
                );
            }
        }
        println!();
    }
}
