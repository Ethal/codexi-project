// src/ui.rs
use console::{Style, StyledObject, style};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use thousands::Separable;

use codexi::{
    core::CoreWarning,
    file_management::CodexiInfos,
    logic::{
        account::{SearchEntry, StatsEntry, SummaryEntry},
        balance::BalanceItem,
        operation::OperationFlow,
        utils::MIN_SHORT_LEN,
    },
};


use crate::ui::{CREDIT_STYLE, DEBIT_STYLE, LABEL_STYLE, NOTE_STYLE, TITLE_STYLE, VALUE_STYLE};

pub fn format_long_id_to_short(id: &str) -> String {
    let len = id.len();
    let start = len.saturating_sub(MIN_SHORT_LEN);
    id[start..].to_string()
}

pub fn view_warning(warnings: &[CoreWarning]) {
    let title_text = TITLE_STYLE.apply_to("Warning(s)");
    println!("{}", title_text);
    for warn in warnings {
        println!(" {}", warn);
    }
}

/// view of the codexi infos.
pub fn view_codexi_infos(datas: &CodexiInfos) {
    println!("📒 {}", TITLE_STYLE.apply_to("Infos:"));
    println!("Codexi data format version: {}", datas.data_version);
    println!("Codexi exchange format version: {}", datas.exchange_version);
    println!("Codexi storage format: {}", datas.storage_format);
    println!();
    println!("{}", LABEL_STYLE.apply_to("Codexi:"));
    println!("Account count: {}", datas.codexi_account_count);
    println!("Bank count: {}", datas.codexi_account_count);
    println!("Currency count: {}", datas.codexi_currency_count);
    println!("Category count: {}", datas.codexi_category_count);
    println!();
    let usage = &datas.disk_usage;
    println!("📦 {}", TITLE_STYLE.apply_to("Codexi disk usage"));
    println!();
    println!("data_dir/");
    println!(
        "  codexi.dat       {:>12}",
        format_bytes(usage.data_dir.codexi.size_bytes)
    );
    println!(
        "  snapshots/       {:>12}  ({} files)",
        format_bytes(usage.data_dir.snapshots.total_bytes),
        usage.data_dir.snapshots.file_count
    );
    println!(
        "  archives/        {:>12}  ({} account, {} files)",
        format_bytes(usage.data_dir.archives.total_bytes),
        usage.data_dir.archives.account_count,
        usage.data_dir.archives.file_count
    );

    println!("  ────────────────────────────────");
    println!(
        "  total data_dir   {:>12}",
        format_bytes(usage.data_dir.total_bytes)
    );

    println!();
    println!(
        "trash/             {:>12}  ({} restore points)",
        format_bytes(usage.trash.total_bytes),
        usage.trash.restore_point_count
    );

    println!();
    println!("TOTAL              {:>12}", format_bytes(usage.total_bytes));
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
        let note_text = NOTE_STYLE
            .apply_to("Note: Considered to performe a `data clean`, snapshot file grether than 5.");
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
/// view the balance (credit/debit/balance)
pub fn view_balance(balance: &BalanceItem) {
    let title_text = TITLE_STYLE.apply_to("codexi balance summary");
    let credit_value =
        CREDIT_STYLE.apply_to(format!("{:.2}", balance.credit).separate_with_commas());
    let debit_value = DEBIT_STYLE.apply_to(format!("{:.2}", balance.debit).separate_with_commas());
    let balance_value =
        VALUE_STYLE.apply_to(format!("{:.2}", balance.total()).separate_with_commas());

    println!();
    println!("{}", title_text);
    println!(" {:<10}{:>18}", "Credit:", credit_value);
    println!(" {:<10}{:>18}", "Debit:", debit_value);
    println!(" {:<10}{:>18}", "Balance:", balance_value);
    println!();
}
/// view of the search results
pub fn view_search(items: &SearchEntry) {
    let title_text = TITLE_STYLE.apply_to("Operation(s)");
    println!(
        "┌─────────────────────────────────────────────────────────────────────────────────────────────────────────┐"
    );
    println!("│ {:<104}│", title_text);
    println!(
        "├───────┬──────────┬───────┬──────────────────┬──────────────────┬────────────────────────────────────────┤"
    );
    println!(
        "│Id     │Date      │ Type  │            Amount│           Balance│Description                             │"
    );
    println!(
        "├───────┼──────────┼───────┼──────────────────┼──────────────────┼────────────────────────────────────────┤"
    );

    for item in items.iter() {
        // Determine the color according to the flow (credit/debit)
        let amount_style = match item.operation.flow {
            OperationFlow::Credit => CREDIT_STYLE,
            OperationFlow::Debit => DEBIT_STYLE,
            OperationFlow::None => LABEL_STYLE,
        };
        let amount_text =
            amount_style.apply_to(format!("{:.2}", item.operation.amount).separate_with_commas());

        let index_str = item.operation.id.to_string();
        let index_text = LABEL_STYLE.apply_to(format!("#{}",format_long_id_to_short(&index_str)));

        println!(
            "│{:<7}│{}│{}│{:>18}│{:>18}│{:<40}│",
            index_text,
            item.operation.date,
            item.operation.flow,
            amount_text,
            VALUE_STYLE.apply_to(format!("{:.2}", item.balance).separate_with_commas()),
            truncate_desc(&item.operation.description, 40),
        );
    }

    println!(
        "└───────┴──────────┴───────┴──────────────────┴──────────────────┴────────────────────────────────────────┘"
    );
    println!();
    println!("Total operations found: {}", items.items.len());
    println!();
    println!(
        "{}",
        NOTE_STYLE
            .apply_to("Note: Descriptions longer than 40 characters are truncated with '...'.")
    );
    println!(
        "{}",
        NOTE_STYLE.apply_to("Remember to regularly perform closing operations to maintain accurate financial records.")
    );
    println!();
}
/// view to a summary of the codexi
pub fn view_summary(summary: &SummaryEntry) {
    let title_text = format!("{:<84}", "Current Acccount summary");
    println!(
        "┌─────────────────────────────────────────────────────────────────────────────────────┐"
    );
    println!("│ {}│", TITLE_STYLE.apply_to(title_text));
    println!(
        "├────────────────────────┬──────────────────┬─────────────────────────────────────────┤"
    );
    println!(
        "│{:<24}│{:>18}│ {} {:>18} │",
        LABEL_STYLE.apply_to("Regulars count"),
        VALUE_STYLE.apply_to(summary.counts.regular),
        LABEL_STYLE.apply_to("Latest date regular:"),
        VALUE_STYLE.apply_to(
            summary
                .anchors
                .last_regular
                .clone()
                .unwrap_or("..........".to_string())
        )
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>21} │",
        LABEL_STYLE.apply_to("Init count"),
        VALUE_STYLE.apply_to(summary.counts.init),
        LABEL_STYLE.apply_to("Latest date init:"),
        VALUE_STYLE.apply_to(
            summary
                .anchors
                .last_init
                .clone()
                .unwrap_or("..........".to_string())
        ),
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>15} │",
        LABEL_STYLE.apply_to("Adjustments count"),
        VALUE_STYLE.apply_to(summary.counts.adjust),
        LABEL_STYLE.apply_to("Latest date adjustment:"),
        VALUE_STYLE.apply_to(
            summary
                .anchors
                .last_adjust
                .clone()
                .unwrap_or("..........".to_string())
        ),
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>11} │",
        LABEL_STYLE.apply_to("Void operation count"),
        VALUE_STYLE.apply_to(summary.counts.void),
        LABEL_STYLE.apply_to("Latest date void operation:"),
        VALUE_STYLE.apply_to(
            summary
                .anchors
                .last_void
                .clone()
                .unwrap_or("..........".to_string())
        ),
    );

    println!(
        "│{:<24}│{:>18}│ {} {:>18} │",
        LABEL_STYLE.apply_to("Closings count"),
        VALUE_STYLE.apply_to(summary.counts.checkpoint),
        LABEL_STYLE.apply_to("Latest date closing:"),
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
        LABEL_STYLE.apply_to("Operations count"),
        VALUE_STYLE.apply_to(summary.counts.total()).bold()
    );

    println!(
        "│{:<24}│{:>18}│                                         │",
        LABEL_STYLE.apply_to("Balance"),
        VALUE_STYLE
            .apply_to(format!("{:.2}", summary.balance.total()).separate_with_commas())
            .bold()
    );

    println!(
        "└────────────────────────┴──────────────────┴─────────────────────────────────────────┘"
    );
    println!();
    println!(
        "{}",
        NOTE_STYLE.apply_to(
            "Note: 'latest date' corresponds to the most recent date for each operation type."
        )
    );
    println!(
        "{}",
        NOTE_STYLE.apply_to("Remember to regularly perform closing operations to maintain accurate financial records.")
    );
    println!();
}

