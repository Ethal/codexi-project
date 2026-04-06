// src/ui.rs
use console::{Style, style};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use thousands::Separable;

use codexi::{
    core::{CoreWarning, format_id_short},
    dto::{SearchOperationCollection, StatsCollection, SummaryCollection},
    file_management::CodexiInfos,
};

use crate::ui::{
    CREDIT_STYLE, DEBIT_STYLE, NOTE_STYLE, STYLE_DANGER, STYLE_MUTED, STYLE_NORMAL, TITLE_STYLE, VALUE_STYLE, label,
    truncate_text,
};

pub fn view_warning(warnings: &[CoreWarning]) {
    let title_text = TITLE_STYLE.apply_to("Warning(s)");
    println!("{}", title_text);
    for warn in warnings {
        println!(" {}", warn);
    }
}

/// view of the codexi infos.
pub fn view_codexi_infos(datas: &CodexiInfos) {
    println!();
    println!("📒 {}", TITLE_STYLE.apply_to("Infos"));
    println!("{}", "─".repeat(55));
    println!("  {} {}", label("Data version", 18), datas.data_version);
    println!("  {} {}", label("Exchange version", 18), datas.exchange_version);
    println!("  {} {}", label("Storage format", 18), datas.storage_format);

    println!();
    println!("💰 {}", TITLE_STYLE.apply_to("Codexi"));
    println!("{}", "─".repeat(55));
    println!("  {} {}", label("Accounts", 27), datas.codexi_account_count);
    println!(
        "  {} {}",
        label("Operations(incl. archives)", 27),
        datas.codexi_operation_count
    );
    println!("  {} {}", label("Banks", 27), datas.codexi_bank_count);
    println!("  {} {}", label("Currencies", 27), datas.codexi_currency_count);
    println!("  {} {}", label("Categories", 27), datas.codexi_category_count);
    println!("  {} {}", label("Counterparty", 27), datas.codexi_counterparty_count);
    println!();
    let usage = &datas.disk_usage;
    println!("📦 {}", TITLE_STYLE.apply_to("Disk usage"));
    println!("{}", "─".repeat(55));
    println!("  data_dir/");
    println!(
        "    {:<18} {:<10}",
        STYLE_MUTED.apply_to("codexi.dat"),
        VALUE_STYLE.apply_to(format_bytes(usage.data_dir.codexi.size_bytes))
    );
    println!(
        "    {:<18} {:<10} {} files",
        STYLE_MUTED.apply_to("snapshots/"),
        VALUE_STYLE.apply_to(format_bytes(usage.data_dir.snapshots.total_bytes)),
        usage.data_dir.snapshots.file_count
    );
    println!(
        "    {:<18} {:<10} {} account, {} files",
        STYLE_MUTED.apply_to("archives/"),
        VALUE_STYLE.apply_to(format_bytes(usage.data_dir.archives.total_bytes)),
        usage.data_dir.archives.account_count,
        usage.data_dir.archives.file_count
    );
    println!("  {}", "─".repeat(30));
    println!(
        "  {:<20} {:<10}",
        STYLE_MUTED.apply_to("total data_dir"),
        VALUE_STYLE.apply_to(format_bytes(usage.data_dir.total_bytes))
    );

    println!();
    println!(
        "  {:<20} {:<10} {} restore points",
        STYLE_MUTED.apply_to("trash/"),
        VALUE_STYLE.apply_to(format_bytes(usage.trash.total_bytes)),
        usage.trash.restore_point_count
    );

    println!();
    println!("{}", "─".repeat(55));
    println!(
        "  {:<20} {}",
        TITLE_STYLE.apply_to("TOTAL"),
        VALUE_STYLE.apply_to(format_bytes(usage.total_bytes))
    );
    println!();
}

