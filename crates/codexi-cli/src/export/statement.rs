// src/export/statement.rs

use anyhow::Result;
use minijinja::{Environment, context};
use rust_decimal::Decimal;
use thousands::Separable;

use codexi::dto::StatementCollection;

use crate::ui::{format_optional_bank_item, format_optional_currency_item};

const STATEMENT_TEMPLATE: &str = include_str!("../assets/templates/statement.html");

pub fn export_statement_html(entry: StatementCollection) -> Result<String> {
    let mut env = Environment::new();
    env.add_template("statement.html", STATEMENT_TEMPLATE)?;

    let operations: Vec<serde_json::Value> = entry
        .items
        .iter()
        .map(|op| {
            serde_json::json!({
                "date": op.date,
                "description": op.description,
                "debit": if op.debit == Decimal::ZERO {
                    String::new()
                } else {
                    format!("{:.2}", op.debit).separate_with_commas()
                },
                "credit": if op.credit == Decimal::ZERO {
                    String::new()
                } else {
                    format!("{:.2}", op.credit).separate_with_commas()
                },
            })
        })
        .collect();

    let ctx = context! {
        items => operations,
        date_min => &entry.from.unwrap_or_else(|| "N/A".to_string()),
        date_max => &entry.to.unwrap_or_else(|| "N/A".to_string()),
        operation_count => entry.counts.total(),
        account_name => &entry.account.name,
        account_number => &entry.account.id,
        account_bank => format_optional_bank_item(&entry.account.bank),
        account_currency => format_optional_currency_item(&entry.account.currency),
        balance_debit => &entry.balance.debit.separate_with_commas(),
        balance_credit => &entry.balance.credit.separate_with_commas(),
        balance_total => &entry.balance.total.separate_with_commas(),
    };

    let template = env.get_template("statement.html")?;
    Ok(template.render(ctx)?)
}
