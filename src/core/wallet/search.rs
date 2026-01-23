// src/core/wallet/search.rs

use anyhow::{Result, bail};
use rust_decimal::Decimal;

use crate::core::wallet::Codexi;
use crate::core::wallet::operation_kind::OperationKind;
use crate::core::wallet::operation_flow::OperationFlow;
use crate::core::wallet::operation::Operation;
use crate::core::helpers::parse_flexible_date_range;

/// Struct for search item
#[derive(Debug, Clone)]
pub struct SearchItem<'a> {
    pub index: usize,
    pub op: &'a Operation,
    pub balance: Decimal,
}

impl Codexi {

    /// Search
    /// Returns a vector of SearchItem
    pub fn search(
        &self,
        from: Option<String>,
        to: Option<String>,
        text: Option<String>,
        kind: Option<String>,
        flow: Option<String>,
        amount_min: Option<Decimal>,
        amount_max: Option<Decimal>,
        latest: Option<usize>,
    ) -> Result<Vec<SearchItem<'_>>> {

        let ops_map = self.get_operations_with_balance();

        let start_date = from
            .as_deref()
            .map(|d| parse_flexible_date_range(d, true))
            .transpose()?;

        let end_date = to
            .as_deref()
            .map(|d| parse_flexible_date_range(d, false))
            .transpose()?;

        if let (Some(start), Some(end)) = (start_date, end_date) {
            if end < start {
                bail!(
                    "Invalid date range: end date ({}) is before start date ({})",
                    end,
                    start
                );
            }
        }

        let text_lc = text.as_ref().map(|t| t.to_lowercase());

        let o_flow_filter = match flow {
            Some(ref s) => match OperationFlow::try_from(s.as_str()) {
                Ok(v) => Some(v),
                Err(_) => return Ok(Vec::new()),
            },
            None => None,
        };

        let o_kind_filter = match kind {
            Some(ref s) => match OperationKind::try_from(s.as_str()) {
                Ok(v) => Some(v),
                Err(_) => return Ok(Vec::new()),
            },
            None => None,
        };

        let mut matched: Vec<SearchItem> = Vec::new();

        for (_, &(op, bal)) in ops_map.iter().enumerate() {
            // from
            if let Some(s_date) = start_date {
                if op.date < s_date {
                    continue;
                }
            }

            // to
            if let Some(e_date) = end_date {
                if op.date > e_date {
                    continue;
                }
            }
            // text
            if let Some(ref needle) = text_lc {
                if !op.description.to_lowercase().contains(needle) {
                    continue;
                }
            }
            // debit/credit
            if let Some(f_op) = o_flow_filter {
                if op.flow != f_op {
                    continue;
                }
            }
            //adjust,close,init,void,transaction(trans)
            if let Some(k_op) = o_kind_filter {
                if op.kind != k_op {
                    continue;
                }
            }

            if let Some(min) = amount_min {
                if op.amount < min {
                    continue;
                }
            }

            if let Some(max) = amount_max {
                if op.amount > max {
                    continue;
                }
            }

            matched.push(SearchItem {
                index: op.id,
                op,
                balance: bal,
            });
        }

        let result = if let Some(n) = latest {
            if matched.len() <= n {
                matched
            } else {
                let start = matched.len().saturating_sub(n);
                matched[start..].to_vec()
            }
        } else {
            matched
        };

        Ok(result)
    }

}