/// view to list the snapshot file
pub fn view_snapshot(datas: &[String]) {
    let title_text = TITLE_STYLE.apply_to("Snapshot(s)");
    println!();
    println!("{}", title_text);
    if datas.is_empty() {
        println!(" No snapshot");
    } else {
        for f in datas {
            println!(" {}", f);
        }
    }
    if datas.len() > 5 {
        println!();
        let note_text =
            NOTE_STYLE.apply_to("Note: Considered to performe a `data clean`, snapshot file grether than 5.");
        println!("{}", note_text);
        println!();
    }
}
/// view to list the archive file
pub fn view_archive(datas: &[String]) {
    let title_text = TITLE_STYLE.apply_to("Archive(s)");
    println!();
    println!("{}", title_text);
    if datas.is_empty() {
        println!(" No archive");
    } else {
        for f in datas {
            println!(" {}", f);
        }
    }
    println!();
}

/// view of the search results
pub fn view_search(datas: &SearchOperationCollection) {
    let title_text = TITLE_STYLE.apply_to("Operation(s)");
    println!(
        "┌────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐"
    );
    println!("│ {:<115}│", title_text);
    println!(
        "├───────┬──────────┬───────┬──────────────────┬──────────────────┬───────────────────────────────────────────────────┤"
    );
    println!(
        "│Id     │Date      │ Type  │            Amount│           Balance│Description                                        │"
    );
    println!(
        "├───────┼──────────┼───────┼──────────────────┼──────────────────┼───────────────────────────────────────────────────┤"
    );

    for item in datas.items.iter() {
        let (id_style, row_style) = match item.void_by {
            Some(_) => (STYLE_DANGER, STYLE_MUTED),
            None => (STYLE_MUTED, Style::new()),
        };

        // Determine the color according to the flow (credit/debit)
        let amount_style = match item.void_by {
            Some(_) => STYLE_MUTED,
            None => match item.flow.as_str() {
                "Credit" => CREDIT_STYLE,
                "Debit" => DEBIT_STYLE,
                _ => STYLE_MUTED,
            },
        };
        let balance_style = match item.void_by {
            Some(_) => STYLE_MUTED,
            None => VALUE_STYLE,
        };

        let id_text = id_style.apply_to(format!("#{}", format_id_short(&item.id)));
        let amount_text = amount_style.apply_to(format!("{:.2}", item.amount).separate_with_commas());
        let balance_text = balance_style.apply_to(format!("{:.2}", item.balance).separate_with_commas());

        println!(
            "│{:<7}│{}│{:<7}│{:>18}│{:>18}│{:<51}│",
            id_text,
            row_style.apply_to(&item.date),
            row_style.apply_to(&item.flow),
            amount_text,
            balance_text,
            row_style.apply_to(truncate_text(&item.description, 50)),
        );
    }

    println!(
        "└───────┴──────────┴───────┴──────────────────┴──────────────────┴───────────────────────────────────────────────────┘"
    );
    println!();
    println!("Total operations found: {}", datas.counts.total());
    println!();
    let voided_op = STYLE_DANGER.apply_to("#XXXXX");
    println!("{}", NOTE_STYLE.apply_to("Note:"));
    println!("{} Operation voided", voided_op,);
    println!(
        "{}",
        NOTE_STYLE.apply_to("Descriptions longer than 50 characters are truncated with '...'.")
    );
    println!(
        "{}",
        NOTE_STYLE.apply_to("Remember to regularly perform closing operations to maintain accurate financial records.")
    );
    println!();
}
/// view to a summary of the codexi
pub fn view_summary(summary: &SummaryCollection) {
    let title_text = format!("{:<84}", "Current Acccount summary");
    println!("┌─────────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ {}│", TITLE_STYLE.apply_to(title_text));
    println!("├────────────────────────┬──────────────────┬─────────────────────────────────────────┤");
    println!(
        "│{:<24}│{:>18}│ {} {:>18} │",
        STYLE_NORMAL.apply_to("Regulars count"),
        VALUE_STYLE.apply_to(summary.counts.regular),
        STYLE_NORMAL.apply_to("Latest date regular:"),
        VALUE_STYLE.apply_to(summary.anchors.last_regular.clone().unwrap_or("..........".to_string()))
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>21} │",
        STYLE_NORMAL.apply_to("Init count"),
        VALUE_STYLE.apply_to(summary.counts.init),
        STYLE_NORMAL.apply_to("Latest date init:"),
        VALUE_STYLE.apply_to(summary.anchors.last_init.clone().unwrap_or("..........".to_string())),
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>15} │",
        STYLE_NORMAL.apply_to("Adjustments count"),
        VALUE_STYLE.apply_to(summary.counts.adjust),
        STYLE_NORMAL.apply_to("Latest date adjustment:"),
        VALUE_STYLE.apply_to(summary.anchors.last_adjust.clone().unwrap_or("..........".to_string())),
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>11} │",
        STYLE_NORMAL.apply_to("Void operation count"),
        VALUE_STYLE.apply_to(summary.counts.void),
        STYLE_NORMAL.apply_to("Latest date void operation:"),
        VALUE_STYLE.apply_to(summary.anchors.last_void.clone().unwrap_or("..........".to_string())),
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>18} │",
        STYLE_NORMAL.apply_to("Closings count"),
        VALUE_STYLE.apply_to(summary.counts.checkpoint),
        STYLE_NORMAL.apply_to("Latest date closing:"),
        VALUE_STYLE.apply_to(
            summary
                .anchors
                .last_checkpoint
                .clone()
                .unwrap_or("..........".to_string())
        ),
    );

    println!(
        "│{:<24}│{:>18}│                                         │",
        STYLE_NORMAL.apply_to("Operations count"),
        VALUE_STYLE.apply_to(summary.counts.total()).bold()
    );

    println!(
        "│{:<24}│{:>18}│                                         │",
        STYLE_NORMAL.apply_to("Balance"),
        VALUE_STYLE
            .apply_to(format!("{:.2}", summary.balance.total).separate_with_commas())
            .bold()
    );

    println!("└────────────────────────┴──────────────────┴─────────────────────────────────────────┘");
    println!();
    println!(
        "{}",
        NOTE_STYLE.apply_to("Note: 'latest date' corresponds to the most recent date for each operation type.")
    );
    println!(
        "{}",
        NOTE_STYLE.apply_to("Remember to regularly perform closing operations to maintain accurate financial records.")
    );
    println!();
}

