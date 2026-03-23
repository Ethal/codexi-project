# Changelog
All notable changes to this project will be documented in this file.
---

## [0.2.0] — 2026-03-23
> ⚠️ No automatic migration — use `admin export-script` from previous release before update to this release to rebuild your data.

### Added
- **AccountContext** — configurable parameters per account instance (overdraft limit,
  minimum balance, monthly transaction quota, deposit lock date, interest, joint signers).
  Each account starts from type defaults and can be individually adjusted.
- **CompliancePolicy** — per-account-type business rules validation (overdraft, minimum
  balance, monthly quota, deposit maturity lock). Each account type has its own policy
  implementation (Current, Saving, Joint, Deposit, Business, Student). Values are read
  from `AccountContext` — never hardcoded.
- **LifecyclePolicy** — account lifecycle rules: account type is immutable once operations
  exist; close date validated against today, `open_date`, and last operation date.
- **TemporalPolicy** — renamed and clarified from the previous `FinancialPolicy`.
  Handles chronological and structural rules on operations (date ordering, period locking,
  init/void/checkpoint sequencing). Orthogonal to `CompliancePolicy`.
- **`LastAnchor` struct** — replaces bare `Option<NaiveDate>` in `AccountAnchors`.
  Each anchor now stores both `date` and `id` (Nulid), enabling precise same-day
  ordering for void and lock decisions. Applied uniformly to all anchor types
  (`last_init`, `last_adjust`, `last_checkpoint`, `last_void`, `last_regular`).
- **Generic ID resolution (`resolve_id`)** — reusable mechanism to resolve full or short
  IDs (Nulid) across entities (operations, accounts, etc.). Supports suffix-based lookup
  with configurable minimum length (`MIN_SHORT_LEN`).
- **Traits `HasNulid` and `ResolveError`** — enable generic ID resolution by decoupling
  entity identification and error handling from business logic. Allows reuse across
  multiple domains (Account, Bank, Currency, Category, ...).
- **Transfer between accounts** — creates two linked `Regular::Transfer` operations
  (Debit on source, Credit on destination). Always operates from the current account.
  Exchange rate is calculated automatically as `amount_to / amount_from` — net effective
  rate including all fees, no manual input required.
- **Cross-currency transfer** — supports transfers between accounts with different
  currencies (e.g. EUR → IDR). The net exchange rate reflects the real cost of the
  transfer, fees included.
- **Transfer void** — voiding a transfer operation automatically voids both linked
  operations atomically. Uses the existing `void` command transparently — no separate
  command needed. Void is rejected if the twin operation is archived.
- **Cross-linked transfer references** — each transfer operation carries `transfer_id`
  (twin operation) and `transfer_account_id` (twin account) in `OperationLinks`,
  following the same symmetric pattern as `void_of`/`void_by`.
- **Short ID support in CLI commands** — entities can now be referenced using the last
  characters of their ID instead of the full 26-character Nulid.
- **Import validation hardened** — `validate_import` now rejects negative or zero amounts
  (`InvalidAmount`) and broken transfer links (`BrokenTransferLink`). Cross-account
  transfer references produce a warning rather than an error, as the twin operation
  belongs to another account.
- **Anchors always recalculated on import** — imported `AccountAnchors` are discarded
  and rebuilt from scratch via `refresh_anchors()`. Prevents stale or corrupted anchor
  data from silently propagating.
- **`OperationFlow::apply`** — new helper method eliminating duplicated `match flow`
  patterns across `rebuild_balances_from`, `balance_at`, and related balance calculations.
- **`OperationDetailItem`** — new rich DTO for single operation detail view. All
  referenced fields resolved (currency name, category name, transfer links). Built by
  `Codexi::operation_detail()`.
- **DTO / Entry layer refactored** — clean separation across all domains:
  `dto.rs` holds structures and `From` conversions only; `entry.rs` holds construction
  logic on `impl Account` or `impl Codexi`. `StatementEntry` moved to `codexi/` layer
  as it requires bank and currency resolution from `Codexi`.
- **`AccountAnchorsItem` and `SummaryEntry`** moved to `account/dto.rs` — consistent
  with the DTO pattern, away from `reports.rs`.
- **`StatsEntry` analytics** moved to `impl Account::stats_entry()` in `reports.rs` —
  clearly separated as analytical computation, not a simple DTO conversion.
