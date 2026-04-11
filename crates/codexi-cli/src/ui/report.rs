// src/ui/report.rs

use rust_decimal::Decimal;
use thousands::Separable;

use codexi::{
    core::format_id_short,
    dto::{ExchangeRateCollection, MonthlyReport, StatsCollection},
};

use crate::ui::{
    CREDIT_STYLE, DEBIT_STYLE, NOTE_STYLE, STYLE_MUTED, STYLE_NORMAL, TITLE_STYLE, VALUE_STYLE, draw_savings_bar,
    truncate_text,
};

pub fn view_monthly_report(report: &MonthlyReport) {
    println!();
    let account_name = report
        .items
        .first()
        .map(|i| i.stats.account_name.as_str())
        .unwrap_or("unknown");
    println!(
        "{}",
        TITLE_STYLE.apply_to(format!("Monthly report Account: {} ", account_name))
    );
    println!(
        "┌───────────┬──────────────────┬──────────────────┬──────────────────┬────────────────────────────────────────────┐"
    );
    println!(
        "│{:<11}│{:>18}│{:>18}│{:>18}│{:<44}│",
        " Period", "Credit", "Debit", "Balance", " Savings"
    );
    println!(
        "├───────────┼──────────────────┼──────────────────┼──────────────────┼────────────────────────────────────────────┤"
    );

    let mut has_ignored = false;
    for item in &report.items {
        let s = &item.stats;
        let balance_txt = VALUE_STYLE.apply_to(format!("{:>18}", format!("{:.2}", s.balance).separate_with_commas()));

        let (savings_val, savings_bar) = match s.savings_rate {
            Some(v) => {
                let bar = draw_savings_bar(v, 32);
                let savings_txt = format!("{:>7.2}", v);
                let savings_styled = if v < Decimal::ZERO {
                    DEBIT_STYLE.apply_to(savings_txt)
                } else {
                    CREDIT_STYLE.apply_to(savings_txt)
                };
                (format!("{}%", savings_styled), format!("{:<31}", bar))
            }
            None => (
                format!("{}", STYLE_MUTED.apply_to("N/A")),
                format!("{:<38}", STYLE_MUTED.apply_to("(not supported by account type)")),
            ),
        };

        let period_txt = if s.ignored.is_empty() {
            item.period.to_string()
        } else {
            has_ignored = true;
            format!("{}({})", item.period, s.ignored.len())
        };
        println!(
            "│{:<11}│{:>18}│{:>18}│{}│ {} {} │",
            period_txt,
            CREDIT_STYLE.apply_to(format!("{:.2}", s.total_credit).separate_with_commas()),
            DEBIT_STYLE.apply_to(format!("{:.2}", s.total_debit).separate_with_commas()),
            balance_txt,
            savings_val,
            savings_bar,
        );
    }

    println!(
        "├───────────┼──────────────────┼──────────────────┼──────────────────┼────────────────────────────────────────────┤"
    );
    // ligne total
    let total_balance = report.total_balance;
    let total_balance_txt = VALUE_STYLE.apply_to(format!(
        "{:>18}",
        format!("{:.2}", total_balance).separate_with_commas()
    ));
    println!(
        "│{:<11}│{:>18}│{:>18}│{}│{:<44}│",
        TITLE_STYLE.apply_to("TOTAL"),
        CREDIT_STYLE.apply_to(format!("{:.2}", report.total_credit).separate_with_commas()),
        DEBIT_STYLE.apply_to(format!("{:.2}", report.total_debit).separate_with_commas()),
        total_balance_txt,
        "",
    );
    println!(
        "└───────────┴──────────────────┴──────────────────┴──────────────────┴────────────────────────────────────────────┘"
    );
    if has_ignored {
        println!();
        println!("{}", NOTE_STYLE.apply_to("Note:"));
        let ignore_txt = NOTE_STYLE.apply_to(
            "Period with (xx) indicate the number of operation(s) ignored, pair Void,Voided not in the period",
        );
        println!("{}", ignore_txt);
        println!();
    }
}

