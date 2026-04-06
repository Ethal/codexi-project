// src/exchange/export.rs

use crate::CODEXI_EXCHANGE_FORMAT_VERSION;
use crate::core::{format_date, format_decimal, format_id, format_optional_date, format_optional_id};

use crate::{
    exchange::{
        ExchangeAccountAnchors, ExchangeAccountContext, ExchangeAccountHeader, ExchangeAccountMeta,
        ExchangeAccountOperations, ExchangeCategory, ExchangeCategoryList, ExchangeCheckpointRef, ExchangeCounterparty,
        ExchangeCounterpartyList, ExchangeCurrency, ExchangeCurrencyList, ExchangeOperation,
    },
    logic::{
        account::Account, category::CategoryList, counterparty::CounterpartyList, currency::CurrencyList,
        operation::AccountOperations,
    },
};

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
            checkpoints: export.checkpoints.iter().map(ExchangeCheckpointRef::from).collect(),
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
            operations: export.operations.iter().map(ExchangeOperation::from).collect(),
        }
    }
}

impl ExchangeCurrencyList {
    /// Single entry point for exporting the currency list (JSON / TOML / CSV)
    pub fn export_data(export: &CurrencyList) -> ExchangeCurrencyList {
        ExchangeCurrencyList {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            currencies: export.currencies.iter().map(ExchangeCurrency::from).collect(),
        }
    }
}

impl ExchangeCounterpartyList {
    /// Single entry point for exporting the currency list (JSON / TOML / CSV)
    pub fn export_data(export: &CounterpartyList) -> ExchangeCounterpartyList {
        ExchangeCounterpartyList {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            list: export.list.iter().map(ExchangeCounterparty::from).collect(),
        }
    }
}

impl ExchangeCategoryList {
    /// Single entry point for exporting the currency list (JSON / TOML / CSV)
    pub fn export_data(export: &CategoryList) -> ExchangeCategoryList {
        ExchangeCategoryList {
            version: CODEXI_EXCHANGE_FORMAT_VERSION,
            list: export.list.iter().map(ExchangeCategory::from).collect(),
        }
    }
}
