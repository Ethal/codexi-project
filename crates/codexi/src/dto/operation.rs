// src/dto/operation.rs

// Detailed dto of a colection/single operation — all referenced fields resolved

use rust_decimal::Decimal;

use crate::{
    core::{format_date, format_id, format_optional_date, format_optional_id, format_optional_path},
    logic::{
        account::Account,
        codexi::Codexi,
        counts::Counts,
        operation::OperationFlow,
        search::{CounterpartyCategoryGroup, SearchOperation, SearchOperationList, SearchParams},
    },
};

#[derive(Debug)]
pub struct SearchOperationItem {
    // ── Identity ─────────────────────────────────────────────
    pub id: String,
    pub date: String,
    pub kind: String,
    pub flow: String,
    pub amount: Decimal,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
    pub description: String,
    pub can_be_void: bool,

    // ── Links ─────────────────────────────────────────────────
    pub void_of: Option<String>,
    pub void_by: Option<String>,
    pub transfer_id: Option<String>,
    pub transfer_account_id: Option<String>,

    // ── Context — resolved ────────────────────────────────────
    pub currency: Option<String>, // resolved from currency_id
    pub exchange_rate: Decimal,
    pub category: Option<String>, // resolved from category_id
    pub payee: Option<String>,
    pub reconciled: Option<String>,

    // ── Meta ──────────────────────────────────────────────────
    pub tags: Option<String>,
    pub note: Option<String>,
    pub attachment: Option<String>,
}

//let items = search(self, params).ok().unwrap_or_default();

impl SearchOperationItem {
    pub fn build(codexi: &Codexi, account: &Account, s_op: &SearchOperation) -> Self {
        // calculated balance from search
        let balance = s_op.balance;

        // Resolve currency
        let currency = s_op
            .operation
            .context
            .currency_id
            .and_then(|id| codexi.currencies.currency_code_by_id(&id));

        // Resolve category
        let category = s_op
            .operation
            .context
            .category_id
            .and_then(|id| codexi.categories.get_name_by_id(&id));

        // can_be_void
        let can_be_void = account.can_void(s_op.operation.id);

        let (debit, credit) = match s_op.operation.flow {
            OperationFlow::Debit => (s_op.operation.amount, Decimal::ZERO),
            OperationFlow::Credit => (Decimal::ZERO, s_op.operation.amount),
            _ => (Decimal::ZERO, Decimal::ZERO),
        };

        Self {
            // ── Identity ─────────────────────────────────────
            id: format_id(s_op.operation.id),
            date: format_date(s_op.operation.date),
            kind: s_op.operation.kind.as_str().to_string(),
            flow: s_op.operation.flow.as_str().to_string(),
            amount: s_op.operation.amount,
            debit,
            credit,
            balance,
            description: s_op.operation.description.clone(),
            can_be_void,

            // ── Links ─────────────────────────────────────────
            void_of: format_optional_id(s_op.operation.links.void_of),
            void_by: format_optional_id(s_op.operation.links.void_by),
            transfer_id: format_optional_id(s_op.operation.links.transfer_id),
            transfer_account_id: format_optional_id(s_op.operation.links.transfer_account_id),

            // ── Context ───────────────────────────────────────
            currency,
            exchange_rate: s_op.operation.context.exchange_rate,
            category,
            payee: s_op.operation.context.payee.clone(),
            reconciled: format_optional_date(s_op.operation.context.reconciled),

            // ── Meta ──────────────────────────────────────────
            tags: s_op.operation.meta.tags.clone().map(|t| t.join(", ")),
            note: s_op.operation.meta.note.clone(),
            attachment: format_optional_path(s_op.operation.meta.attachment_path.as_deref()),
        }
    }
}

#[derive(Debug)]
pub struct SearchOperationCollection {
    pub params: SearchParams,
    pub items: Vec<SearchOperationItem>,
    pub counts: Counts, // add from SearchOperationList
}

impl SearchOperationCollection {
    /// Builds an SearchOperationCollection for an account using SearchOperation.
    /// Count and search parmeters added.
    pub fn build(codexi: &Codexi, account: &Account, s_ops: &SearchOperationList) -> Self {
        let items = s_ops
            .iter()
            .map(|s_op| SearchOperationItem::build(codexi, account, s_op))
            .collect();

        Self {
            counts: Counts::new(s_ops),
            params: s_ops.params.clone(),
            items,
        }
    }
}

/*==============================================================================*/
/*              TREE COUNTERPART / CATEGORY / OPERATION                         */
/*==============================================================================*/

#[derive(Debug)]
pub struct OperationLeaf {
    pub id: String,
    pub date: String,
    pub flow: String,
    pub amount: Decimal,
    pub description: String,
}

#[derive(Debug)]
pub struct CategoryNode {
    pub id: Option<String>,
    pub name: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub operations: Vec<OperationLeaf>,
}

#[derive(Debug)]
pub struct CounterpartyNode {
    pub id: Option<String>,
    pub name: String,
    pub kind: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub categories: Vec<CategoryNode>,
}

#[derive(Debug)]
pub struct CounterpartyTreeCollection {
    pub nodes: Vec<CounterpartyNode>,
}

impl CounterpartyTreeCollection {
    pub fn build(groups: Vec<CounterpartyCategoryGroup>) -> Self {
        let nodes = groups
            .into_iter()
            .map(|cp| {
                let categories = cp
                    .categories
                    .into_iter()
                    .map(|cg| {
                        let operations = cg
                            .operations
                            .into_iter()
                            .map(|so| OperationLeaf {
                                id: format_id(so.operation.id),
                                date: format_date(so.operation.date),
                                flow: so.operation.flow.as_str().to_string(),
                                amount: so.operation.amount,
                                description: so.operation.description.clone(),
                            })
                            .collect();
                        CategoryNode {
                            id: format_optional_id(cg.id),
                            name: cg.name,
                            total_debit: cg.total_debit,
                            total_credit: cg.total_credit,
                            operations,
                        }
                    })
                    .collect();
                CounterpartyNode {
                    id: format_optional_id(cp.id),
                    name: cp.name,
                    kind: cp.kind,
                    total_debit: cp.total_debit,
                    total_credit: cp.total_credit,
                    categories,
                }
            })
            .collect();
        Self { nodes }
    }
}
