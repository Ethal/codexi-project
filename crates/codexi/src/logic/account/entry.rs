// src/logic/account/view.rs

use chrono::NaiveDate;
use nulid::Nulid;

use crate::core::{format_date, format_id};
use crate::logic::{
    account::{
        Account, OperationEntry, OperationItem, SearchEntry, SearchItem, SearchParams,
        StatementEntry, StatementItem, error::SearchError, search,
    },
    balance::{Balance, BalanceItem},
    codexi::Codexi,
    counts::Counts,
};

impl OperationEntry {
    pub fn operation_entry(
        account: &Account,
        params: &SearchParams,
    ) -> Result<OperationEntry, SearchError> {
        let matched_items = search(account, params)?;

        let items: Vec<OperationItem> = matched_items
            .iter()
            .map(|item| Self::operation_item(account, item))
            .collect();

        Ok(OperationEntry {
            operation_count: items.len().to_string(),
            items,
        })
    }
    fn operation_item(account: &Account, item: &SearchItem) -> OperationItem {
        let mut view = OperationItem::from(item);
        view.can_be_void = account.can_void(item.operation.id).unwrap_or(false);
        view
    }
}

impl StatementEntry {
    pub fn statement_entry(
        codexi: &Codexi,
        account_id: &Nulid,
        params: &SearchParams,
    ) -> Option<StatementEntry> {
        let mut statement_entry = StatementEntry::default();

        let (matched_items, account_number, account_name, bank_id, currency_id) = {
            let account = codexi.get_account_by_id(account_id).ok()?;
            let matched_items = search(account, params).ok()?;
            (
                matched_items,
                format_id(account.id),
                account.name.clone(),
                account.bank_id,
                account.currency_id,
            )
        };

        statement_entry.account_name = account_name;
        statement_entry.account_number = account_number;

        if let Some(id) = bank_id
            && let Ok(bank) = codexi.banks.get_by_id(&id)
        {
            statement_entry.account_bank = bank.name.clone();
        }
        if let Some(id) = currency_id
            && let Ok(currency) = codexi.currencies.get_by_id(&id)
        {
            statement_entry.account_currency = currency.code.clone();
        }

        let (date_min, date_max) = Self::find_date_range(&matched_items)
            .map(|(min, max)| (format_date(min), format_date(max)))
            .unwrap_or(("N/A".into(), "N/A".into()));

        statement_entry.date_min = date_min;
        statement_entry.date_max = date_max;
        statement_entry.balance = BalanceItem::from(Balance::balance(&matched_items));
        statement_entry.counts = Counts::counts(&matched_items);
        statement_entry.items = matched_items.iter().map(Self::statement_item).collect();

        Some(statement_entry)
    }

    fn statement_item(item: &SearchItem) -> StatementItem {
        StatementItem::from(item)
    }

    fn find_date_range(items: &SearchEntry) -> Option<(NaiveDate, NaiveDate)> {
        let mut iter = items.iter().map(|i| i.operation.date);

        let first = iter.next()?;
        let (min, max) = iter.fold((first, first), |(min, max), d| (min.min(d), max.max(d)));

        Some((min, max))
    }
}
