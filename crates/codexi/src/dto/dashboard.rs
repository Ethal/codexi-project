// src/dto/dashboard.rs

use rust_decimal::Decimal;

use crate::dto::{
    CategoryStatsCollection, CategoryStatsItem, CounterpartyStatsCollection, CounterpartyStatsItem, StatsCollection,
    TopExpenseItem,
};

#[derive(Debug)]
pub struct DashboardCollection {
    pub account_name: String,
    pub from: Option<String>,
    pub to: Option<String>,
    // Financial summary
    pub total_credit: Decimal,
    pub total_debit: Decimal,
    pub balance: Decimal,
    pub savings_rate: Decimal,
    pub op_count: usize,
    pub average_operation: Decimal,
    // Top 5
    pub top_expenses: Vec<TopExpenseItem>,              //  StatsCollection
    pub top_categories: Vec<CategoryStatsItem>,         // top 5 of CategoryStatsCollection
    pub top_counterparties: Vec<CounterpartyStatsItem>, // top 5 of CounterpartyStatsCollection
}

impl DashboardCollection {
    pub fn build(stats: StatsCollection, cp: CounterpartyStatsCollection, ca: CategoryStatsCollection) -> Self {
        Self {
            account_name: stats.account_name,
            from: stats.from,
            to: stats.to,
            total_credit: stats.total_credit,
            total_debit: stats.total_debit,
            balance: stats.balance,
            savings_rate: stats.savings_rate,
            op_count: stats.operation_count,
            average_operation: stats.average_operation,
            top_expenses: stats.top_expenses,
            top_categories: ca.top_debit,
            top_counterparties: cp.top_debit,
        }
    }
}
