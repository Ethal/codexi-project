// src/core/wallet/viewer.rs

use thousands::Separable;
use owo_colors::{OwoColorize, Style};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::search::SearchItem;
use crate::core::wallet::reports::BalanceResult;
use crate::core::wallet::reports::ResumeResult;
use crate::core::wallet::reports::StatsResult;
use crate::core::wallet::operation_flow::OperationFlow;
use crate::core::wallet::file_management::LedgerInfos;

/// Methods for viewing the ledger datas
impl Codexi {

    /// view of the ledger(codexi) infos.
    pub fn view_ledger_infos(datas: &LedgerInfos ) {

        println!("{}", "Infos:".bold());
        println!("Ledger version: {}", datas.version);
        println!("Ledger storage format: {}", datas.storage_format);
        println!();
        println!("{}", "Current Ledger".italic());
        println!("Number of operation in the current ledger: {}", datas.nb_current_op);
        println!();
        println!("{}", "Archives:".italic());
        println!("Number of archived: {}", datas.nb_archive_file);
        for d in &datas.archive_infos {
            println!("Archive name: {}, Number of operations: {}", d.name, d.nb_op);
        }
        println!();
    }

    /// view to list the snapshot file
    pub fn view_snapshot(datas: &[String]) {

        let note_style = Style::new().blue().italic();

        println!("┌─────────────────────────────┐");
        let title_text = format!("{:<28}", "Snapshot(s)");
        println!("│ {}│", title_text.cyan().bold());
        println!("├─────────────────────────────┤");
        if datas.len() == 0 {
            println!("│ {:<28}│", "No snapshot");
        } else {
            for f in datas {
                println!("│ {:<28}│", f);
            }
        }
        println!("└─────────────────────────────┘");
        if datas.len() > 5 {
            println!();
            println!("{}", "Note: Considered to performe a `data clean`, snapshot file grether than 5.".style(note_style));
            println!();
        }

    }
    /// view to list the archive file
    pub fn view_archive(datas: &[String]) {

        println!("┌─────────────────────────────┐");
        let title_text = format!("{:<28}", "Archive(s)");
        println!("│ {}│", title_text.cyan().bold());
        println!("├─────────────────────────────┤");
        if datas.len() == 0 {
            println!("│ {:<28}│", "No archive");
        } else {
            for f in datas {
                println!("│ {:<28}│", f);
            }
        }
        println!("└─────────────────────────────┘");
    }
    /// view the balance (credit/debit/balance)
    pub fn view_balance(balance: &BalanceResult) {
        println!("┌───────────────────────────┐");
        println!("│ {}    │", "codexi balance summary".cyan().bold());
        println!("├────────┬──────────────────┤");
        println!("│Credit  │{:>18}│", format!("{:.2}", balance.credit).separate_with_commas().green());
        println!("│Debit   │{:>18}│", format!("{:.2}", balance.debit).separate_with_commas().red());
        println!("│Balance │{:>18}│", format!("{:.2}", balance.total).separate_with_commas().yellow().bold());
        println!("└────────┴──────────────────┘");
    }
    /// view of the search results
    pub fn view_search(rows: &[SearchItem]){
        println!("┌─────────────────────────────────────────────────────────────────────────────────────────────────────────┐");
        let title_text = format!("{:<104}", "Operation(s)");
        println!("│ {}│", title_text.bold().cyan());
        println!("├───────┬──────────┬───────┬──────────────────┬──────────────────┬────────────────────────────────────────┤");
        println!("│Index  │Date      │ Type  │           Montant│           Balance│Description                             │");
        println!("├───────┼──────────┼───────┼──────────────────┼──────────────────┼────────────────────────────────────────┤");

        for item in rows {
            // Determine the color according to the flow (credit/debit)
            let amount_str = format!("{:.2}", item.op.amount).separate_with_commas();
            let amount_style = match item.op.flow {
                OperationFlow::Credit => Style::new().green(),
                OperationFlow::Debit  => Style::new().red(),
                OperationFlow::None   => Style::new().dimmed(),
            };
            let colored_amount = amount_str.style(amount_style);

            let index_style = Style::new().dimmed();
            let index_str = format!("#{}", item.index);
            let colored_index = index_str.style(index_style);

            println!(
                "│{:<7}│{}│{}│{:>18}│{:>18}│{:<40}│",
                colored_index,
                item.op.date,
                item.op.flow,
                colored_amount,
                format!("{:.2}", item.balance).separate_with_commas().yellow(),
                Self::truncate_desc(&item.op.description, 40),
            );
        }

        let note_style = Style::new().blue().italic();

        println!("└───────┴──────────┴───────┴──────────────────┴──────────────────┴────────────────────────────────────────┘");
        println!();
        println!("Total operations found: {}", rows.len());
        println!();
        println!("{}", "Note: Descriptions longer than 40 characters are truncated with '...'.".style(note_style));
        println!("{}", "Remember to regularly perform closing operations to maintain accurate financial records.".style(note_style));
        println!();
    }
    /// view to resume the codexi
    pub fn view_resume(resume: &ResumeResult) {

        let title_style = Style::new().cyan().bold();
        let label_style = Style::new().dimmed();
        let value_style = Style::new().yellow();
        let note_style = Style::new().blue().italic();

        println!("┌─────────────────────────────────────────────────────────────────────────────────────┐");
        let title_text = format!("{:<84}", "Current ledger resume");
        println!("│ {}│", title_text.style(title_style));
        println!("├────────────────────────┬──────────────────┬─────────────────────────────────────────┤");
        println!("│{:<24}│{:>18}│ {} {:>13} │",
                "Number of transactions".style(label_style),
                resume.nb_transaction.style(value_style),
                "Latest date transactions:".style(label_style),
                resume.latest_transaction_date.style(value_style));

        println!("│{:<24}│{:>18}│ {} {:>21} │",
                "Number of init".style(label_style),
                resume.nb_init.style(value_style),
                "Latest date init:".style(label_style),
                resume.latest_init_date.style(value_style));

        println!("│{:<24}│{:>18}│ {} {:>15} │",
                "Number of adjustments".style(label_style),
                resume.nb_adjust.style(value_style),
                "Latest date adjustment:".style(label_style),
                resume.latest_adjust_date.style(value_style));

        println!("│{:<24}│{:>18}│ {} {:>11} │",
                "Number of void operation".style(label_style),
                resume.nb_void.style(value_style),
                "Latest date void operation:".style(label_style),
                resume.latest_void_date.style(value_style));

        println!("│{:<24}│{:>18}│ {} {:>18} │",
                "Number of closings ".style(label_style),
                resume.nb_close.style(value_style),
                "Latest date closing:".style(label_style),
                resume.latest_close_date.style(value_style));

        println!("│{:<24}│{:>18}│                                         │",
            "total operations".style(label_style),
            resume.nb_op.style(value_style).bold());

        println!("│{:<24}│{:>18}│                                         │",
            "Balance".style(label_style),
            format!("{:.2}", resume.balance).separate_with_commas().style(value_style).bold());

        println!("└────────────────────────┴──────────────────┴─────────────────────────────────────────┘");
        println!();
        println!("{}", "Note: 'latest date' corresponds to the most recent date for each operation type.".style(note_style));
        println!("{}", "Remember to regularly perform closing operations to maintain accurate financial records.".style(note_style));
        println!();
    }

