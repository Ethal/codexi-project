// src/wallet/reports.rs

use rust_decimal::Decimal;
use rust_decimal_macros::dec;


use crate::core::wallet::codexi::Codexi;
use crate::core::wallet::search::SearchItem;
use crate::core::wallet::system_kind::SystemKind;
use crate::core::wallet::regular_kind::RegularKind;
use crate::core::wallet::operation_kind::OperationKind;
use crate::core::wallet::operation_flow::OperationFlow;
use crate::core::wallet::operation::Operation;

/// Struct for resume result
#[derive(Debug, Clone)]
pub struct ResumeResult {
    pub nb_transaction: usize,
    pub nb_init: usize,
    pub nb_adjust: usize,
    pub nb_void: usize,
    pub nb_close: usize,
    pub nb_op: usize,
    pub balance: Decimal,
    pub latest_transaction_date: String,
    pub latest_init_date: String,
    pub latest_adjust_date: String,
    pub latest_void_date: String,
    pub latest_close_date: String,
}
/// Struct for balance result
#[derive(Debug, Clone, Default)]
pub struct BalanceResult {
    pub credit: Decimal,
    pub debit: Decimal,
    pub total: Decimal,
}

/// Structure related to detailed statistics
#[derive(Debug)]
pub struct DetailedStats {
    pub max_single_debit: Decimal,
    pub daily_average: Decimal,
    pub days_count: i64,
    pub adjustment_count: usize,
    pub adjustment_percentage: Decimal,
}

/// Structure that represent the top expense
#[derive(Debug)]
pub struct TopExpense {
    pub op_id: usize,
    pub description: String,
    pub amount: Decimal,
    pub percentage: Decimal,
}

/// Structure that represent the result of the statics
#[derive(Debug)]
pub struct StatsResult {
    pub total_credit: Decimal,
    pub total_debit: Decimal,
    pub balance: Decimal,
    pub savings_rate: Decimal, // (Credit - Debit) / Credit * 100
    pub average_operation: Decimal,
    pub operation_count: usize,
    pub top_expenses: Vec<TopExpense>,
    pub detailed: DetailedStats,
}

impl Codexi {

    /// Calculates the total of credits, debits and the final balance,
    /// with several date filters (from/to/day/month/year).
    /// Returns a BalanceResult struct.
    pub fn balance(&self, items: &[SearchItem]) -> Option<BalanceResult> {
        let mut credit = Decimal::ZERO;
        let mut debit  = Decimal::ZERO;

        if items.is_empty() {
            log::warn!("No balance available as per criteria");
            return None;
        }

        for item in items {
            match item.op.flow {
                OperationFlow::Credit => credit += item.op.amount,
                OperationFlow::Debit  => debit  += item.op.amount,
                OperationFlow::None   => {}
            }
        }

        let total = credit - debit;

        Some(BalanceResult {
            credit: credit,
            debit:  debit,
            total:  total,
        })
    }

    /// Resume
    /// Returns a ResumeResult struct
    pub fn resume(&self, items: &[SearchItem]) -> Option<ResumeResult> {

        if items.is_empty() {
            log::warn!("No resume available");
            return None;
        }

        let mut nb_transaction: usize = 0;
        let mut nb_init: usize = 0;
        let mut nb_adjust: usize = 0;
        let mut nb_void: usize = 0;
        let mut nb_close: usize = 0;
        let mut latest_transaction_date = String::from("__________");
        let mut latest_init_date = String::from("__________");
        let mut latest_adjust_date = String::from("__________");
        let mut latest_void_date = String::from("__________");
        let mut latest_close_date = String::from("__________");

        for op in &self.operations {
            match op.kind {
                OperationKind::Regular(RegularKind::Transaction) => {
                    nb_transaction += 1;
                    latest_transaction_date = op.date.format("%Y-%m-%d").to_string();
                }
                OperationKind::System(SystemKind::Init) => {
                    nb_init += 1;
                    latest_init_date = op.date.format("%Y-%m-%d").to_string();
                }
                OperationKind::System(SystemKind::Adjust) => {
                    nb_adjust += 1;
                    latest_adjust_date = op.date.format("%Y-%m-%d").to_string();
                }
                OperationKind::System(SystemKind::Void) => {
                    nb_void += 1;
                    latest_void_date = op.date.format("%Y-%m-%d").to_string();
                }
                OperationKind::System(SystemKind::Close) => {
                    nb_close += 1;
                    latest_close_date = op.date.format("%Y-%m-%d").to_string();
                }
                _ => { /* Ignore other types of operations */ }
            }
        }

        let balance = self
            .balance(&items)
            .unwrap_or_default();

        let nb_op = nb_transaction + nb_init + nb_adjust + nb_close;

        Some(ResumeResult {
            nb_transaction,
            nb_init,
            nb_adjust,
            nb_void,
            nb_close,
            nb_op,
            balance: balance.total,
            latest_transaction_date,
            latest_init_date,
            latest_adjust_date,
            latest_void_date,
            latest_close_date,
        })
    }

