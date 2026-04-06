// src/exchange/models/account.rs

use serde::{Deserialize, Serialize};

use crate::{
    core::{
        default_zero, format_date, format_decimal, format_id, format_optional_date, format_path, parse_decimal,
        parse_optional_date,
    },
    logic::account::{
        AccountAnchors, AccountContext, AccountError, AccountMeta, AccountType, CheckpointRef, LastAnchor,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeAccountHeader {
    pub version: u16,
    #[serde(default)]
    pub id: Option<String>, // Id
    pub name: String, // Account name
    pub context: ExchangeAccountContext,
    #[serde(default)]
    pub bank_id: Option<String>, // Nulid of the Bank
    #[serde(default)]
    pub currency_id: Option<String>, // Main currency id for the account
    #[serde(default = "default_zero")]
    pub carry_forward_balance: String, // for internal calculation
    pub open_date: String, // Open date of the account,typivcaly the date of the init.
    #[serde(default)]
    pub terminated_date: Option<String>, // Close date of the account.
    #[serde(default)]
    pub checkpoints: Vec<ExchangeCheckpointRef>,
    #[serde(default = "default_zero")]
    pub current_balance: String,
    #[serde(default)]
    pub anchors: ExchangeAccountAnchors,
    #[serde(default)]
    pub meta: ExchangeAccountMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeCheckpointRef {
    pub checkpoint_date: String,
    pub checkpoint_balance: String,
    pub archive_file: String, // "<ID>_<APP_NAME>_<YYY-MM-DD>.cld"
}

impl From<&CheckpointRef> for ExchangeCheckpointRef {
    fn from(checkpoint: &CheckpointRef) -> Self {
        Self {
            checkpoint_date: format_date(checkpoint.checkpoint_date),
            checkpoint_balance: format_decimal(checkpoint.checkpoint_balance),
            archive_file: format_path(&checkpoint.archive_file),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeLastAnchor {
    pub id: String,
    pub date: String,
}

impl From<&LastAnchor> for ExchangeLastAnchor {
    fn from(last_anchor: &LastAnchor) -> Self {
        Self {
            id: format_id(last_anchor.id),
            date: format_date(last_anchor.date),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeAccountAnchors {
    pub last_regular: Option<ExchangeLastAnchor>,
    pub last_init: Option<ExchangeLastAnchor>,
    pub last_adjust: Option<ExchangeLastAnchor>,
    pub last_void: Option<ExchangeLastAnchor>,
    pub last_checkpoint: Option<ExchangeLastAnchor>,
}

impl From<&AccountAnchors> for ExchangeAccountAnchors {
    fn from(anchor: &AccountAnchors) -> Self {
        Self {
            last_regular: anchor.last_regular.as_ref().map(ExchangeLastAnchor::from),
            last_init: anchor.last_init.as_ref().map(ExchangeLastAnchor::from),
            last_adjust: anchor.last_adjust.as_ref().map(ExchangeLastAnchor::from),
            last_void: anchor.last_void.as_ref().map(ExchangeLastAnchor::from),
            last_checkpoint: anchor.last_checkpoint.as_ref().map(ExchangeLastAnchor::from),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeAccountMeta {
    pub iban: Option<String>,
    pub color: Option<String>,
    pub display_order: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub note: Option<String>,
}

impl From<&AccountMeta> for ExchangeAccountMeta {
    fn from(meta: &AccountMeta) -> Self {
        Self {
            iban: meta.iban.clone(),
            color: meta.color.clone(),
            display_order: meta.display_order,
            tags: meta.tags.clone(),
            note: meta.note.clone(),
        }
    }
}

impl TryFrom<&ExchangeAccountMeta> for AccountMeta {
    type Error = AccountError;
    fn try_from(meta: &ExchangeAccountMeta) -> Result<Self, Self::Error> {
        Ok(Self {
            iban: meta.iban.clone(),
            color: meta.color.clone(),
            display_order: meta.display_order,
            tags: meta.tags.clone(),
            note: meta.note.clone(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeAccountContext {
    pub account_type: String,
    #[serde(default = "default_zero")]
    pub overdraft_limit: String,
    #[serde(default = "default_zero")]
    pub min_balance: String,
    #[serde(default)]
    pub max_monthly_transactions: Option<u32>,
    #[serde(default)]
    pub deposit_locked_until: Option<String>,
    #[serde(default)]
    pub allows_interest: bool,
    #[serde(default)]
    pub allows_joint_signers: bool,
}

impl From<&AccountContext> for ExchangeAccountContext {
    fn from(context: &AccountContext) -> Self {
        Self {
            account_type: context.account_type.as_str().to_string(),
            overdraft_limit: format_decimal(context.overdraft_limit),
            min_balance: format_decimal(context.min_balance),
            max_monthly_transactions: context.max_monthly_transactions,
            deposit_locked_until: format_optional_date(context.deposit_locked_until),
            allows_interest: context.allows_interest,
            allows_joint_signers: context.allows_joint_signers,
        }
    }
}

impl TryFrom<&ExchangeAccountContext> for AccountContext {
    type Error = AccountError;
    fn try_from(ctx: &ExchangeAccountContext) -> Result<Self, Self::Error> {
        Ok(Self {
            account_type: AccountType::try_from_str(&ctx.account_type)?,
            overdraft_limit: parse_decimal(&ctx.overdraft_limit, "overdraft_limit")?,
            min_balance: parse_decimal(&ctx.min_balance, "min_balance")?,
            max_monthly_transactions: ctx.max_monthly_transactions,
            deposit_locked_until: parse_optional_date(ctx.deposit_locked_until.as_deref())?,
            allows_interest: ctx.allows_interest,
            allows_joint_signers: ctx.allows_joint_signers,
        })
    }
}
