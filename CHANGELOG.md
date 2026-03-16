# Changelog

---

## [Unreleased]

### Added
- **Multi-account support** — create, list, rename, close accounts; switch active account with `account use`
- **Balance integrity** — `op.balance` stored on each operation, `rebuild_balances_from` for correction
- **Audit command** — `admin audit [--rebuild]` with policy replay, balance crossref, void link checks
- **Structured reporting** — `statement`, `stats`, `summary` with HTML export
- **AccountAnchors** — consolidated last dates (init, checkpoint, adjust, void, regular) as a dedicated struct
- **Counts module** — `src/logic/counts` shared across views
- **Script export** — `admin export-script` generates a replayable shell script per account

### Changed
- **Workspace structure** — project split into two crates: `codexi` (core lib) and `codexi-cli`
- **Operation sorting** — unified sort by `(date, nulid)` in `commit_operation`
- **Balance calculation** — removed incremental `calculate_current_balance`, replaced by `rebuild_balances_from`

### Removed
- Export/import CSV format *(to be reintroduced in a future version)*

---

## [2.0.1] — 2026-02-09
- Mono-crate application (`codexi`)

---
