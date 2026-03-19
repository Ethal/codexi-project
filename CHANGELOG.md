# Changelog
All notable changes to this project will be documented in this file.
---

## [Unreleased]

> ⚠️ No automatic migration — use `admin export-script` to rebuild your data.

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

- **Generic ID resolution (`resolve_id`)** — reusable mechanism to resolve full or short IDs (Nulid) across entities (operations, accounts, etc.). Supports suffix-based lookup with configurable minimum length (`MIN_SHORT_LEN`).

- **Traits `HasNulid` and `ResolveError`** — enable generic ID resolution by decoupling entity identification and error handling from business logic. Allows reuse across multiple domains (Account, Bank, Currency, Category, ...).

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

- **Short ID support in CLI commands** — entities can now be referenced using the last characters of their ID instead of the full 26-character Nulid.

- Command `account set-context` to update the configurable parameters of the current account.
- Command `account context` to view the context of the current account.
- Command `account create` now accepts an optional account type argument (Current, Joint, Saving, Deposit, Business, Student). Defaults to Current.
- Command `transfer <DATE> <AMOUNT_FROM> <AMOUNT_TO> <ACCOUNT_ID_TO> [DESCRIPTION]`
- Command `void` now handles both regular operations and transfer operations transparently.

### Changed
- Command `account list` now shows the account type.
- Refactored ID resolution logic — moved from account-specific implementation to a generic utility (`resolve_id`), improving consistency and reducing duplication.
- Commands such as `account use` and `operation void` now support short ID input with validation (minimum length and ambiguity detection).

### Deprecated
- **File storage format** migrated from `Cbor` (`serde_cbor`, abandoned upstream) to
  `Ciborium` (official successor, maintained by the serde team). Read supports both
  `Cbor` and `Ciborium` for backward compatibility. Write uses `Ciborium` only.
  `serde_cbor` will be removed in a future version once no legacy files remain.

### Removed

### Fixed
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
