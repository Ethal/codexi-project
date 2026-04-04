// src/handler/main.rs

use anyhow::Result;
use chrono::{Local, NaiveDate};
use rust_decimal::Decimal;
use std::path::Path;

use codexi::{
    core::{
        CoreError, DataPaths, format_date, parse_date, parse_decimal, parse_optional_decimal,
        parse_text,
    },
    dto::{AccountCollection, SearchOperationCollection, StatementCollection},
    file_management::FileManagement,
    logic::{
        account::Account,
        category::{Category, CategoryError},
        codexi::CodexiError,
        counterparty::{Counterparty, CounterpartyError},
        operation::{OperationFlow, OperationKind, RegularKind},
        search::{SearchParamsBuilder, search},
        utils::resolve_by_id_or_name,
    },
    types::DateRange,
};

use crate::{
    command::{Cli, RootCommand},
    export::export_statement_html,
    handler::{
        account::handle_account_command, admin::handle_admin_command, bank::handle_bank_command,
        category::handle_category_command, counterparty::handle_counterparty_command,
        currency::handle_currency_command, data::handle_data_command,
        history::handle_history_command, loan::handle_loan_command,
        operation::handle_operation_command, report::handle_report_command,
    },
    msg_info, msg_warn,
    ui::overview_account,
};

use crate::ui::view_search;

