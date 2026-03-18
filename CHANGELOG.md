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

- Command `account set-context` to update the configurable parameters of the current account.
- Command `account context` to view the context of the current account.
- Command `account create` now accepts an optional account type argument
  (Current, Joint, Saving, Deposit, Business, Student). Defaults to Current.

### Changed
- Command `account list` now shows the account type.

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
