// src/handler/main.rs

use anyhow::Result;
use chrono::Local;
use std::path::Path;

use codexi::{
    core::{DataPaths, format_date, parse_date, parse_decimal, parse_optional_decimal, parse_text},
    file_management::FileManagement,
    logic::{
        account::{Account, SearchParamsBuilder, search},
        codexi::CodexiError,
        operation::{OperationFlow, OperationKind, RegularKind},
        utils::resolve_id,
    },
    types::DateRange,
};

use crate::handler::account::handle_account_command;
use crate::handler::admin::handle_admin_command;
use crate::handler::bank::handle_bank_command;
use crate::handler::category::handle_category_command;
use crate::handler::currency::handle_currency_command;
use crate::handler::data::handle_data_command;
use crate::handler::history::handle_history_command;
use crate::handler::report::handle_report_command;
use crate::{
    command::{Cli, RootCommand},
    msg_info, msg_warn,
};

use crate::ui::view_search;

pub fn handle_root_command(cli: Cli, paths: &DataPaths, cwd: &Path) -> Result<()> {
    let skip_confirm = cli.yes;

    match cli.command {
        RootCommand::Debit {
            date,
            amount,
            description,
        } => {
            let amount_d = parse_decimal(&amount, "amount")?;
            let date = parse_date(&date)?;

            let mut codexi = FileManagement::load_current_state(paths)?;
            let account = codexi.get_current_account_mut()?;

            account.register_transaction(
                date,
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Debit,
                amount_d,
                parse_text(description.clone()),
            )?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!(
                "Debit operation added: {} {} {}",
                date,
                amount_d,
                &description.join(" ")
            );
        }

        RootCommand::Credit {
            date,
            amount,
            description,
        } => {
            let amount_d = parse_decimal(&amount, "amount")?;
            let date = parse_date(&date)?;

            let mut codexi = FileManagement::load_current_state(paths)?;
            let account = codexi.get_current_account_mut()?;

            account.register_transaction(
                date,
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Credit,
                amount_d,
                parse_text(description.clone()),
            )?;
            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!(
                "Crebit operation added: {} {} {}",
                date,
                amount_d,
                description.join(" ")
            );
        }
        RootCommand::Transfer {
            date,
            amount_from,
            amount_to,
            account_id_to,
            description,
        } => {
            let mut codexi = FileManagement::load_current_state(paths)?;

            let date = parse_date(&date)?;
            let amount_from_d = parse_decimal(&amount_from, "amount fom")?;
            let amount_to_d = parse_decimal(&amount_to, "amount to")?;
            let acc_id_to = resolve_id::<Account, CodexiError>(&account_id_to, &codexi.accounts)?;
            let desc = parse_text(description.clone());

            codexi.transfer(date, amount_from_d, acc_id_to, amount_to_d, desc.clone())?;

            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!(
                "Transfer operation added: {} {} {} to {} {}",
                date,
                amount_from_d,
                amount_to_d,
                acc_id_to,
                desc
            );
        }

        RootCommand::Search {
            from,
            to,
            text,
            kind,
            flow,
            amount_min,
            amount_max,
            last,
            today,
        } => {
            let amount_min_d = parse_optional_decimal(&amount_min, "amount_min")?;
            let amount_max_d = parse_optional_decimal(&amount_max, "amount_max")?;
            let mut range = DateRange::parse(from.as_deref(), to.as_deref())?;
            if today {
                let from = format_date(Local::now().date_naive());
                let to = format_date(Local::now().date_naive());
                range = DateRange::parse(Some(from.as_ref()), Some(to.as_ref()))?;
            }

            let mut codexi = FileManagement::load_current_state(paths)?;
            let account = codexi.get_current_account_mut()?;

            let params = SearchParamsBuilder::default()
                .from(range.from)
                .to(range.to)
                .text(text)
                .kind(kind)
                .flow(flow)
                .amount_min(amount_min_d)
                .amount_max(amount_max_d)
                .latest(last)
                .build()?;

            let search_items = search(account, &params)?;
            if search_items.is_empty() {
                msg_warn!("No data available as per criteria.");
            } else {
                view_search(&search_items);
            }
        }
        RootCommand::Report(args) => handle_report_command(args.command, cwd, paths)?,
        RootCommand::Data(args) => handle_data_command(args.command, cwd, paths, skip_confirm)?,
        RootCommand::History(args) => handle_history_command(args.command, paths)?,
        RootCommand::Admin(args) => handle_admin_command(args.command, cwd, paths, skip_confirm)?,
        RootCommand::Account(args) => handle_account_command(args.command, paths)?,
        RootCommand::Bank(args) => handle_bank_command(args.command, paths)?,
        RootCommand::Currency(args) => handle_currency_command(args.command, paths)?,
        RootCommand::Category(args) => handle_category_command(args.command, paths)?,
    }
    Ok(())
}