pub fn view_stats(stats: &StatsEntry) {
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
        LABEL_STYLE.apply_to("total credit"),
        CREDIT_STYLE.apply_to(format!("{:.2}", stats.total_credit).separate_with_commas()),
        LABEL_STYLE.apply_to("ops count:"),
        VALUE_STYLE.apply_to(ops_count_val),
    );

    // Line 2 related to total_debit/ avg/op
    let avg_op_val = format!("{:.2}", stats.average_operation);
    println!(
        "│ {:<20} │ {:>16} │ {} {:<26} │",
        LABEL_STYLE.apply_to("total debit"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.total_debit).separate_with_commas()),
        LABEL_STYLE.apply_to("avg/op:"),
        VALUE_STYLE.apply_to(avg_op_val)
    );

    // Line 3 related to balance
    println!(
        "│ {:<20} │ {:>16} │ {:<26} │",
        LABEL_STYLE.apply_to("balance"),
        VALUE_STYLE.apply_to(format!("{:.2}", stats.balance).separate_with_commas()),
        " ".repeat(34)
    );

    println!("├──────────────────────┴──────────────────┴────────────────────────────────────┤");

    let label = LABEL_STYLE.apply_to("savings rate");
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
        LABEL_STYLE.apply_to("daily burning rate:"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.daily_average).separate_with_commas()),
        LABEL_STYLE.apply_to("period length:"),
        VALUE_STYLE.apply_to(format!("{} days", stats.days_count))
    );

    // Largest expense and account quality (adjustments)
    println!(
        "│ {:<20}{:>18} │ {:<21}{:>14} │",
        LABEL_STYLE.apply_to("max single expense:"),
        DEBIT_STYLE.apply_to(format!("{:.2}", stats.max_single_debit).separate_with_commas()),
        LABEL_STYLE.apply_to("adjustments:"),
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
            LABEL_STYLE.apply_to(index_str),
            LABEL_STYLE.apply_to(&exp.op_date),
            DEBIT_STYLE.apply_to(format!("{:.2}", exp.amount).separate_with_commas()),
            VALUE_STYLE.apply_to(pct_str),
            LABEL_STYLE.apply_to(truncate_desc(&exp.description, 31))
        );
    }
    println!("└────────┴──────────┴──────────────────┴───────┴───────────────────────────────┘");
}

/// Utility function for the visual toolbar
fn draw_savings_bar(rate: Decimal, width: usize) -> StyledObject<String> {
    if rate < Decimal::ZERO {
        style("!".repeat(width).to_string()).red()
    } else {
        let normalized = rate.max(Decimal::ZERO).min(Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED;
        let filled = (normalized * Decimal::from(width)).to_usize().unwrap_or(0);
        let empty = width - filled;
        style(format!(
            "{}{}",
            style("█".repeat(filled)).green(),
            style("░".repeat(empty)).dim()
        ))
    }
}

/// Truncate description for display
fn truncate_desc(desc: &str, max_width: usize) -> String {
    // If the visible length is already OK → simple formatting
    if desc.chars().count() <= max_width {
        return format!("{:<width$}", desc, width = max_width);
    }

    // Otherwise → truncate without ever breaking a UTF-8 character
    let visible = max_width.saturating_sub(3);

    let truncated: String = desc.chars().take(visible).collect();

    format!("{:<width$}", format!("{}...", truncated), width = max_width)
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