pub fn handle_root_command(cli: Cli, paths: &DataPaths, cwd: &Path) -> Result<()> {
    let skip_confirm = cli.yes;

    match cli.command {
        RootCommand::Overview {} => {
            let codexi = FileManagement::load_current_state(paths)?;
            let accounts = AccountCollection::build(&codexi);
            overview_account(&accounts);
        }

        RootCommand::Debit {
            date,
            amount,
            description,
            counterparty,
            category,
        } => {
            let amount_d = parse_decimal(&amount, "amount")?;
            let date = parse_date(&date)?;

            let mut codexi = FileManagement::load_current_state(paths)?;
            let category_id = category
                .map(|name| {
                    resolve_by_id_or_name::<Category, CategoryError>(
                        &name,
                        &codexi.categories.categories,
                    )
                })
                .transpose()?;
            let counterparty_id = counterparty
                .map(|name| {
                    resolve_by_id_or_name::<Counterparty, CounterpartyError>(
                        &name,
                        &codexi.counterparties.counterparties,
                    )
                })
                .transpose()?;
            let account = codexi.get_current_account_mut()?;

            account.register_transaction(
                date,
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Debit,
                amount_d,
                parse_text(description.clone()),
                counterparty_id,
                category_id,
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
            counterparty,
            category,
        } => {
            let (date_n, amount_n, desc_n) =
                normalize_date_amount_desc(&date, &amount, description)?;

            let mut codexi = FileManagement::load_current_state(paths)?;
            let category_id = category
                .map(|name| {
                    resolve_by_id_or_name::<Category, CategoryError>(
                        &name,
                        &codexi.categories.categories,
                    )
                })
                .transpose()?;
            let counterparty_id = counterparty
                .map(|name| {
                    resolve_by_id_or_name::<Counterparty, CounterpartyError>(
                        &name,
                        &codexi.counterparties.counterparties,
                    )
                })
                .transpose()?;
            let account = codexi.get_current_account_mut()?;

            let reg_kind = RegularKind::Transaction;
            account.register_transaction(
                date_n,
                OperationKind::Regular(reg_kind),
                OperationFlow::Credit,
                amount_n,
                desc_n.clone(),
                counterparty_id,
                category_id,
            )?;

            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Crebit operation added: {} {} {}", date_n, amount_n, desc_n);
        }
        RootCommand::Interest {
            date,
            amount,
            description,
            counterparty,
            category,
        } => {
            let (date_n, amount_n, desc_n) =
                normalize_date_amount_desc(&date, &amount, description)?;

            let mut codexi = FileManagement::load_current_state(paths)?;
            let category_id = category
                .map(|name| {
                    resolve_by_id_or_name::<Category, CategoryError>(
                        &name,
                        &codexi.categories.categories,
                    )
                })
                .transpose()?;
            let counterparty_id = counterparty
                .map(|name| {
                    resolve_by_id_or_name::<Counterparty, CounterpartyError>(
                        &name,
                        &codexi.counterparties.counterparties,
                    )
                })
                .transpose()?;
            let account = codexi.get_current_account_mut()?;

            let reg_kind = RegularKind::Interest;
            account.register_transaction(
                date_n,
                OperationKind::Regular(reg_kind),
                OperationFlow::Credit,
                amount_n,
                desc_n.clone(),
                counterparty_id,
                category_id,
            )?;

            FileManagement::save_current_state(&codexi, paths)?;
            msg_info!("Crebit operation added: {} {} {}", date_n, amount_n, desc_n);
        }
        RootCommand::Transfer {
            date,
            amount_from,
            amount_to,
            account_id_to,
            description,
            category,
        } => {
            let mut codexi = FileManagement::load_current_state(paths)?;

            let date = parse_date(&date)?;
            let amount_from_d = parse_decimal(&amount_from, "amount fom")?;
            let amount_to_d = parse_decimal(&amount_to, "amount to")?;
            let acc_id_to =
                resolve_by_id_or_name::<Account, CodexiError>(&account_id_to, &codexi.accounts)?;
            let desc = parse_text(description.clone());
            let category_id = category
                .map(|name| {
                    resolve_by_id_or_name::<Category, CategoryError>(
                        &name,
                        &codexi.categories.categories,
                    )
                })
                .transpose()?;

            codexi.transfer(
                date,
                amount_from_d,
                acc_id_to,
                amount_to_d,
                desc.clone(),
                category_id,
            )?;

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
            open,
        } => {
            let amount_min_d = parse_optional_decimal(&amount_min, "amount_min")?;
            let amount_max_d = parse_optional_decimal(&amount_max, "amount_max")?;
            let mut range = DateRange::parse(from.as_deref(), to.as_deref())?;
            if today {
                let from = format_date(Local::now().date_naive());
                let to = format_date(Local::now().date_naive());
                range = DateRange::parse(Some(from.as_ref()), Some(to.as_ref()))?;
            }

            let codexi = FileManagement::load_current_state(paths)?;
            let account = codexi.get_current_account()?;

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

            let s_ops = search(account, &params)?;
            let search_items = SearchOperationCollection::build(&codexi, account, &s_ops);
            let statement_results = StatementCollection::build(&codexi, account, &s_ops);
            if search_items.items.is_empty() {
                msg_warn!("No data available as per criteria.");
                return Ok(());
            }
            if open {
                let html = export_statement_html(statement_results)?;
                let file_path = FileManagement::export_html(&html, cwd)?;
                msg_info!("search completed (report.html)");
                opener::open_browser(file_path)?;
            } else {
                view_search(&search_items);
            }
        }
        RootCommand::Operation(args) => handle_operation_command(args.command, paths)?,
        RootCommand::Report(args) => handle_report_command(args.command, cwd, paths)?,
        RootCommand::Data(args) => handle_data_command(args.command, cwd, paths, skip_confirm)?,
        RootCommand::History(args) => handle_history_command(args.command, paths)?,
        RootCommand::Admin(args) => handle_admin_command(args.command, cwd, paths, skip_confirm)?,
        RootCommand::Account(args) => handle_account_command(args.command, paths)?,
        RootCommand::Bank(args) => handle_bank_command(args.command, paths)?,
        RootCommand::Currency(args) => handle_currency_command(args.command, paths)?,
        RootCommand::Counterparty(args) => handle_counterparty_command(args.command, paths)?,
        RootCommand::Category(args) => handle_category_command(args.command, paths)?,
        RootCommand::Loan(args) => handle_loan_command(args.command, paths)?,
    }
    Ok(())
}

/// Normalized the date, amount, description
fn normalize_date_amount_desc(
    date: &str,
    amount: &str,
    description: Vec<String>,
) -> Result<(NaiveDate, Decimal, String), CoreError> {
    let date_n = parse_date(date)?;
    let amount_d = parse_decimal(amount, "amount")?;
    let desc = parse_text(description.clone());
    Ok((date_n, amount_d, desc))
}
