// src/exchange/export.rs

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;
use crate::core::{
    format_date, format_decimal, format_id, format_optional_date, format_optional_id,
};
use crate::exchange::models::ExchangeAccountMeta;
use crate::exchange::{
    ExchangeAccountAnchors, ExchangeAccountContext, ExchangeAccountHeader, ExchangeCheckpointRef,
    ExchangeCurrency, ExchangeCurrencyList,
};
use crate::logic::{account::Account, currency::CurrencyList};

impl ExchangeAccountHeader {
    /// Single entry point for exporting the account heder(JSON / TOML / CSV)
    pub fn export_data(export: &Account) -> ExchangeAccountHeader {
        ExchangeAccountHeader {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            id: Some(format_id(export.id)),
            name: export.name.clone(),
            context: ExchangeAccountContext::from(&export.context),
            bank_id: format_optional_id(export.bank_id),
            currency_id: format_optional_id(export.currency_id),
            carry_forward_balance: format_decimal(export.carry_forward_balance),
            open_date: format_date(export.open_date),
            terminated_date: format_optional_date(export.terminated_date),
            current_balance: format_decimal(export.current_balance),
            checkpoints: export
                .checkpoints
                .iter()
                .map(ExchangeCheckpointRef::from)
                .collect(),
            anchors: ExchangeAccountAnchors::from(&export.anchors),
            meta: ExchangeAccountMeta::from(&export.meta),
        }
    }
}

impl ExchangeCurrencyList {
    /// Single entry point for exporting the currency list (JSON / TOML / CSV)
    pub fn export_data(export: &CurrencyList) -> ExchangeCurrencyList {
        ExchangeCurrencyList {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            currencies: export
                .currencies
                .iter()
                .map(ExchangeCurrency::from)
                .collect(),
        }
    }
}
