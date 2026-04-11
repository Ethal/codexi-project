// src/dto/stats.rs

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashSet;

use crate::{
    core::{format_date, format_id},
    dto::SearchOperationItem,
    logic::{
        account::Account,
        codexi::Codexi,
        search::{SearchOperation, SearchOperationList},
    },
    types::DateRange,
};

#[derive(Debug, Default)]
pub struct TopExpenseItem {
    pub op_id: String,
    pub op_date: String,
    pub description: String,
    pub amount: Decimal,
    pub percentage: Decimal,
}

#[derive(Debug, Default)]
pub struct StatsCollection {
    pub account_name: String,
    pub from: Option<String>,
    pub to: Option<String>,
    // Perimeter 1 — all flows (excl. init, checkpoint, void, voided)
    pub total_credit: Decimal,
    pub total_debit: Decimal,
    pub balance: Decimal,
    pub operation_count: usize,
    pub adjustment_count: usize,
    pub adjustment_percentage: Decimal,
    // Perimeter 2 — real economic flows (excl. transfers)
    pub real_total_credit: Decimal,
    pub real_total_debit: Decimal,
    pub savings_rate: Option<Decimal>,
    pub average_operation: Decimal,
    pub real_operation_count: usize,
    pub top_expenses: Vec<TopExpenseItem>,
    pub max_single_debit: Decimal,
    pub daily_average: Decimal,
    pub days_count: i64,
    // Ignored (void/voided pairs outside period)
    pub ignored: Vec<SearchOperationItem>,
}

impl StatsCollection {
    /// Builds financial statistics for a given account and search result.
    pub fn build(codexi: &Codexi, account: &Account, s_ops: &SearchOperationList) -> Self {
        let (from, to) = DateRange::compute(s_ops, s_ops.params.from, s_ops.params.to).formatted();

        let active: Vec<&SearchOperation> = s_ops.active_items().collect();
        if active.is_empty() {
            return Self::default();
        }

        // HashSet of active IDs for quick lookup
        let active_ids: HashSet<_> = active.iter().map(|i| i.operation.id).collect();

        // Map voided_id -> void_id to check if the pair is present
        let mut void_map = std::collections::HashMap::new();
        for op in &active {
            if op.operation.kind.is_void()
                && let Some(void_of) = op.operation.links.void_of
            {
                void_map.insert(void_of, op.operation.id);
            }
        }

        // Perimeter 1 accumulators
        let mut total_credit = Decimal::ZERO;
        let mut total_debit = Decimal::ZERO;
        let mut operation_count = 0usize;
        let mut adj_count = 0usize;
        // Perimeter 2 accumulators
        let mut real_credit = Decimal::ZERO;
        let mut real_debit = Decimal::ZERO;
        let mut real_op_count = 0usize;
        let mut max_debit = Decimal::ZERO;
        let mut expenses_candidates = Vec::new();

        let mut ignored = Vec::new();

        for op_item in &active {
            let op = &op_item.operation;
            let id = op.id;
            let is_void = op.is_void();
            let is_voided = op.is_voided();
            let is_adjust = op.is_adjust();
            let is_transfer = op.is_transfer();

            // Check if the operation can be included (void/voided pair logic)
            let include_in_stats = if is_voided {
                void_map.get(&id).is_some_and(|void_id| active_ids.contains(void_id))
            } else if is_void {
                let target_id = op.links.void_of.unwrap();
                active_ids.contains(&target_id)
            } else {
                true
            };

            if !include_in_stats {
                ignored.push(SearchOperationItem::build(codexi, account, op_item));
                continue;
            }

            // Perimeter 1 — all flows
            operation_count += 1;
            if is_adjust {
                adj_count += 1;
            }
            if op.flow.is_credit() {
                total_credit += op.amount;
            } else {
                total_debit += op.amount;
            }

            // Perimeter 2 — real economic flows (excl. Transfer DR)
            if !is_transfer || op.flow.is_credit() {
                real_op_count += 1;
                if op.flow.is_credit() {
                    real_credit += op.amount;
                } else {
                    real_debit += op.amount;
                    if !is_adjust && op.amount > max_debit {
                        max_debit = op.amount;
                    }
                }
                // Top 5 candidates: debit, not adjust, not transfer
                if op.flow.is_debit() && !is_adjust {
                    expenses_candidates.push(op);
                }
            }
        }

        // Perimeter 1 derived
        let balance = total_credit - total_debit;
        let adjustment_percentage = if operation_count > 0 {
            Decimal::from(adj_count) / Decimal::from(operation_count) * Decimal::ONE_HUNDRED
        } else {
            Decimal::ZERO
        };

        // Perimeter 2 derived
        let cal_savings_rate = if real_credit > Decimal::ZERO {
            ((real_credit - real_debit) / real_credit) * Decimal::ONE_HUNDRED
        } else if real_debit > Decimal::ZERO {
            dec!(-100)
        } else {
            Decimal::ZERO
        };

        let savings_rate = if account.has_saving_rate() {
            Some(cal_savings_rate)
        } else {
            None
        };

        let average_operation = if real_op_count > 0 {
            (real_credit + real_debit) / Decimal::from(real_op_count)
        } else {
            Decimal::ZERO
        };

        // Top 5 expenses
        let mut top_expenses: Vec<TopExpenseItem> = expenses_candidates
            .into_iter()
            .map(|op| TopExpenseItem {
                op_id: format_id(op.id),
                op_date: format_date(op.date),
                description: op.description.clone(),
                amount: op.amount,
                percentage: if real_debit > Decimal::ZERO {
                    (op.amount / real_debit) * Decimal::ONE_HUNDRED
                } else {
                    Decimal::ZERO
                },
            })
            .collect();

        top_expenses.sort_by(|a, b| {
            b.amount
                .partial_cmp(&a.amount)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.op_date.cmp(&a.op_date))
        });
        top_expenses.truncate(5);

        // Days count based on active operations
        let days_count = if let (Some(start), Some(end)) = (
            active.first().map(|i| i.operation.date),
            active.last().map(|i| i.operation.date),
        ) {
            (end - start).num_days() + 1
        } else {
            0
        };

        let daily_average = if days_count > 0 {
            real_debit / Decimal::from(days_count)
        } else {
            Decimal::ZERO
        };

        Self {
            account_name: account.name.clone(),
            from,
            to,
            total_credit,
            total_debit,
            balance,
            operation_count,
            adjustment_count: adj_count,
            adjustment_percentage,
            real_total_credit: real_credit,
            real_total_debit: real_debit,
            savings_rate,
            average_operation,
            real_operation_count: real_op_count,
            top_expenses,
            max_single_debit: max_debit,
            daily_average,
            days_count,
            ignored,
        }
    }
}
