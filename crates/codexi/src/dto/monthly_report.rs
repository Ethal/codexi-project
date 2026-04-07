// src/dto/monthly_report.rs

use rust_decimal::Decimal;

use crate::dto::StatsCollection;

#[derive(Debug)]
pub struct MonthlyReportItem {
    pub period: String,
    pub stats: StatsCollection,
}

#[derive(Debug)]
pub struct MonthlyReport {
    pub items: Vec<MonthlyReportItem>,
    pub total_credit: Decimal,
    pub total_debit: Decimal,
    pub total_balance: Decimal,
}

impl MonthlyReport {
    pub fn build(items: Vec<(String, StatsCollection)>) -> Self {
        let mut total_credit = Decimal::ZERO;
        let mut total_debit = Decimal::ZERO;
        let items = items
            .into_iter()
            .map(|(period, stats)| {
                total_credit += stats.total_credit;
                total_debit += stats.total_debit;
                MonthlyReportItem { period, stats }
            })
            .collect();
        Self {
            items,
            total_credit,
            total_debit,
            total_balance: total_credit - total_debit,
        }
    }
}
