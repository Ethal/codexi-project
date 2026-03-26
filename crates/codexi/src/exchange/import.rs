// src/exchange/import.rs

use rust_decimal::Decimal;

use crate::core::{
    CoreWarning, parse_date, parse_id, parse_optional_date, parse_optional_id,
    resolve_or_generate_id,
};
use crate::exchange::models::ExchangeAccountOperations;
use crate::exchange::validator::validate_import_operations;
use crate::exchange::{
    ExchangeAccountHeader, ExchangeCurrency, ExchangeCurrencyList, ExchangeError,
    validate_import_account_header, validate_import_currency,
};
use crate::logic::operation::{AccountOperations, Operation};
use crate::logic::{
    account::{Account, AccountAnchors, AccountContext, AccountMeta},
    currency::{Currency, CurrencyList},
};

impl ExchangeAccountHeader {
    /// Single entry point for importing a account (JSON / TOML / CSV)
    pub fn import_data(
        data: &ExchangeAccountHeader,
    ) -> Result<(Account, Vec<CoreWarning>), ExchangeError> {
        let warnings = validate_import_account_header(data)?;
        let account = Self::build_from_export(data)?;
        Ok((account, warnings))
    }

    /// internal build after validation
    fn build_from_export(import: &ExchangeAccountHeader) -> Result<Account, ExchangeError> {
        // carry_forward_balance, terminated_date, current_balance, checkpoints, anchors,
        // ignored on import — recalculated by refresh_anchors() after merge
        Ok(Account {
            id: resolve_or_generate_id(import.id.as_deref()),
            name: import.name.clone(),
            context: AccountContext::try_from(&import.context)?,
            bank_id: parse_optional_id(import.bank_id.as_deref())?, // Bank Id
            currency_id: parse_optional_id(import.currency_id.as_deref())?, // Currency id for the account
            carry_forward_balance: Decimal::ZERO, // for internal calculation
            open_date: parse_date(&import.open_date)?, // Open date of the account,typivcaly the date of the init.
            terminated_date: parse_optional_date(import.terminated_date.as_deref())?, // Close date of the account.
            operations: Vec::new(),
            current_balance: Decimal::ZERO,
            checkpoints: Vec::new(),
            anchors: AccountAnchors::default(),
            meta: AccountMeta::try_from(&import.meta)?,
        })
    }
}

impl ExchangeAccountOperations {
    pub fn import_data(
        data: &ExchangeAccountOperations,
    ) -> Result<(AccountOperations, Vec<CoreWarning>), ExchangeError> {
        let warnings = validate_import_operations(&data)?;
        let account_operations = AccountOperations {
            account_id: parse_id(&data.account_id)?,
            operations: data
                .operations
                .iter()
                .map(Operation::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        };

        Ok((account_operations, warnings))
    }
}

impl ExchangeCurrencyList {
    pub fn import_data(
        data: &ExchangeCurrencyList,
    ) -> Result<(CurrencyList, Vec<CoreWarning>), ExchangeError> {
        let warnings = validate_import_currency(data)?;

        let currencies: Vec<Currency> = data
            .currencies
            .iter()
            .cloned()
            .map(Self::map_currency)
            .collect();

        let currency_list = CurrencyList { currencies };
        Ok((currency_list, warnings))
    }

    /// Mapping strict Export → Domain (without alteration)
    fn map_currency(c: ExchangeCurrency) -> Currency {
        Currency {
            id: resolve_or_generate_id(c.id.as_deref()),
            code: c.code.clone(),
            symbol: c.symbol.clone(),
            decimal_places: c.decimal_places,
            note: c.note.clone(),
        }
    }
}
