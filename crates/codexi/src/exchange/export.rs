// src/exchange/export.rs

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;
use crate::core::{
    format_date, format_decimal, format_id, format_optional_date, format_optional_id,
};
use crate::exchange::{
    ExchangeAccountAnchors, ExchangeAccountContext, ExchangeAccountHeader, ExchangeAccountMeta,
    ExchangeAccountOperations, ExchangeCheckpointRef, ExchangeCounterparty,
    ExchangeCounterpartyList, ExchangeCurrency, ExchangeCurrencyList, ExchangeOperation,
};
use crate::logic::counterparty::CounterpartyList;
use crate::logic::operation::AccountOperations;
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

impl ExchangeAccountOperations {
    /// Single entry point for exporting the operation list from an account(JSON / TOML / CSV)
    pub fn export_data(export: &AccountOperations) -> ExchangeAccountOperations {
        ExchangeAccountOperations {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            account_id: format_id(export.account_id),
            operations: export
                .operations
                .iter()
                .map(ExchangeOperation::from)
                .collect(),
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

impl ExchangeCounterpartyList {
    /// Single entry point for exporting the currency list (JSON / TOML / CSV)
    pub fn export_data(export: &CounterpartyList) -> ExchangeCounterpartyList {
        ExchangeCounterpartyList {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            counterparties: export
                .counterparties
                .iter()
                .map(ExchangeCounterparty::from)
                .collect(),
        }
    }
}
