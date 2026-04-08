// src/dto/category.rs

use rust_decimal::Decimal;

use crate::{
    core::{format_date, format_id, format_optional_date, format_optional_id},
    logic::{
        category::{Category, CategoryList},
        search::CategoryGroup,
    },
};

#[derive(Debug)]
pub struct CategoryItem {
    pub id: String,   // Nulid
    pub name: String, // ex: "Food"
    pub note: Option<String>,
    pub parent_id: Option<String>,
    pub parent_name: Option<String>,
    pub parent_terminated: Option<String>,
    pub terminated: Option<String>,
}

impl CategoryItem {
    pub fn build(category: &Category, categories: &CategoryList) -> Self {
        let parent = category
            .parent_id
            .as_ref()
            .and_then(|pid| categories.get_by_id(pid).ok());
        Self {
            id: format_id(category.id),
            name: category.name.clone(),
            note: category.note.clone(),
            parent_id: format_optional_id(category.parent_id),
            parent_name: parent.map(|p| p.name.clone()),
            parent_terminated: parent.and_then(|p| format_optional_date(p.terminated)),
            terminated: format_optional_date(category.terminated),
        }
    }
}

#[derive(Debug)]
pub struct CategoryCollection {
    pub items: Vec<CategoryItem>,
}

impl CategoryCollection {
    pub fn build(categories: &CategoryList) -> Self {
        let items = categories
            .list
            .iter()
            .map(|c| CategoryItem::build(c, categories))
            .collect();
        Self { items }
    }
}

#[derive(Debug)]
pub struct CategoryStatsItem {
    pub id: Option<String>,
    pub name: String,
    pub op_count: usize,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub average_amount: Decimal,
    pub debit_percentage: Decimal,
    pub credit_percentage: Decimal,
    pub last_date: Option<String>,
}

#[derive(Debug)]
pub struct CategoryStatsCollection {
    pub items: Vec<CategoryStatsItem>,
}

impl CategoryStatsCollection {
    pub fn build(groups: Vec<CategoryGroup>) -> Self {
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
                CategoryStatsItem {
                    id: format_optional_id(g.id),
                    name: g.name,
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
