// src/export/stats.rs

use anyhow::Result;
use rust_decimal::Decimal;
use tera::{Context, Tera};
use thousands::Separable;

use codexi::{core::format_id_short, dto::StatsCollection};

const STATS_TEMPLATE: &str = include_str!("../assets/templates/stats.html");

pub fn export_stats_html(entry: StatsCollection) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("stats.html", STATS_TEMPLATE)?;

    // Savings bar : clamp 0–100 for the bar width, flag negative for color
    let savings_rate_negative = entry.savings_rate < Decimal::ZERO;
    let savings_bar_pct = entry
        .savings_rate
        .max(Decimal::ZERO)
        .min(Decimal::ONE_HUNDRED)
        .to_string();

    let top_expenses: Vec<serde_json::Value> = entry
        .top_expenses
        .iter()
        .map(|exp| {
            serde_json::json!({
                "op_id":      format!("#{}",format_id_short(&exp.op_id)),
                "op_date":    exp.op_date,
                "description": exp.description,
                "amount":     format!("{:.2}",exp.amount).separate_with_commas(),
                "percentage": format!("{:.1}", exp.percentage),
            })
        })
        .collect();

    let ignored: Vec<serde_json::Value> = entry
        .ignored
        .iter()
        .map(|op| {
            serde_json::json!({
                "op_id":      format!("#{}",format_id_short(&op.id)),
                "op_date":    op.date,
                "type":       op.flow,
                "amount":     format!("{:.2}",op.amount).separate_with_commas(),
                "description": op.description,
            })
        })
        .collect();

    let mut ctx = Context::new();

    // Overview
    ctx.insert("date_min", &entry.from.unwrap_or("N/A".to_string()));
    ctx.insert("date_max", &entry.to.unwrap_or("N/A".to_string()));
    ctx.insert(
        "total_credit",
        &format!("{:.2}", entry.total_credit).separate_with_commas(),
    );
    ctx.insert(
        "total_debit",
        &format!("{:.2}", entry.total_debit).separate_with_commas(),
    );
    ctx.insert("balance", &format!("{:.2}", entry.balance).separate_with_commas());
    ctx.insert("operation_count", &entry.operation_count);
    ctx.insert(
        "average_operation",
        &format!("{:.2}", entry.average_operation).separate_with_commas(),
    );
    ctx.insert(
        "daily_average",
        &format!("{:.2}", entry.daily_average).separate_with_commas(),
    );

    // Savings bar
    ctx.insert("savings_rate", &format!("{:.2}", entry.savings_rate));
    ctx.insert("savings_rate_negative", &savings_rate_negative);
    ctx.insert("savings_bar_pct", &savings_bar_pct);

    // Insights
    ctx.insert(
        "max_single_debit",
        &format!("{:.2}", entry.max_single_debit).separate_with_commas(),
    );
    ctx.insert("adjustment_count", &entry.adjustment_count);
    ctx.insert("adjustment_percentage", &format!("{:.1}", entry.adjustment_percentage));
    ctx.insert("days_count", &entry.days_count);

    // Top expenses
    ctx.insert("top_expenses", &top_expenses);

    // Ignored
    ctx.insert("ignored", &ignored);

    let html = tera.render("stats.html", &ctx)?;

    Ok(html)
}
