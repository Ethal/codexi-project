// src/dto/bank.rs

use rust_decimal::Decimal;

use crate::{
    core::{format_date, format_id, format_optional_date, format_optional_id},
    logic::{
        counterparty::{Counterparty, CounterpartyList},
        search::CounterpartyGroup,
    },
};

#[derive(Debug)]
pub struct CounterpartyItem {
    pub id: String, // Nulid
    pub name: String,
    pub kind: String,
    pub terminated: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug)]
pub struct CounterpartyCollection {
    pub items: Vec<CounterpartyItem>,
}

impl From<&Counterparty> for CounterpartyItem {
    fn from(cp: &Counterparty) -> Self {
        Self {
            id: format_id(cp.id),
            name: cp.name.clone(),
            kind: cp.kind.as_str().to_string(),
            terminated: format_optional_date(cp.terminated),
            note: cp.note.clone(),
        }
    }
}

impl CounterpartyCollection {
    pub fn build(cps: &CounterpartyList) -> Self {
        let items: Vec<CounterpartyItem> = cps.list.iter().map(CounterpartyItem::from).collect();
        Self { items }
    }
}

#[derive(Debug)]
pub struct CounterpartyStatsItem {
    pub id: Option<String>,
    pub name: String,
    pub kind: String,
    pub op_count: usize,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub average_amount: Decimal,
    pub debit_percentage: Decimal,
    pub credit_percentage: Decimal,
    pub last_date: Option<String>,
}

#[derive(Debug)]
pub struct CounterpartyStatsCollection {
    pub items: Vec<CounterpartyStatsItem>,
}

impl CounterpartyStatsCollection {
    pub fn build(groups: Vec<CounterpartyGroup>) -> Self {
        let grand_total_debit: Decimal = groups.iter().map(|g| g.total_debit).sum();
        let grand_total_credit: Decimal = groups.iter().map(|g| g.total_credit).sum();

        let items = groups
            .into_iter()
            .map(|g| {
                let total = g.total_debit + g.total_credit;
                let average_amount = if g.op_count > 0 {
                    total / Decimal::from(g.op_count)
                } else {
                    Decimal::ZERO
                };
                let debit_percentage = if grand_total_debit > Decimal::ZERO {
                    (g.total_debit / grand_total_debit) * Decimal::ONE_HUNDRED
                } else {
                    Decimal::ZERO
                };
                let credit_percentage = if grand_total_credit > Decimal::ZERO {
                    (g.total_credit / grand_total_credit) * Decimal::ONE_HUNDRED
                } else {
                    Decimal::ZERO
                };
                CounterpartyStatsItem {
                    id: format_optional_id(g.id),
                    name: g.name,
                    kind: g.kind,
                    op_count: g.op_count,
                    total_debit: g.total_debit,
                    total_credit: g.total_credit,
                    average_amount,
                    debit_percentage,
                    credit_percentage,
                    last_date: g.last_date.map(format_date),
                }
            })
            .collect();

        Self { items }
    }
}