- Command `account set-context` to update the configurable parameters of the current account.
- Command `account context` to view the context of the current account.
- Command `account create` now accepts an optional account type argument
  (Current, Joint, Saving, Deposit, Business, Student). Defaults to Current.
- Command `transfer <DATE> <AMOUNT_FROM> <AMOUNT_TO> <ACCOUNT_ID_TO> [DESCRIPTION]`
- Command `void` now handles both regular operations and transfer operations transparently.
- Command `operation view <ID> [--raw]` — detailed single operation view with resolved
  fields, color-coded flow, `[can void]` indicator, and optional raw debug output.

### Changed
- Command `account list` now shows the account type.
- Command `account -set-currency [--update-operation]`, option --update-operation to update the existing operation to the account currency.
- Refactored ID resolution logic — moved from account-specific implementation to a
  generic utility (`resolve_id`), improving consistency and reducing duplication.
- Commands such as `account use` and `history void` now support short ID input with
  validation (minimum length and ambiguity detection).
- **`compliance_policy` now takes an explicit `date` parameter** — balance validation
  uses `balance_at(date)` instead of `current_balance`, ensuring operations inserted at
  a past date are validated against the correct historical balance. Also fixes
  `monthly_operation_count` to count operations at the correct month.
- **Adjust lock granularity** — void of an operation on the same day as an adjust is now
  resolved by Nulid ordering: operations inserted before the adjust are locked, operations
  inserted after are allowed. Previously all same-day operations were incorrectly allowed.
- **`merge_from_import` now uses `commit_operation` directly** — new operations from
  import are inserted without re-running temporal or compliance policy, as validation
  is handled upstream by `validate_import`. Prevents incorrect rejection of valid
  historical data during merge.
- **Storage format** — write now uses `Ciborium` (official `serde_cbor` successor,
  maintained by the serde team). Read supports both `Cbor` and `Ciborium` for backward
  compatibility.
- **`Account::summary_entry`** now returns `SummaryEntry` directly (not `Option`) —
  a summary is always meaningful even for an empty period, as anchors are account-level.

### Deprecated
- `serde_cbor` — will be removed in a future version once no legacy `Cbor` files remain.

### Removed
- `OperationEntry::new` and `StatementEntry::new` as standalone constructors — replaced
  by `Account::operation_entry()` and `Codexi::statement_entry()` respectively.

### Fixed
- **Compliance validation on past-dated operations** — `compliance_policy` previously
  used `current_balance` for all validations, allowing overdraft violations when
  operations were inserted at a past date. Now uses `balance_at(date)` for accurate
  historical balance at the operation date.
- **Same-day adjust lock bypass** — operations on the same day as an adjust could
  incorrectly bypass the void lock. Now resolved precisely using Nulid ordering.
- **Audit TEST 8** — transfer link consistency check added: operations with `transfer_id`
  must also carry `transfer_account_id`, and must use `Regular::Transfer` kind.
- Option `--open` for commands `report stats` and `report statement` that did not open
  the default browser in some cases.

---

## [0.1.0] — 2026-03-16

*First release of the redesigned workspace version.*

### Added
- **Multi-account support** — create, list, rename, close accounts; switch active account with `account use`
- **Balance integrity** — `op.balance` stored on each operation for auditability with `rebuild_balances_from` for correction
- **Audit command** — `admin audit [--rebuild]` with policy replay, balance cross-reference, void link checks
- **Structured reporting** — `statement`, `stats`, `summary` with HTML export
- **AccountAnchors** — consolidated last dates (init, checkpoint, adjust, void, regular) as a dedicated struct
- **Counts module** — `src/logic/counts` shared across views
- **Script export** — `admin export-script` generates a replayable shell script for each account

### Changed
- **Workspace structure** — project split into two crates: `codexi` (core lib) and `codexi-cli`
- **Operation sorting** — unified sort by `(date, nulid)` in `commit_operation`
- **Balance calculation** — removed incremental `calculate_current_balance`, replaced by `rebuild_balances_from`

### Removed
- Export/import CSV format *(to be reintroduced in a future version)*

---

## Previous versions (legacy mono-crate)
### [2.0.1] — 2026-02-09
- Mono-crate application (`codexi`)

---