pub fn view_stats(stats: &StatsCollection) {
    let savings_style = if stats.savings_rate >= Decimal::ZERO {
        Style::new().green().bold()
    } else {
        Style::new().red().bold()
    };

    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    let title_text = format!(
        "{:<77}",
        "Current account financial analytics (excl. init and checkpoint)"
    );
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
        "│ {}   {}   {} {:<12} │",
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
}

/// Utility function for the visual toolbar
fn draw_savings_bar(rate: Decimal, width: usize) -> String {
    if rate <= Decimal::ZERO {
        // red bar proportional to the deficit, max -100%
        let normalized = rate.abs().min(Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED;
        let filled = (normalized * Decimal::from(width)).to_usize().unwrap_or(0);
        let empty = width - filled;
        format!("{}{}", style("█".repeat(filled)).red(), style("░".repeat(empty)).dim())
    } else {
        // green bar proportional to the profit, max -100%
        let normalized = rate.min(Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED;
        let filled = (normalized * Decimal::from(width)).to_usize().unwrap_or(0);
        let empty = width - filled;
        format!(
            "{}{}",
            style("█".repeat(filled)).green(),
            style("░".repeat(empty)).dim()
        )
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;

    if b < KB {
        format!("{} B", bytes)
    } else if b < MB {
        format!("{:.2} KB", b / KB)
    } else if b < GB {
        format!("{:.2} MB", b / MB)
    } else {
        format!("{:.2} GB", b / GB)
    }
}
