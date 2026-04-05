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
    pub from: Option<String>,
    pub to: Option<String>,
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
    pub ignored: Vec<SearchOperationItem>,
}

impl StatsCollection {
    pub fn build(codexi: &Codexi, account: &Account, s_ops: &SearchOperationList) -> Self {
        // compute the from / to date
        let (from, to) = DateRange::compute(s_ops, s_ops.params.from, s_ops.params.to).formatted();

        let active: Vec<&SearchOperation> = s_ops.active_items().collect();
        if active.is_empty() {
            return Self::default();
        }

        // HashSet of active IDs for quick lookup
        let active_ids: HashSet<_> = active.iter().map(|i| i.operation.id).collect();

        // Map voided_id -> void_id pour savoir si la paire est présente
        let mut void_map = std::collections::HashMap::new();
        for op in &active {
            if op.operation.kind.is_void()
                && let Some(void_of) = op.operation.links.void_of
            {
                void_map.insert(void_of, op.operation.id); // op est le void
            }
        }

        let mut total_credit = Decimal::ZERO;
        let mut total_debit = Decimal::ZERO;
        let mut max_debit = Decimal::ZERO;
        let mut adj_count = 0usize;
        let mut operation_count = 0usize;
        let mut ignored = Vec::new();
        let mut expenses_candidates = Vec::new();

        for op_item in &active {
            let op = &op_item.operation;
            let id = op.id;
            let is_void = op.is_void();
            let is_voided = op.is_voided();
            let is_adjust = op.is_adjust();

            // Check if the operation can be included
            let include_in_stats = if is_voided {
                // Operation voided : include only if its void is present
                void_map
                    .get(&id)
                    .is_some_and(|void_id| active_ids.contains(void_id))
            } else if is_void {
                // Opération void : include only if its voided is present
                let target_id = op.links.void_of.unwrap();
                active_ids.contains(&target_id)
            } else {
                // Normal operation
                true
            };

            if !include_in_stats {
                ignored.push(SearchOperationItem::build(codexi, account, op_item));
                continue;
            }

            operation_count += 1;

            if is_adjust {
                adj_count += 1
            }

            // basic calculation debit/credit , max debit does not included adjust, void, voided
            if op.flow.is_credit() {
                total_credit += op.amount;
            } else {
                total_debit += op.amount;
                if (!is_adjust || !is_void || !is_voided) && op.amount > max_debit {
                    max_debit = op.amount;
                }
            }

            // Candidat for the top 5 expenses
            if op.flow.is_debit() && !is_adjust && !is_void && !is_voided {
                expenses_candidates.push(op);
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

        // Top 5 dépenses
        let mut top_expenses: Vec<TopExpenseItem> = expenses_candidates
            .into_iter()
            .map(|op| TopExpenseItem {
                op_id: format_id(op.id),
                op_date: format_date(op.date),
                description: op.description.clone(),
                amount: op.amount,
                percentage: if total_debit > Decimal::ZERO {
                    (op.amount / total_debit) * Decimal::ONE_HUNDRED
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

        // Durée en jours
        let days_count = if let (Some(start), Some(end)) = (
            active.first().map(|i| i.operation.date),
            active.last().map(|i| i.operation.date),
        ) {
            (end - start).num_days() + 1
        } else {
            0
        };
        let daily_average = if days_count > 0 {
            total_debit / Decimal::from(days_count)
        } else {
            Decimal::ZERO
        };

        let adjustment_percentage = if operation_count > 0 {
            Decimal::from(adj_count) / Decimal::from(operation_count) * Decimal::ONE_HUNDRED
        } else {
            Decimal::ZERO
        };

        Self {
            from,
            to,
            total_credit,
            total_debit,
            balance,
            savings_rate,
            average_operation,
            operation_count,
            top_expenses,
            max_single_debit: max_debit,
            daily_average,
            days_count,
            adjustment_count: adj_count,
            adjustment_percentage,
            ignored,
        }
    }
}