    pub fn view_stats(stats: &StatsResult) {
        let title_style = Style::new().cyan().bold();
        let label_style = Style::new().dimmed();
        let value_style = Style::new().yellow();
        let savings_style = if stats.savings_rate >= Decimal::ZERO { Style::new().green().bold() } else { Style::new().red().bold() };

        println!("┌──────────────────────────────────────────────────────────────────────────────┐");
        let title_text = format!("{:<77}", "ledger financial analytics (exlude Initial amount and closing)");
        println!("│ {}│", title_text.style(title_style));
        println!("├──────────────────────┬──────────────────┬────────────────────────────────────┤");

        // Line 1 related to total_credit/op count
        let ops_count_val = format!("{}", stats.operation_count);
        println!("│ {:<20} │ {:>16} │ {} │",
            "total credit".style(label_style),
            format!("{:.2}", stats.total_credit).separate_with_commas().green(),
            format!("{} {:<23}", "ops count:".style(label_style), ops_count_val.style(value_style)));


        // Line 2 related to total_debit/ avg/op
        let avg_op_val = format!("{:.2}", stats.average_operation);
        println!("│ {:<20} │ {:>16} │ {} │",
            "total debit".style(label_style),
            format!("{:.2}", stats.total_debit).separate_with_commas().red(),
            format!("{} {:<26}","avg/op:".style(label_style), avg_op_val.style(value_style)));

        // Line 3 related to balance
        println!("│ {:<20} │ {:>16} │ {} │",
            "balance".style(label_style),
            format!("{:.2}", stats.balance).separate_with_commas().yellow(),
            format!("{:<26}"," ".repeat(34)));

        println!("├──────────────────────┴──────────────────┴────────────────────────────────────┤");

        let label = "savings rate".style(label_style);
        let rate_val = format!("{:>12.2}%", stats.savings_rate);
        let bar = Self::draw_savings_bar(stats.savings_rate, 32);
        println!("│ {}   {}   {} {:<12} │",
            label,
            rate_val.style(savings_style),
            bar,
            ""
        );

        println!("├──────────────────────────────────────────────────────────────────────────────┤");
        println!("│ {:<76} │", "behavioral insights & system health".style(title_style));
        println!("├────────────────────────────────────────┬─────────────────────────────────────┤");

        // Spending Rate and Duration
        println!("│ {:<20}{:>18} │ {:<21}{:>14} │",
            "daily burning rate:".style(label_style),
            format!("{:.2}", stats.detailed.daily_average).red(),
            "period length:".style(label_style),
            format!("{} days", stats.detailed.days_count).style(value_style));

        // Largest expense and Ledger quality (adjustments)
        println!("│ {:<20}{:>18} │ {:<21}{:>14} │",
            "max single expense:".style(label_style),
            format!("{:.2}", stats.detailed.max_single_debit).red().bold(),
            "adjustments:".style(label_style),
            format!("{} ({:.1}%)", stats.detailed.adjustment_count, stats.detailed.adjustment_percentage).style(value_style));

        println!("├────────────────────────────────────────┴─────────────────────────────────────┤");

        // Section Top Expenses
        println!("│ {:<76} │", "top 5 expenses (excl. adjust)".style(title_style));
        println!("├────────┬──────────────────┬───────┬──────────────────────────────────────────┤");

        for (_, exp) in stats.top_expenses.iter().enumerate() {
            let index_str = format!("#{:<7}", exp.op_id);
            let pct_str = format!("{:>6.1}%", exp.percentage);
            println!("│{}│{:>18}│{}│{:<42}│",
                index_str.style(label_style),
                format!("{:.2}", exp.amount).separate_with_commas().red(),
                pct_str.style(value_style),
                Self::truncate_desc(&exp.description, 42).italic()
            );
        }
        println!("└────────┴──────────────────┴───────┴──────────────────────────────────────────┘");
    }

    /// Utility function for the visual toolbar
    fn draw_savings_bar(rate: Decimal, width: usize) -> String {
        if rate < Decimal::ZERO {
            format!("{}", "!".repeat(width).red())
        } else {
            let normalized = rate.max(Decimal::ZERO).min(Decimal::ONE_HUNDRED) / Decimal::ONE_HUNDRED;
            let filled = (normalized * Decimal::from(width)).to_usize().unwrap_or(0);
            let empty = width - filled;
            format!("{}{}", "█".repeat(filled).green(), "░".repeat(empty).dimmed())
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

}
