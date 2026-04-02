// src/export/statement.rs

use anyhow::Result;
use rust_decimal::Decimal;
use tera::{Context, Tera};
use thousands::Separable;

use codexi::dto::StatementCollection;

use crate::ui::{format_optional_bank_item, format_optional_currency_item};

const STATEMENT_TEMPLATE: &str = include_str!("../assets/templates/statement.html");

pub fn export_statement_html(entry: StatementCollection) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("statement.html", STATEMENT_TEMPLATE)?;

    let operations: Vec<serde_json::Value> = entry
        .items
        .iter()
        .map(|op| {
            serde_json::json!({
                "date":    op.date,
                "description": op.description,
                "debit": if op.debit == Decimal::ZERO{"".into()} else {format!("{:.2}",op.debit).separate_with_commas()},
                "credit": if op.credit == Decimal::ZERO{"".into()} else {format!("{:.2}",op.credit).separate_with_commas()},
            })
        })
        .collect();

    let mut ctx = Context::new();
    ctx.insert("items", &operations);
    ctx.insert("date_min", &entry.from.unwrap_or("N/A".to_string()));
    ctx.insert("date_max", &entry.to.unwrap_or("N/A".to_string()));
    ctx.insert("operation_count", &entry.counts.total());
    ctx.insert("account_name", &entry.account.name);
    ctx.insert("account_number", &entry.account.id);
    ctx.insert(
        "account_bank",
        &format_optional_bank_item(&entry.account.bank),
    );
    ctx.insert(
        "account_currency",
        &format_optional_currency_item(&entry.account.currency),
    );
    ctx.insert("balance_debit", &entry.balance.debit.separate_with_commas());
    ctx.insert(
        "balance_credit",
        &entry.balance.credit.separate_with_commas(),
    );
    ctx.insert("balance_total", &entry.balance.total.separate_with_commas());

    let html = tera.render("statement.html", &ctx)?;

    Ok(html)
}