    ///Performed the stats calculations
    pub fn stats(&self, items: &[SearchItem], net: bool) -> Option<StatsResult> {

        let mut total_credit = Decimal::ZERO;
        let mut total_debit = Decimal::ZERO;
        let mut max_debit = Decimal::ZERO;
        let mut adj_count = 0;

        let all_ops: Vec<&Operation> = items.iter().map(|i| i.op).collect();

        let active_items: Vec<&SearchItem> = items.iter()
                .filter(|i| !i.op.kind.is_structural())
                .collect();

        if active_items.is_empty() {
            log::warn!("No statistics available as per criteria");
            return None;
        }

        let operation_count = active_items.len();

        // 1. Calculating the totals (debit/credit)
        for item in &active_items {
            let op = &item.op;

            // Case op void
            if op.kind.is_void() {
                if net {
                    let signed = op.signed_amount();
                    if signed > Decimal::ZERO {
                        total_credit += signed;
                    } else {
                        total_debit += -signed;
                    }
                }
                continue;
            }

            // Case op voided (mode historique)
            if !net && op.is_voided(&all_ops) {
                continue;
            }

            // Normal Operation
            let signed = op.signed_amount();
            if signed > Decimal::ZERO {
                total_credit += signed;
            } else {
                total_debit += -signed;
            }
        }
        let balance = total_credit - total_debit;

        // 2. Savings Rate
        let savings_rate = if total_credit > Decimal::ZERO {
            ((total_credit - total_debit) / total_credit) * Decimal::ONE_HUNDRED
        } else if total_debit > Decimal::ZERO {
            dec!(-100) // expenditure without income
        } else {
            Decimal::ZERO
        };

        // 3. Average per transaction
        let average_operation = if operation_count > 0 {
            (total_credit + total_debit) / Decimal::from(operation_count)
        } else {
            Decimal::ZERO
        };

        // 4. Extraction and sorting of the Top 5 expenses (exclude Adjust)
        let mut expenses: Vec<TopExpense> = active_items.iter()
            .filter(|i| i.op.flow.is_debit())
            // Exclude adjust in the Top 5 of expense.
            .filter(|i| !matches!(i.op.kind, OperationKind::System(SystemKind::Adjust) | OperationKind::System(SystemKind::Void) ))
            .map(|i| TopExpense {
                op_id: i.op.id,
                description: i.op.description.clone(),
                amount: i.op.amount,
                percentage: if total_debit > Decimal::ZERO {
                    (i.op.amount / total_debit) * Decimal::ONE_HUNDRED
                } else {
                    Decimal::ZERO
                },
            })
            .collect();

        // Sorted in descending order by amount
        expenses.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap_or(std::cmp::Ordering::Equal));
        expenses.truncate(5);

        // max expense and count the ajustments,
        for item in &active_items {
            if item.op.flow.is_debit()
                && !matches!(item.op.kind, OperationKind::System(SystemKind::Adjust))
                && item.op.amount > max_debit
            {
                max_debit = item.op.amount;
            }
            if matches!(item.op.kind, OperationKind::System(SystemKind::Adjust)) {
                adj_count += 1;
            }
        }

        // Calculating the duration of the period in days
        let start = active_items.first().unwrap().op.date;
        let end   = active_items.last().unwrap().op.date;

        let days_count = (end - start).num_days() + 1;

        // Normalized
        let daily_average = total_debit / Decimal::from(days_count);
        let adj_pct = (Decimal::from(adj_count) / Decimal::from(operation_count)) * Decimal::ONE_HUNDRED;

        let detailed = DetailedStats {
            max_single_debit: max_debit,
            daily_average,
            days_count,
            adjustment_count: adj_count,
            adjustment_percentage: adj_pct,
        };

        Some(StatsResult {
            total_credit: total_credit,
            total_debit: total_debit,
            balance: balance,
            savings_rate: savings_rate,
            average_operation: average_operation,
            operation_count,
            top_expenses: expenses,
            detailed: detailed,
        })
    }
}
