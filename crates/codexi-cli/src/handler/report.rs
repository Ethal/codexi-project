// src/handler/report.rs

use anyhow::Result;
use rust_decimal::Decimal;
use std::path::Path;

use codexi::{
    core::DataPaths,
    file_management::FileManagement,
    logic::{
        account::{SearchParamsBuilder, search},
        balance::{Balance, BalanceItem},
    },
    types::DateRange,
};

use crate::{
    command::ReportCommand,
    export::{export_statement_html, export_stats_html},
    msg_info, msg_warn,
    ui::{view_balance, view_balance_account, view_stats, view_summary},
};

pub fn handle_report_command(command: ReportCommand, cwd: &Path, paths: &DataPaths) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    let account = codexi.get_current_account_mut()?;
    match command {
        ReportCommand::BalanceAll {} => {
            let items = codexi.account_entry();
            view_balance_account(&items);
        }
        ReportCommand::Balance { from, to } => {
            let range = DateRange::parse(from.as_deref(), to.as_deref())?;
            let params = SearchParamsBuilder::default()
                .from(range.from)
                .to(range.to)
                .build()?;

            let balance_items = search(account, &params)?;
            let balance = BalanceItem::from(Balance::new(&balance_items));
            if balance.total == Decimal::ZERO
                && balance.credit == Decimal::ZERO
                && balance.debit == Decimal::ZERO
            {
                msg_warn!("No data available");
            } else {
                view_balance(&balance);
            }
        }
        ReportCommand::Stats {
            from,
            to,
            net,
            open,
        } => {
            let range = DateRange::parse(from.as_deref(), to.as_deref())?;
            let params = SearchParamsBuilder::default()
                .from(range.from)
                .to(range.to)
                .build()?;

            if let Some(stats) = account.stats_entry(&params, net) {
                if open {
                    let html = export_stats_html(stats)?;
                    let file_path = FileManagement::export_html(&html, cwd)?;
                    msg_info!("stats completed (report.html)");
                    opener::open_browser(file_path)?;
                } else {
                    view_stats(&stats);
                }
            } else {
                msg_warn!("No data available");
            }
        }
        ReportCommand::Summary {} => {
            let params = SearchParamsBuilder::default().build()?;
            let summary = account.summary_entry(&params);
            view_summary(&summary);
        }
        ReportCommand::Statement { from, to, open } => {
            let range = DateRange::parse(from.as_deref(), to.as_deref())?;
            let params = SearchParamsBuilder::default()
                .from(range.from)
                .to(range.to)
                .build()?;
            let account_id = codexi.get_current_account()?.id;
            if let Some(statement_results) = codexi.statement_entry(&account_id, &params) {
                if statement_results.items.is_empty() {
                    msg_warn!("No data available");
                } else {
                    let html = export_statement_html(statement_results)?;
                    let file_path = FileManagement::export_html(&html, cwd)?;
                    msg_info!("statement completed (report.html)");
                    if open {
                        opener::open_browser(file_path)?;
                    }
                }
            } else {
                msg_warn!("No data available");
            }
        }
    }
    Ok(())
}
