// src/dto/exchange_rate.rs

use rust_decimal::Decimal;

use crate::{
    core::{format_date, format_id},
    logic::{account::Account, codexi::Codexi, search::SearchOperationList},
};

#[derive(Debug)]
pub struct ExchangeRateItem {
    pub op_id: String,
    pub date: String,
    pub amount: Decimal,
    pub rate: Decimal,
    pub cost: Decimal,
    pub description: String,
}

#[derive(Debug)]
pub struct ExchangeRateCollection {
    pub account_name: String,
    pub account_currency: String,
    pub cost_currency: Option<String>,
    pub avg_rate: Decimal,
    pub best_rate: Decimal,
    pub worst_rate: Decimal,
    pub items: Vec<ExchangeRateItem>,
}

impl ExchangeRateCollection {
    pub fn build(codexi: &Codexi, account: &Account, s_ops: &SearchOperationList) -> Self {
        let account_name = account.name.clone();

        let account_currency = account
            .currency_id
            .as_ref()
            .and_then(|id| codexi.currencies.currency_code_by_id(id))
            .unwrap_or_default();

        let cost_currency = {
            let others: Vec<String> = codexi
                .currencies
                .iter()
                .filter(|c| Some(c.id) != account.currency_id)
                .filter_map(|c| codexi.currencies.currency_code_by_id(&c.id))
                .collect();
            if others.len() == 1 {
                Some(others[0].clone())
            } else {
                None
            }
        };

        // Collect operations with a meaningful exchange rate (rate != 1)
        let items: Vec<ExchangeRateItem> = s_ops
            .active_items()
            .filter(|so| {
                let op = &so.operation;
                op.context.exchange_rate != Decimal::ONE && op.context.exchange_rate > Decimal::ZERO
            })
            .map(|so| {
                let op = &so.operation;
                let rate = op.context.exchange_rate;
                let cost = if op.flow.is_debit() {
                    op.amount * rate
                } else {
                    op.amount * rate
                };
                ExchangeRateItem {
                    op_id: format_id(op.id),
                    date: format_date(op.date),
                    amount: op.amount,
                    rate,
                    cost,
                    description: op.description.clone(),
                }
            })
            .collect();

        if items.is_empty() {
            return Self {
                account_name,
                account_currency,
                cost_currency,
                avg_rate: Decimal::ZERO,
                best_rate: Decimal::ZERO,
                worst_rate: Decimal::ZERO,
                items,
            };
        }

        // Compute stats
        let avg_rate = items.iter().map(|i| i.rate).sum::<Decimal>() / Decimal::from(items.len());

        let best_rate = items
            .iter()
            .map(|i| i.rate)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(Decimal::ZERO);

        let worst_rate = items
            .iter()
            .map(|i| i.rate)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(Decimal::ZERO);

        Self {
            account_name,
            account_currency,
            cost_currency,
            avg_rate,
            best_rate,
            worst_rate,
            items,
        }
    }
}
