// src/logic/account/entry.rs

use crate::logic::{
    account::{
        Account, AccountAnchorsItem, OperationEntry, OperationItem, SearchParams, SummaryEntry,
        search,
    },
    balance::{Balance, BalanceItem},
    counts::Counts,
};

impl Account {
    /// Builds an OperationEntry for the current account using the given search params.
    /// Includes can_be_void computed from the account context.
    pub fn operation_entry(&self, params: &SearchParams) -> Option<OperationEntry> {
        let matched_items = search(self, params).ok().unwrap_or_default();
        let items: Vec<OperationItem> = matched_items
            .iter()
            .map(|item| {
                let mut op_item = OperationItem::from(item);
                op_item.can_be_void = self.can_void(item.operation.id).unwrap_or(false);
                op_item
            })
            .collect();
        Some(OperationEntry {
            counts: Counts::new(&matched_items),
            items,
        })
    }

    /// Build a Summary Entry from search entry and account.
    /// Always returns a value — anchors are account-level, not period-dependent.
    pub fn summary_entry(&self, params: &SearchParams) -> SummaryEntry {
        let matched_items = search(self, params).ok().unwrap_or_default();

        SummaryEntry {
            counts: Counts::new(&matched_items),
            balance: BalanceItem::from(Balance::new(&matched_items)),
            anchors: AccountAnchorsItem::from(&self.anchors),
        }
    }
}
