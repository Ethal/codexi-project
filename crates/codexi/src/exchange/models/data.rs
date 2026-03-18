// src/exchange/models/data.rs

use chrono::NaiveDate;
use nulid::Nulid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    core::serde_nulid,
    exchange::{ExchangeCheckpointRef, ExchangeOperation},
    logic::account::{AccountAnchors, AccountContext, AccountMeta},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeData {
    #[serde(default)]
    pub version: u16,
    #[serde(default, with = "serde_nulid")]
    pub id: Nulid, // Id
    #[serde(default)]
    pub name: String, // Account name
    #[serde(default)]
    pub context: AccountContext,
    #[serde(default, with = "serde_nulid::option")]
    pub bank_id: Option<Nulid>, // Nulid of the Bank
    #[serde(default, with = "serde_nulid::option")]
    pub currency_id: Option<Nulid>, // Main currency id for the account
    #[serde(default)]
    pub carry_forward_balance: Decimal, // for internal calculation
    #[serde(default)]
    pub open_date: NaiveDate, // Open date of the account,typivcaly the date of the init.
    #[serde(default)]
    pub terminated_date: Option<NaiveDate>, // Close date of the account.
    #[serde(default)]
    pub operations: Vec<ExchangeOperation>, // Operation list
    #[serde(default)]
    pub checkpoints: Vec<ExchangeCheckpointRef>,
    #[serde(default)]
    pub current_balance: Decimal,
    #[serde(default)]
    pub anchors: AccountAnchors,
    #[serde(default)]
    pub meta: AccountMeta,
}
