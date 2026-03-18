// src/exchange/export.rs

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;
use crate::exchange::{ExchangeCheckpointRef, ExchangeData, ExchangeOperation};
use crate::logic::account::Account;

impl ExchangeData {
    /// Single entry point for exporting a codexi (JSON / TOML / CSV)
    pub fn export_data(export: &Account) -> ExchangeData {
        ExchangeData {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            id: export.id,
            name: export.name.clone(),
            context: export.context.clone(),
            bank_id: export.bank_id,                             // Bank Id
            currency_id: export.currency_id,                     // Currency id for the account
            carry_forward_balance: export.carry_forward_balance, // for internal calculation
            open_date: export.open_date, // Open date of the account,typivcaly the date of the init.
            terminated_date: export.terminated_date, // Close date of the account.
            current_balance: export.current_balance,
            checkpoints: export
                .checkpoints
                .iter()
                .map(ExchangeCheckpointRef::from)
                .collect(),
            anchors: export.anchors.clone(),
            meta: export.meta.clone(),
            operations: export
                .operations
                .iter()
                .map(ExchangeOperation::from)
                .collect(),
        }
    }
}
