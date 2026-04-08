// src/ui/report.rs

use rust_decimal::Decimal;
use thousands::Separable;

use codexi::dto::{MonthlyReport, StatsCollection};

use crate::ui::{
    CREDIT_STYLE, DEBIT_STYLE, NOTE_STYLE, STYLE_NORMAL, TITLE_STYLE, VALUE_STYLE, draw_savings_bar, truncate_text,
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
        let bar = draw_savings_bar(s.savings_rate, 32);
        let savings_txt = format!("{:>7.2}%", s.savings_rate);
        let savins_styled = if s.savings_rate < Decimal::ZERO {
            DEBIT_STYLE.apply_to(savings_txt)
        } else {
            CREDIT_STYLE.apply_to(savings_txt)
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
            savins_styled,
            bar,
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
    let savings_style = if stats.savings_rate < Decimal::ZERO {
        DEBIT_STYLE
    } else {
        CREDIT_STYLE
    };
    println!();
    println!(
        "{}",
        TITLE_STYLE.apply_to(format!("Stats report Account: {}", stats.account_name))
    );
    println!();

    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    let title_text = format!("{:<77}", "Financial analytics (excl. init and checkpoint)");
    println!("│ {}│", TITLE_STYLE.apply_to(title_text));
    println!("├──────────────────────┬──────────────────┬────────────────────────────────────┤");

    // Line 1 related to total_credit/op count
    let ops_count_val = format!("{}", stats.operation_count);
    println!(
        "│ {:<20} │ {:>16} │ {} {:<23} │",
        STYLE_NORMAL.apply_to("total credit"),
        CREDIT_STYLE.apply_to(format!("{:.2}", stats.total_credit).separate_with_commas()),
        STYLE_NORMAL.apply_to("ops count:"),
        VALUE_STYLE.apply_to(ops_count_val),
    );

    // Line 2 related to total_debit/ avg/op
    let avg_op_val = format!("{:.2}", stats.average_operation);
    println!(
        "│ {:<20} │ {:>16} │ {} {:<26} │",
        STYLE_NORMAL.apply_to("total debit"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.total_debit).separate_with_commas()),
        STYLE_NORMAL.apply_to("avg/op:"),
        VALUE_STYLE.apply_to(avg_op_val)
    );

    // Line 3 related to balance
    println!(
        "│ {:<20} │ {:>16} │ {:<26} │",
        STYLE_NORMAL.apply_to("balance"),
        VALUE_STYLE.apply_to(format!("{:.2}", stats.balance).separate_with_commas()),
        " ".repeat(34)
    );

    println!("├──────────────────────┴──────────────────┴────────────────────────────────────┤");

    let label = STYLE_NORMAL.apply_to("savings rate");
    let rate_val = format!("{:>12.2}%", stats.savings_rate);
    let bar = draw_savings_bar(stats.savings_rate, 32);
    println!(
        "│ {}   {}   {} {:<11} │",
        label,
        savings_style.apply_to(rate_val),
        bar,
        ""
    );

    println!("├──────────────────────────────────────────────────────────────────────────────┤");
    println!(
        "│ {:<76} │",
        TITLE_STYLE.apply_to("behavioral insights & system health (excl. void, voided)")
    );
    println!("├────────────────────────────────────────┬─────────────────────────────────────┤");

    // Spending Rate and Duration
    println!(
        "│ {:<20}{:>18} │ {:<21}{:>14} │",
        STYLE_NORMAL.apply_to("daily burning rate:"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.daily_average).separate_with_commas()),
        STYLE_NORMAL.apply_to("period length:"),
        VALUE_STYLE.apply_to(format!("{} days", stats.days_count))
    );

    // Largest expense and account quality (adjustments)
    println!(
        "│ {:<20}{:>18} │ {:<21}{:>14} │",
        STYLE_NORMAL.apply_to("max single expense:"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.max_single_debit).separate_with_commas()),
        STYLE_NORMAL.apply_to("adjustments:"),
        VALUE_STYLE.apply_to(format!(
            "{} ({:.1}%)",
            stats.adjustment_count, stats.adjustment_percentage
        ))
    );

    println!("├────────────────────────────────────────┴─────────────────────────────────────┤");

    // Section Top Expenses
    println!(
        "│ {:<76} │",
        TITLE_STYLE.apply_to("top 5 expenses (excl. adjust, voided, void)")
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

    if !stats.ignored.is_empty() {
        println!();
        println!("{}", NOTE_STYLE.apply_to("Note:"));
        let ignore_txt = NOTE_STYLE.apply_to(format!(
            "{} operation(s) ignored, pair Void,Voided not in the period",
            stats.ignored.len()
        ));
        println!("{}", ignore_txt);
        println!();
    }
}
