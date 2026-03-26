// exchange/exchangeable.rs

// Pure domain trait: no I/O, no FileExchangeError.
// Conversion errors stay within the exchange layer.

use serde::{Deserialize, Serialize};

use crate::{
    core::CoreWarning,
    exchange::{
        ExchangeAccountHeader, ExchangeAccountOperations, ExchangeCurrencyList, ExchangeError,
    },
    logic::{account::Account, currency::CurrencyList, operation::AccountOperations},
};

pub trait ExchangeBase {
    type Warning;
}

pub trait Exchangeable: Sized + ExchangeBase {
    type Exchange: Serialize + for<'de> Deserialize<'de>;

    fn to_exchange(&self) -> Self::Exchange;

    /// Converts an exchange DTO into the domain type.
    /// Returns only ExchangeError — no I/O involved at this level.
    fn from_exchange(data: Self::Exchange) -> Result<(Self, Vec<Self::Warning>), ExchangeError>;

    /// Base filename used when writing to disk (without extension).
    /// Declared here so the format layer can build the path without
    /// knowing the concrete type.
    fn exchange_filename() -> &'static str;
}

impl ExchangeBase for Account {
    type Warning = CoreWarning;
}

impl Exchangeable for Account {
    type Exchange = ExchangeAccountHeader;

    fn to_exchange(&self) -> Self::Exchange {
        ExchangeAccountHeader::export_data(self)
    }

    fn from_exchange(data: Self::Exchange) -> Result<(Self, Vec<Self::Warning>), ExchangeError> {
        ExchangeAccountHeader::import_data(&data)
    }

    fn exchange_filename() -> &'static str {
        "account_header"
    }
}

impl ExchangeBase for AccountOperations {
    type Warning = CoreWarning;
}

impl Exchangeable for AccountOperations {
    type Exchange = ExchangeAccountOperations;

    fn to_exchange(&self) -> Self::Exchange {
        ExchangeAccountOperations::export_data(self)
    }

    fn from_exchange(data: Self::Exchange) -> Result<(Self, Vec<Self::Warning>), ExchangeError> {
        ExchangeAccountOperations::import_data(&data)
    }

    fn exchange_filename() -> &'static str {
        "operations"
    }
}

impl ExchangeBase for CurrencyList {
    type Warning = CoreWarning;
}

impl Exchangeable for CurrencyList {
    type Exchange = ExchangeCurrencyList;

    fn to_exchange(&self) -> Self::Exchange {
        ExchangeCurrencyList::export_data(self)
    }

    fn from_exchange(data: Self::Exchange) -> Result<(Self, Vec<Self::Warning>), ExchangeError> {
        ExchangeCurrencyList::import_data(&data)
    }

    fn exchange_filename() -> &'static str {
        "currencies"
    }
}
