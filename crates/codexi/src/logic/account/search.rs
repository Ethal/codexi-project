// src/codexi/account/search.rs

use chrono::NaiveDate;
use derive_builder::Builder;
use nulid::Nulid;
use rust_decimal::Decimal;

use crate::logic::{
    account::{OperationContainer, SearchError},
    operation::{Operation, OperationFlow, OperationKind},
    utils::HasNulid,
};

/// Struct for search entry
#[derive(Debug, Clone)]
pub struct SearchEntry {
    pub items: Vec<SearchItem>,
}

/// Struct for search item
#[derive(Debug, Clone)]
pub struct SearchItem {
    pub operation: Operation,
    pub balance: Decimal,
}

impl SearchEntry {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn iter(&self) -> impl Iterator<Item = &SearchItem> {
        self.items.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    ///Return a SearchItem or None
    pub fn get_searchitem_by_id(&self, si_id: Nulid) -> Option<&SearchItem> {
        self.items.iter().find(|si| si.operation.id == si_id)
    }

    pub fn active_items(&self) -> impl Iterator<Item = &SearchItem> {
        self.items
            .iter()
            .filter(|i| !i.operation.kind.is_structural()) // not the init and the checkpoint
    }

    pub fn is_empty_active(&self) -> bool {
        self.active_items().next().is_none()
    }

    pub fn last_n(&self, n: usize) -> Self {
        let len = self.items.len();
        let start = len.saturating_sub(n);
        Self {
            items: self.items[start..].to_vec(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Default for SearchEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl HasNulid for SearchItem {
    fn id(&self) -> Nulid {
        self.operation.id
    }
}

#[derive(Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SearchParams {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub text: Option<String>,
    pub kind: Option<String>,
    pub flow: Option<String>,
    pub amount_min: Option<Decimal>,
    pub amount_max: Option<Decimal>,
    pub latest: Option<usize>,
}

impl SearchParamsBuilder {
    pub fn build(&self) -> Result<SearchParams, SearchError> {
        self.fallible_build()
            .map_err(|e| SearchError::SearchParametersBuilder(e.to_string()))
    }
}

/// Search
/// Returns SearchEntry
pub fn search<T: OperationContainer>(
    container: &T,
    params: &SearchParams,
) -> Result<SearchEntry, SearchError> {
    let ops_map = container.get_operations_with_balance();
    let from = params.from;
    let to = params.to;
    let text = params.text.as_deref();
    let kind = params.kind.as_deref();
    let flow = params.flow.as_deref();
    let amount_min = params.amount_min;
    let amount_max = params.amount_max;
    let latest = params.latest;

    if let (Some(start), Some(end)) = (from, to)
        && end < start
    {
        return Err(SearchError::InvalidDate(format!(
            "Invalid date range: end date ({}) is before start date ({})",
            end, start
        )));
    }

    let o_flow_filter = match flow {
        Some(s) => match OperationFlow::try_from(s) {
            Ok(v) => Some(v),
            Err(_) => return Ok(SearchEntry::new()),
        },
        None => None,
    };

    let o_kind_filter = match kind {
        Some(s) => match OperationKind::try_from(s) {
            Ok(v) => Some(v),
            Err(_) => return Ok(SearchEntry::new()),
        },
        None => None,
    };

    let mut matched: SearchEntry = SearchEntry::new();

    for item in ops_map {
        let op = &item.operation;

        // from
        if let Some(s_date) = from
            && op.date < s_date
        {
            continue;
        }

        // to
        if let Some(e_date) = to
            && op.date > e_date
        {
            continue;
        }
        // text
        if let Some(ref needle) = text
            && !op.description.to_lowercase().contains(needle)
        {
            continue;
        }
        // debit/credit
        if let Some(f_op) = o_flow_filter
            && op.flow != f_op
        {
            continue;
        }
        //adjust,close,init,void,transaction(trans)
        if let Some(k_op) = o_kind_filter
            && op.kind != k_op
        {
            continue;
        }
        // min amount
        if let Some(min) = amount_min
            && op.amount < min
        {
            continue;
        }
        // max amount
        if let Some(max) = amount_max
            && op.amount > max
        {
            continue;
        }

        matched.items.push(item);
    }

    let result = if let Some(n) = latest {
        if matched.items.len() <= n {
            matched
        } else {
            matched.last_n(n)
        }
    } else {
        matched
    };

    Ok(result)
}
