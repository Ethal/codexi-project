// src/logic/search/operation.rs

use chrono::NaiveDate;
use derive_builder::Builder;
use nulid::Nulid;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::logic::{
    account::OperationContainer,
    category::CategoryList,
    counterparty::CounterpartyList,
    operation::{Operation, OperationFlow, OperationKind},
    search::SearchError,
    utils::HasNulid,
};

// dans operation.rs

#[derive(Debug, Clone)]
pub struct CounterpartyGroup {
    pub id: Nulid,
    pub name: String,
    pub kind: String,
    pub op_count: usize,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub last_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct CategoryGroup {
    pub id: Nulid,
    pub name: String,
    pub op_count: usize,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub last_date: Option<NaiveDate>,
}

/// Struct for search operation
#[derive(Debug, Clone)]
pub struct SearchOperation {
    pub operation: Operation,
    pub balance: Decimal,
}

/// Struct for search operation list
#[derive(Debug, Clone)]
pub struct SearchOperationList {
    pub items: Vec<SearchOperation>,
    pub params: SearchParams,
}

impl SearchOperationList {
    pub fn new(params: SearchParams) -> Self {
        Self {
            items: Vec::new(),
            params,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &SearchOperation> {
        self.items.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    ///Return a SearchOperation or None
    pub fn get_search_operation_by_id(&self, id: Nulid) -> Option<&SearchOperation> {
        self.items.iter().find(|so| so.operation.id == id)
    }

    pub fn active_items(&self) -> impl Iterator<Item = &SearchOperation> {
        self.items.iter().filter(|i| !i.operation.kind.is_structural()) // not the init and the checkpoint
    }

    pub fn is_empty_active(&self) -> bool {
        self.active_items().next().is_none()
    }

    pub fn last_n(&self, n: usize) -> Vec<SearchOperation> {
        let len = self.items.len();
        let start = len.saturating_sub(n);
        self.items[start..].to_vec()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn group_by_counterparty(&self, counterparties: &CounterpartyList) -> Vec<CounterpartyGroup> {
        let mut map: HashMap<Nulid, CounterpartyGroup> = HashMap::new();

        for item in self.active_items() {
            let op = &item.operation;
            let cp_id = match op.context.counterparty_id {
                Some(id) => id,
                None => continue,
            };

            let entry = map.entry(cp_id).or_insert_with(|| {
                let cp = counterparties.get_by_id(&cp_id).ok();
                CounterpartyGroup {
                    id: cp_id,
                    name: cp.map(|c| c.name.clone()).unwrap_or_default(),
                    kind: cp.map(|c| c.kind.as_str().to_string()).unwrap_or_default(),
                    op_count: 0,
                    total_debit: Decimal::ZERO,
                    total_credit: Decimal::ZERO,
                    last_date: None,
                }
            });

            entry.op_count += 1;
            if op.flow.is_debit() {
                entry.total_debit += op.amount;
            } else {
                entry.total_credit += op.amount;
            }
            let d = op.date;
            entry.last_date = Some(match entry.last_date {
                Some(existing) if existing >= d => existing,
                _ => d,
            });
        }

        let mut groups: Vec<CounterpartyGroup> = map.into_values().collect();
        groups.sort_by(|a, b| {
            b.total_debit
                .partial_cmp(&a.total_debit)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        groups
    }

    pub fn group_by_category(&self, categories: &CategoryList) -> Vec<CategoryGroup> {
        let mut map: HashMap<Nulid, CategoryGroup> = HashMap::new();

        for item in self.active_items() {
            let op = &item.operation;
            let cp_id = match op.context.category_id {
                Some(id) => id,
                None => continue,
            };

            let entry = map.entry(cp_id).or_insert_with(|| {
                let cp = categories.get_by_id(&cp_id).ok();
                CategoryGroup {
                    id: cp_id,
                    name: cp.map(|c| c.name.clone()).unwrap_or_default(),
                    op_count: 0,
                    total_debit: Decimal::ZERO,
                    total_credit: Decimal::ZERO,
                    last_date: None,
                }
            });

            entry.op_count += 1;
            if op.flow.is_debit() {
                entry.total_debit += op.amount;
            } else {
                entry.total_credit += op.amount;
            }
            let d = op.date;
            entry.last_date = Some(match entry.last_date {
                Some(existing) if existing >= d => existing,
                _ => d,
            });
        }

        let mut groups: Vec<CategoryGroup> = map.into_values().collect();
        groups.sort_by(|a, b| {
            b.total_debit
                .partial_cmp(&a.total_debit)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        groups
    }
}

impl Default for SearchOperationList {
    fn default() -> Self {
        Self::new(SearchParams::default())
    }
}

impl HasNulid for SearchOperation {
    fn id(&self) -> Nulid {
        self.operation.id
    }
}

#[derive(Debug, Default, Clone, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SearchParams {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub text: Option<String>,
    pub kind: Option<String>,
    pub flow: Option<String>,
    pub counterparty: Option<Nulid>,
    pub category: Option<Nulid>,
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
pub fn search<T: OperationContainer>(container: &T, params: &SearchParams) -> Result<SearchOperationList, SearchError> {
    let ops_map = container.get_operations_with_balance();
    let from = params.from;
    let to = params.to;
    let text = params.text.as_deref();
    let kind = params.kind.as_deref();
    let flow = params.flow.as_deref();
    let counterparty = params.counterparty;
    let category = params.category;
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
            Err(_) => return Ok(SearchOperationList::new(params.clone())),
        },
        None => None,
    };

    let o_kind_filter = match kind {
        Some(s) => match OperationKind::try_from(s) {
            Ok(v) => Some(v),
            Err(_) => return Ok(SearchOperationList::new(params.clone())),
        },
        None => None,
    };

    let mut matched = SearchOperationList::new(params.clone());

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
        // counterparty
        if let Some(c) = counterparty
            && op.context.counterparty_id != Some(c)
        {
            continue;
        }
        // category
        if let Some(g) = category
            && op.context.category_id != Some(g)
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
            SearchOperationList {
                items: matched.last_n(n),
                params: matched.params,
            }
        }
    } else {
        matched
    };

    Ok(result)
}
