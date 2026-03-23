// src/logic/codexi/entry.rs

use chrono::NaiveDate;
use nulid::Nulid;

use crate::core::{format_date, format_id, format_optional_date, format_optional_text};
use crate::logic::{
    account::{Account, AccountContextItem, SearchEntry, SearchParams, search},
    balance::{Balance, BalanceItem},
    codexi::{
        AccountEntry, AccountItem, Codexi, OperationDetailItem, StatementEntry, StatementItem,
    },
    counts::Counts,
};

impl Codexi {
    /// Builds a StatementEntry for the given account, enriched with bank and currency
    /// names from the Codexi context. Returns None if the account is not found.
    pub fn statement_entry(
        &self,
        account_id: &Nulid,
        params: &SearchParams,
    ) -> Option<StatementEntry> {
        let mut entry = StatementEntry::default();

        let (matched_items, account_number, account_name, bank_id, currency_id) = {
            let account = self.get_account_by_id(account_id).ok()?;
            let matched_items = search(account, params).ok()?;
            (
                matched_items,
                format_id(account.id),
                account.name.clone(),
                account.bank_id,
                account.currency_id,
            )
        };

        entry.account_name = account_name;
        entry.account_number = account_number;

        if let Some(id) = bank_id
            && let Ok(bank) = self.banks.get_by_id(&id)
        {
            entry.account_bank = bank.name.clone();
        }
        if let Some(id) = currency_id
            && let Ok(currency) = self.currencies.get_by_id(&id)
        {
            entry.account_currency = currency.code.clone();
        }

        let (date_min, date_max) = find_date_range(&matched_items)
            .map(|(min, max)| (format_date(min), format_date(max)))
            .unwrap_or(("N/A".into(), "N/A".into()));

        entry.date_min = date_min;
        entry.date_max = date_max;
        entry.balance = BalanceItem::from(Balance::new(&matched_items));
        entry.counts = Counts::new(&matched_items);
        entry.items = matched_items.iter().map(StatementItem::from).collect();

        Some(entry)
    }

    pub fn account_entry(&self) -> AccountEntry {
        let items = self
            .accounts
            .iter()
            .map(|acc| self.account_item(acc))
            .collect();

        AccountEntry { items }
    }

    pub fn account_item(&self, acc: &Account) -> AccountItem {
        let mut item = AccountItem::default();
        item.id = format_id(acc.id);
        item.name = acc.name.clone();
        item.current = acc.id == self.current_account;
        item.close = acc.terminated_date.is_some();
        if let Some(id) = acc.bank_id
            && let Ok(bank) = self.banks.get_by_id(&id)
        {
            item.bank = bank.name.clone();
        }
        if let Some(id) = acc.currency_id
            && let Ok(currency) = self.currencies.get_by_id(&id)
        {
            item.currency = currency.code.clone();
        }
        item.context = AccountContextItem::from(&acc.context);
        item
    }

    /// Builds an OperationDetailItem for a given operation in the current account.
    /// Resolves currency, category names from the Codexi context.
    /// Returns None if the account or operation is not found.
    pub fn operation_detail(&self, op_id: &Nulid) -> Option<OperationDetailItem> {
        let account = self.get_current_account().ok()?;
        let op = account.get_operation_by_id(*op_id)?;

        // Resolve balance from search — op.balance is always up to date
        let balance = op.balance;

        // Resolve currency
        let currency = op
            .context
            .currency_id
            .and_then(|id| self.currencies.currency_code_by_id(&id))
            .unwrap_or("—".into());

        // Resolve category
        let category = op
            .context
            .category_id
            .and_then(|id| self.categories.category_name_by_id(&id))
            .unwrap_or("—".into());

        // can_be_void
        let can_be_void = account.can_void(*op_id).unwrap_or(false);

        Some(OperationDetailItem {
            // ── Identity ─────────────────────────────────────
            id: format_id(op.id),
            date: format_date(op.date),
            kind: op.kind.as_str().to_string(),
            flow: op.flow.as_str().to_string(),
            amount: op.amount,
            balance,
            description: op.description.clone(),
            can_be_void,

            // ── Links ─────────────────────────────────────────
            void_of: op.links.void_of.map(format_id),
            void_by: op.links.void_by.map(format_id),
            transfer_id: op.links.transfer_id.map(format_id),
            transfer_account_id: op.links.transfer_account_id.map(format_id),

            // ── Context ───────────────────────────────────────
            currency,
            exchange_rate: op.context.exchange_rate,
            category,
            payee: format_optional_text(op.context.payee.as_deref()),
            reconciled: format_optional_date(op.context.reconciled),

            // ── Meta ──────────────────────────────────────────
            tags: op
                .meta
                .tags
                .as_ref()
                .map(|t| t.join(", "))
                .unwrap_or("—".into()),
            note: format_optional_text(op.meta.note.as_deref()),
            attachment: op
                .meta
                .attachment_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or("—".into()),
        })
    }
}

/// Returns the min and max operation dates from a search result.
fn find_date_range(items: &SearchEntry) -> Option<(NaiveDate, NaiveDate)> {
    let mut iter = items.iter().map(|i| i.operation.date);
    let first = iter.next()?;
    let (min, max) = iter.fold((first, first), |(min, max), d| (min.min(d), max.max(d)));
    Some((min, max))
}
