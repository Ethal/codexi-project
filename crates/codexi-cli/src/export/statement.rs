// src/export/statement.rs

use anyhow::Result;
use rust_decimal::Decimal;
use tera::{Context, Tera};
use thousands::Separable;

use codexi::logic::account::StatementEntry;

const STATEMENT_TEMPLATE: &str = include_str!("../assets/templates/statement.html");

pub fn export_statement_html(entry: StatementEntry) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("statement.html", STATEMENT_TEMPLATE)?;

    let mut rows = Vec::new();
    for item in entry.items {
        rows.push(serde_json::json!({
            "date": item.date,
            "description": item.description,
            "debit": if item.debit == Decimal::ZERO{"".into()} else {item.debit.separate_with_commas()},
            "credit": if item.credit == Decimal::ZERO{"".into()} else {item.credit.separate_with_commas()}
        }));
    }

    let mut ctx = Context::new();
    ctx.insert("items", &rows);
    ctx.insert("date_min", &entry.date_min);
    ctx.insert("date_max", &entry.date_max);
    ctx.insert("operation_count", &entry.counts.total());
    ctx.insert("account_name", &entry.account_name);
    ctx.insert("account_number", &entry.account_number);
    ctx.insert("account_bank", &entry.account_bank);
    ctx.insert("account_currency", &entry.account_currency);
    ctx.insert("balance_debit", &entry.balance.debit.separate_with_commas());
    ctx.insert(
        "balance_credit",
        &entry.balance.credit.separate_with_commas(),
    );
    ctx.insert(
        "balance_total",
        &entry.balance.total().separate_with_commas(),
    );

    let html = tera.render("statement.html", &ctx)?;

    Ok(html)
}