/// View Financial
pub fn view_financial(stats: &StatsCollection) {
    // Savings rate
    // Savings rate
    let savings_line = match stats.savings_rate {
        Some(v) => {
            let bar = draw_savings_bar(v, 32);
            let rate_str = format!("{:>12.2}%", v);
            let rate_styled = if v < Decimal::ZERO {
                DEBIT_STYLE.apply_to(&rate_str)
            } else {
                CREDIT_STYLE.apply_to(&rate_str)
            };
            format!(
                "│ {}   {}   {}{:<13}│",
                STYLE_NORMAL.apply_to("savings rate"),
                rate_styled,
                bar,
                "",
            )
        }
        None => {
            let na_text = "N/A  (not applicable for this account type)";
            format!(
                "│ {} {}{:<21}│",
                STYLE_NORMAL.apply_to("savings rate"),
                STYLE_MUTED.apply_to(na_text),
                "",
            )
        }
    };
    println!();
    println!(
        "{}",
        TITLE_STYLE.apply_to(format!("Stats report Account: {}", stats.account_name))
    );
    println!();

    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    let title_text = format!("{:<77}", "Financial analytics (excl. init, checkpoint, void, voided)");
    println!("│ {}│", TITLE_STYLE.apply_to(title_text));
    println!("├──────────────────────┬──────────────────┬────────────────────────────────────┤");

    // Line 1 — total credit / ops count all
    println!(
        "│ {:<20} │ {:>16} │ {} {:<17} │",
        STYLE_NORMAL.apply_to("total credit"),
        CREDIT_STYLE.apply_to(format!("{:.2}", stats.total_credit).separate_with_commas()),
        STYLE_NORMAL.apply_to("ops count (all):"),
        VALUE_STYLE.apply_to(format!("{}", stats.operation_count)),
    );

    // Line 2 — total debit / ops count real
    println!(
        "│ {:<20} │ {:>16} │ {} {:<16} │",
        STYLE_NORMAL.apply_to("total debit"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.total_debit).separate_with_commas()),
        STYLE_NORMAL.apply_to("ops count (real):"),
        VALUE_STYLE.apply_to(format!("{:<4}", stats.real_operation_count)),
    );

    // Line 3 — balance / avg/op real
    println!(
        "│ {:<20} │ {:>16} │ {} {:<19} │",
        STYLE_NORMAL.apply_to("balance"),
        VALUE_STYLE.apply_to(format!("{:.2}", stats.balance).separate_with_commas()),
        STYLE_NORMAL.apply_to("avg/op (real):"),
        VALUE_STYLE.apply_to(format!("{:.2}", stats.average_operation).separate_with_commas()),
    );

    // Line 4 — adjustments
    println!(
        "│                      │                  │ {} {:<21} │",
        STYLE_NORMAL.apply_to("adjustments:"),
        VALUE_STYLE.apply_to(format!(
            "{} ({:.1}%)",
            stats.adjustment_count, stats.adjustment_percentage
        )),
    );

    println!("├──────────────────────┴──────────────────┴────────────────────────────────────┤");

    // Savings rate
    println!("{}", savings_line);

    println!("├──────────────────────────────────────────────────────────────────────────────┤");
    println!(
        "│ {:<76} │",
        TITLE_STYLE.apply_to("behavioral insights (excl. transfer DR, adjust, void, voided)")
    );
    println!("├────────────────────────────────────────┬─────────────────────────────────────┤");

    // Daily burning rate / period length
    println!(
        "│ {:<20}{:>18} │ {:<21}{:>14} │",
        STYLE_NORMAL.apply_to("daily burning rate:"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.daily_average).separate_with_commas()),
        STYLE_NORMAL.apply_to("period length:"),
        VALUE_STYLE.apply_to(format!("{} days", stats.days_count))
    );

    // Max single expense
    println!(
        "│ {:<20}{:>18} │ {:<35} │",
        STYLE_NORMAL.apply_to("max single expense:"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.max_single_debit).separate_with_commas()),
        ""
    );

    println!("├────────────────────────────────────────┴─────────────────────────────────────┤");

    // Top 5 expenses
    println!(
        "│ {:<76} │",
        TITLE_STYLE.apply_to("top 5 expenses (excl. transfer DR, adjust, void, voided)")
    );
    println!("├────────┬──────────┬──────────────────┬───────┬───────────────────────────────┤");

    for exp in stats.top_expenses.iter() {
        let index_str = exp.op_id.to_string();
        let index_str = format!("#{:<7}", &index_str[(index_str.len() - 5)..]);
        let pct_str = format!("{:>6.1}%", exp.percentage);
        println!(
            "│{}│{}│{:>18}│{}│{}│",
            STYLE_NORMAL.apply_to(index_str),
            STYLE_NORMAL.apply_to(&exp.op_date),
            DEBIT_STYLE.apply_to(format!("{:.2}", exp.amount).separate_with_commas()),
            VALUE_STYLE.apply_to(pct_str),
            STYLE_NORMAL.apply_to(truncate_text(&exp.description, 31))
        );
    }
    println!("└────────┴──────────┴──────────────────┴───────┴───────────────────────────────┘");

    // Notes section
    let show_savings_note = stats.savings_rate.is_some();
    let show_ignored_note = !stats.ignored.is_empty();

    if show_savings_note || show_ignored_note {
        println!();
        println!("{}", NOTE_STYLE.apply_to("Notes:"));
        if show_ignored_note {
            println!(
                "{}",
                NOTE_STYLE.apply_to(format!(
                    "• {} operation(s) ignored: Void/Voided pair not fully in the period.",
                    stats.ignored.len()
                ))
            );
        }
        if show_savings_note {
            println!(
                "{}",
                NOTE_STYLE
                    .apply_to("• Savings rate excludes outgoing transfers (loans, internal moves) from spending.")
            );
            println!(
                "{}",
                NOTE_STYLE.apply_to("• A positive savings rate with a negative balance means outgoing transfers")
            );
            println!(
                "{}",
                NOTE_STYLE.apply_to("  exceeded incoming transfers — not that real expenses exceeded real income.")
            );
        }
        println!();
    }
}

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
            STYLE_MUTED.apply_to(&item.date),
            DEBIT_STYLE.apply_to(format!("{:.2}", item.amount).separate_with_commas()),
            VALUE_STYLE.apply_to(format!("{:.4}", item.rate).separate_with_commas()),
            CREDIT_STYLE.apply_to(format!("{:.2}", item.cost).separate_with_commas()),
            truncate_text(&item.description, 31),
        );
    }

    println!(
        "├───────┴──────────┴──────────────────┼──────────────────┼──────────────────┼───────────────────────────────┤"
    );
    println!(
        "│{:<37}│{:>18}│{:>18}│{:<31}│",
        STYLE_MUTED.apply_to(" avg rate"),
        VALUE_STYLE.apply_to(format!("{:.4}", data.avg_rate).separate_with_commas()),
        "",
        "",
    );
    println!(
        "│{:<37}│{:>18}│{:>18}│{:<31}│",
        STYLE_MUTED.apply_to(" best rate"),
        VALUE_STYLE.apply_to(format!("{:.4}", data.best_rate).separate_with_commas()),
        "",
        "",
    );
    println!(
        "│{:<37}│{:>18}│{:>18}│{:<31}│",
        STYLE_MUTED.apply_to(" worst rate"),
        VALUE_STYLE.apply_to(format!("{:.4}", data.worst_rate).separate_with_commas()),
        "",
        "",
    );
    println!(
        "└─────────────────────────────────────┴──────────────────┴──────────────────┴───────────────────────────────┘"
    );
    println!();
}
