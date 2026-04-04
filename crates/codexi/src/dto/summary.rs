// src/dto/report.rs

use crate::{
    core::format_date,
    dto::BalanceItem,
    logic::{
        account::{Account, AccountAnchors},
        balance::Balance,
        counts::Counts,
        search::SearchOperationList,
    },
};

/*---------------------- ACCOUNT ANCHORS ITEM ---------------------*/

#[derive(Debug, Default, Clone)]
pub struct AccountAnchorsItem {
    pub last_regular: Option<String>,
    pub last_init: Option<String>,
    pub last_adjust: Option<String>,
    pub last_void: Option<String>,
    pub last_checkpoint: Option<String>,
}

impl From<&AccountAnchors> for AccountAnchorsItem {
    fn from(a: &AccountAnchors) -> Self {
        Self {
            last_regular: a.last_regular.clone().map(|la| format_date(la.date)),
            last_init: a.last_init.clone().map(|la| format_date(la.date)),
            last_adjust: a.last_adjust.clone().map(|la| format_date(la.date)),
            last_void: a.last_void.clone().map(|la| format_date(la.date)),
            last_checkpoint: a.last_checkpoint.clone().map(|la| format_date(la.date)),
        }
    }
}

/*-------------------------- SUMMARY ENTRY -------------------------*/

#[derive(Debug, Default, Clone)]
pub struct SummaryCollection {
    pub counts: Counts,
    pub balance: BalanceItem,
    pub anchors: AccountAnchorsItem,
}

/// Build a Summary Entry from search collection and account.
/// Always returns a value — anchors are account-level, not period-dependent.
impl SummaryCollection {
    pub fn summary_entry(account: &Account, s_ops: &SearchOperationList) -> SummaryCollection {
        SummaryCollection {
            counts: Counts::new(s_ops),
            balance: BalanceItem::from(Balance::build(s_ops)),
            anchors: AccountAnchorsItem::from(&account.anchors),
        }
    }
}
