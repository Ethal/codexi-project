// src/logic/account/reports.rs

use nulid::Nulid;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{
    core::{format_date, format_id},
    logic::{
        account::{Account, AccountAnchors, SearchEntry, SearchItem},
        balance::{Balance, BalanceItem},
        counts::Counts,
        operation::{OperationKind, SystemKind},
    },
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountAnchorsItem {
    pub last_regular: Option<String>,
    pub last_init: Option<String>,
    pub last_adjust: Option<String>,
    pub last_void: Option<String>,
    pub last_checkpoint: Option<String>,
}

impl From<&AccountAnchors> for AccountAnchorsItem {
    fn from(a: &AccountAnchors) -> Self {
        Self {
            last_regular: a.last_regular.map(format_date),
            last_init: a.last_init.map(format_date),
            last_adjust: a.last_adjust.map(format_date),
            last_void: a.last_void.map(format_date),
            last_checkpoint: a.last_checkpoint.map(format_date),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SummaryEntry {
    pub counts: Counts,
    pub balance: BalanceItem,
    pub anchors: AccountAnchorsItem,
}

impl SummaryEntry {
    /// Build a SummaryEntry from items and account.
    pub fn new(items: &SearchEntry, account: &Account) -> Option<Self> {
        if items.is_empty() {
            return None;
        }

        Some(Self {
            counts: Counts::new(items),
            balance: BalanceItem::from(Balance::new(items)),
            anchors: AccountAnchorsItem::from(&account.anchors),
        })
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TopExpenseItem {
    pub op_id: String,
    pub op_date: String,
    pub description: String,
    pub amount: Decimal,
    pub percentage: Decimal,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatsEntry {
    pub total_credit: Decimal,
    pub total_debit: Decimal,
    pub balance: Decimal,
    pub savings_rate: Decimal, // (Credit - Debit) / Credit * 100
    pub average_operation: Decimal,
    pub operation_count: usize,
    pub top_expenses: Vec<TopExpenseItem>,
    pub max_single_debit: Decimal,
    pub daily_average: Decimal,
    pub days_count: i64,
    pub adjustment_count: usize,
    pub adjustment_percentage: Decimal,
}

impl StatsEntry {
    /// Performed the stats calculations
    pub fn new(items: &SearchEntry, net: bool) -> Option<Self> {
        let active_items: Vec<&SearchItem> = items.active_items().collect();

        if active_items.is_empty() {
            return None;
        }

        let voided_in_period: HashSet<Nulid> = active_items
            .iter()
            .filter_map(|i| i.operation.links.void_by)
            .collect();

        let operation_count = active_items.len();

        let mut total_credit = Decimal::ZERO;
        let mut total_debit = Decimal::ZERO;
        let mut max_debit = Decimal::ZERO;
        let mut adj_count = 0usize;

        //  calcul totals + max_debit + adj_count
        for item in &active_items {
            let op = &item.operation;
            let is_voided = voided_in_period.contains(&op.id);
            let is_adjust = matches!(op.kind, OperationKind::System(SystemKind::Adjust));
            let is_void_op = op.kind.is_void();

            if is_adjust {
                adj_count += 1;
            }

            // credit / debit
            let include = if is_void_op { net } else { net || !is_voided };

            if include {
                let signed = op.signed_amount();
                if signed > Decimal::ZERO {
                    total_credit += signed;
                } else {
                    total_debit += -signed;
                }
            }

            // max debit
            if include && op.flow.is_debit() && !is_adjust && !is_void_op && op.amount > max_debit {
                max_debit = op.amount;
            }
        }

        let balance = total_credit - total_debit;

        let savings_rate = if total_credit > Decimal::ZERO {
            ((total_credit - total_debit) / total_credit) * Decimal::ONE_HUNDRED
        } else if total_debit > Decimal::ZERO {
            dec!(-100)
        } else {
            Decimal::ZERO
        };

        let average_operation = if operation_count > 0 {
            (total_credit + total_debit) / Decimal::from(operation_count)
        } else {
            Decimal::ZERO
        };

        let mut expenses: Vec<TopExpenseItem> = active_items
            .iter()
            .filter(|i| i.operation.flow.is_debit())
            .filter(|i| {
                !matches!(
                    i.operation.kind,
                    OperationKind::System(SystemKind::Adjust)
                        | OperationKind::System(SystemKind::Void)
                )
            })
            .filter(|i| net || !voided_in_period.contains(&i.operation.id))
            .map(|i| TopExpenseItem {
                op_id: format_id(i.operation.id),
                op_date: format_date(i.operation.date),
                description: i.operation.description.clone(),
                amount: i.operation.amount,
                percentage: if total_debit > Decimal::ZERO {
                    (i.operation.amount / total_debit) * Decimal::ONE_HUNDRED
                } else {
                    Decimal::ZERO
                },
            })
            .collect();

        expenses.sort_by(|a, b| {
            b.amount
                .partial_cmp(&a.amount)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        expenses.truncate(5);

        let adj_pct =
            (Decimal::from(adj_count) / Decimal::from(operation_count)) * Decimal::ONE_HUNDRED;

        let start = active_items.first().unwrap().operation.date;
        let end = active_items.last().unwrap().operation.date;
        let days_count = (end - start).num_days() + 1;
        let daily_average = total_debit / Decimal::from(days_count);

        Some(Self {
            total_credit,
            total_debit,
            balance,
            savings_rate,
            average_operation,
            operation_count,
            top_expenses: expenses,
            max_single_debit: max_debit,
            daily_average,
            days_count,
            adjustment_count: adj_count,
            adjustment_percentage: adj_pct,
        })
    }
}
