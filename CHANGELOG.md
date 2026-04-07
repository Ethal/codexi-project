# Changelog
All notable changes to this project will be documented in this file.

---


Add a flag and UI indicator to mark accounts that are expected to have zero balance. Introduce AccountType::is_zero_balance_expected() (Loan/Income) and propagate it into AccountItem as is_zero_balance_expected. Update DTO mapping to set the flag and update the CLI view to display a ✓ when balance = 0 or a ! when a non-zero balance is unexpected; adjust table column widths/formatting and add a legend. Also import Decimal in the CLI module and tweak a comment text.

## [Unreleased] — 

### Added
- **CLI command `report monthly`** — support `--from` and `--to`.
- **CLI command `use`** — alias for the command `account use`.
- **DTOs `CategoryItem::build`** — to resolve parent data.
- **DTOs `MonthlyReport::build`** — used by the new command `report monthly.
- **UI `view_montly`** — view of  the new command `report monthly`.

### Changed
-**UI** — Introduce unified UI style constants and helpers.
-**UI** — account switch messaging to include the account name and id ("Switched to account: <name> (<id>)").
- **UI `Top Expense`** — tie-breaker added to compare op_date so items with equal amounts are ordered deterministically.
- **UI `view_category_list`** — Enhance category listing and logic to surface parent info and termination state.
- **DTOs** — include parent_name and parent_terminated in CategoryItem, update CategoryCollection::build accordingly.
- **ExchangeCCategoryList.list** — Add a serde rename, it serializes/deserializes as the JSON key "categories".
- **ExchangeCounterpartyList.list** — Add a serde rename, it serializes/deserializes as the JSON key "counterparties".
- **Import to codexi** — refactoring import functionality out of the `Codexi model` to `Codexi import`, improve separation of concerns.
- **UI `view_codexi_infos`** — add count of all operations of the ledger included the one in archive files.
- **UI `overview_account`** — Add an UI indicator to mark accounts that are expected to have zero balance.

### Fixed

## [0.4.0] — 2026-04-05
> ⚠️ Breaking: DTO modules have been moved from `logic/*` to `dto/*`

> ⚠️ Breaking: `Operation` structure, additional field `account_id`, performed the command `admin audit --rebuild`

### Added

- **CLI command `loan policy show`** — display current persisted loan policy.
- **CLI command `loan policy set`** — update one or more policy fields without
  overwriting unspecified fields. Flags: `--type`, `--rate`, `--free-days`,
  `--max-cap`, `--max-days`, `--min-capital`, `--max-penalty`.
- **CLI command `loan policy reset`** — reset policy to default values and persist.
- **CLI command `loan simulate`** — compute amount due for a given loan.
  Required: `--capital`, `--start`, `--refund`.
  Optional overrides (without modifying policy): `--type`, `--rate`, `--free-days`.
  Displays: amount due, total interest, first interest date, per-day interest breakdown.
- **CLI Command** — `overview` to display the main informations of the accounts, including the balance, account type, bank, and currency. 
- **CLI commands** — `data export category` and their `import` counterparts. Format argument: `json or toml`. CSV reserved, returns `UnsupportedFormat`.
- **CLI commands** — `data export couterparty` and their `import` counterparts. Format argument: `json or toml`. CSV reserved, returns `UnsupportedFormat`.
- **Account::set_context** as a wrapper around `context.update_context`.
- **termination guards** in `set_context`, `set_bank` and `set_currency` to prevent mutating terminated accounts.
- **`ignored`** operations list to stats output to surface operations excluded from calculations (e.g. unmatched void/voided pairs)
- **ExchangeCategoryList** — dedicated exchange DTO for the category list. Export produces `categories.json/toml`. Import merges categoris — new categories created, existing updated by id. Duplicate code against existing list rejected as error.
- **Import validation — categories** — `validate_import_categorie` covers: version, id format if present, name.
- **Seed** — default seed for counterparty are available at the creation of a new ledger.
- **Counterparty** — model, DTO, exchange import/export and validation wired end-to-end.
  Fields `id`, `name`, `kind` (`Person` / `Organisation`), `note`, `terminated`.
  Includes default seed at ledger creation and CLI subcommand `counterparty` (list, add, terminate).
- **`-c / --counterparty`** flag added to CLI commands `debit`, `credit`, `interest`.
  Accepts full id, short id, or name (exact / prefix / contains). Not available on `transfer` (inter-account operation).
- **`-g / --category`** flag added to CLI commands `debit`, `credit`, `interest`, `transfer`.
  Same resolution strategy as counterparty.
- **`counterparty_id` and `category_id`** added to `OperationContext` — carried through
  `register_transaction`, `transfer`, merge, audit and rebuild.
- **`ExchangeCounterparty`** — exchange DTO for counterparty import/export. Validation covers
  duplicate ids, unknown kind. Import creates or updates counterparties by id.
- **`logic/loan` module** — standalone calculation module for microloan tracking.
  No ledger integration — pure computation aid. Two interest models supported:
  - `LinearInterest` — simple daily interest: `capital × rate × late_days`
  - `CompoundInterest` — compound daily interest: `capital × ((1 + rate)^days - 1)`
  Both models share a common `LoanBase` with `LoanPolicy` (cap, penalty, min capital,
  max duration, free period). Cap applies to interest only; penalty is a fixed percentage
  of capital added after cap. `LoanSummary` returns `final_due`, `total_interest`,
  per-day interest vector with dates, and `first_interest_date`.
- **`LoanKind`** — enum (`Linear` / `Compound`) shared between domain model and settings.
  Replaces the former `LoanInterestType` in settings layer.
- **`LoanPolicySettings`** — persisted loan policy stored in `tmp/loan_policy.json`.
  Loaded via `load_or_create`, saved via `save`, reset to defaults via `reset`.
  Converted to domain `LoanPolicy` via `to_loan_policy()`.

### Changed
- **CLI handlers** to use new DTO construction patterns
- **CLI command** `search`, alias `view` now support the flag `--open` to open the result in the default browser.
- **Refactored DTO layer**:
  - Moved DTOs from `logic/*` into dedicated `dto/*` modules
  - Simplified DTOs to only handle basic type conversions (e.g. `NaiveDate`, `Nulid`, `Path` → `String`)
- **Moved formatting logic** (e.g. booleans, display helpers) from DTOs to CLI UI layer
- **Stats calculation** — Reworked logic to ensure consistency with account statements.
  - Stats now exclude init and checkpoint operations while including all other valid operations,
  - Introduced robust handling of void and voided operations: Operations are only included if both sides of the pair exist within the selected period,
  - Unmatched operations are excluded from financial calculations,
  - Improved accuracy of financial indicators (balance, totals, savings rate, averages, daily burn rate),
  - Refactored stats computation pipeline for clarity, consistency, and performance (single-pass processing).
- **`account_id`** added to `Operation` structure — existing data requires `admin audit --rebuild`.
- **`Account::set/merge/audit`** updated to handle legacy operations missing `account_id`
  and to populate it during audit/rebuild.

### Removed
- **`AccountBalance`** and **`CodexiBalance`** from Balance. 
- **`signed_amount`** function in `logic/operation/model.rs`.
- **CLI Command** - `report balance-all`. Replace by the command `òverview`.
- **Formatting responsibilities** from DTOs (now handled exclusively in UI)
- Replaced legacy `entry.rs` mapping functions with DTO-based conversions
- **CLI command** `report stats`, the flag `--net` is not more available 


## [0.3.0] — 2026-03-28

### Added
- **ExchangeAccountHeader** — dedicated exchange DTO for account metadata (name, context, bank, currency, dates, meta). Replaces the monolithic `ExchangeData`. Export produces `account-header.json/toml`, import creates or updates account metadata only — operations untouched.
- **ExchangeAccountOperations** — dedicated exchange DTO for operations tied to an account (`account_id` + `Vec<ExchangeOperation>`). Export produces `operations.json/toml`. Import merges operations into the target account identified by `account_id` — account must exist, terminated accounts rejected.
- **ExchangeCurrencyList** — dedicated exchange DTO for the currency list. Export produces `currencies.json/toml`. Import merges currencies — new currencies created, existing updated by id. Duplicate code against existing list rejected as error.
- **AccountOperations** — new domain struct in `logic/operation` grouping `account_id` and `Vec<Operation>`. Used as the unit of exchange for operation import/export. `Account::to_account_operations()` builds a snapshot for export.
- **Exchangeable** trait — single generic trait replacing the former `JsonExchange` + `TomlExchange` duplication. Each exchangeable type declares its `Exchange` DTO, `exchange_filename`, `to_exchange` and `from_exchange`. Implemented for `Account`, `CurrencyList`, `AccountOperations`. 
- **ExchangeSerdeFormat** — central dispatch for serde I/O. Single place in the codebase that knows about `serde_json` and `toml. Bridges `ExchangeError` (domain)` → `FileExchangeError` (I/O) via `From`.
- **ExchangeBase trait** — associates a `Warning` type to each exchangeable domain type. Currently `CoreWarning` for all implementations.
- **Import validation — operations** — `validate_import_operations` covers: version, account id format, duplicate operation ids, strictly positive amounts, transfer link consistency (both fields or neither), transfer kind coherence, void consistency (void must reference, no double-void, no void-of-void). Operations without id (new) only allowed without void/transfer links.
- **Import validation — currencies** — `validate_import_currency` covers: version, id format if present, non-empty code, ISO 4217 length (exactly 3 chars), duplicate codes within the file.
- **Import validation — account header** — `validate_import_account_header` covers: version, non-empty name, known `account_type`.
- **Identity resolution on currency import** — `CurrencyList::resolve_id` resolves incoming currency identity: known id → reuse, unknown id but matching code → reuse existing id, no match → generate fresh Nulid. Prevents duplicate currencies across repeated imports.
- **Merge** — **currencies** — `merge_from_import` updates `symbol` and `note` on existing currencies (by id). New currencies added via `add()`. Duplicate code against existing list returns `CurrencyError::DuplicateCode` — full import rejected.
- **Merge** — **account header** — `merge_account_header_from_import` updates `name`, `currency_id`, `bank_id`, `context`, `meta`. Terminated accounts rejected. `carry_forward_balance`, `open_date`, `anchors`, `checkpoints` never updated via import.
- **Merge** — **operations** — `merge_operation_from_import` updates `description`, `context` (category, payee, reconciled), `meta` on existing operations. New `Transaction` and `Init` operations added. Unsupported kinds (`Void`, `Transfer`, `Adjust`) skipped with `CoreWarning`. `refresh_anchors() always called after merge.
- **CLI commands** — `codexi-cli data export account-header`, `codexi-cli data export operation`, `codexi-cli data export currency` and their `import` counterparts. Format argument: `json or toml`. CSV reserved, returns `UnsupportedFormat`.
- **`FileExchangeError::UnsupportedFormat`** — returned when CSV format requested for import/export.
- **`resolve_or_generate_id`** helper in `core/parse.rs` — resolves `Option<&str>` to `Nulid`: parses if present (expects prior validation), generates fresh Nulid if absent.
- **`default_zero`** serde helper — used for optional decimal string fields (`carry_forward_balance`, `current_balance`, `overdraft_limit`, `min_balance`) defaulting to `"0"` when absent.
- **Accountype** — New account type for loan, Income context and policy updated accordingly
- **`HasName` trait** — optional trait in `logic/utils` decoupled from `HasNulid`. Implemented on types that carry a human-readable name field (currently `Account`). Types without a name (`Operation`, `Transfer`, …) are unaffected and continue to use `resolve_id`.
- **`resolve_by_id_or_name`** — new generic resolver in `logic/utils` combining ID-based and name-based lookup in a single call. Resolution priority: full Nulid (26 chars) → ID suffix → name exact → name prefix → name contains. Ambiguity is evaluated independently at each level — a unique exact match wins regardless of prefix/contains candidates. Available only for types implementing both `HasNulid` and `HasName`.
- **`resolve_by_name`** (internal) — three-tier name search (exact / prefix / contains, case-insensitive) extracted as a private helper. Keeps `resolve_by_id_or_name` readable and allows independent testing of the name tier.
- **`match_unique`** (internal) — shared finalizer converting a candidate `Vec<&T>` into `Ok(Nulid)`, `Err::not_found`, or `Err::ambiguous`. Eliminates duplicated match-count logic across resolution tiers.
- **CLI command `account use`** — now accepts account name (or prefix/substring) in addition to full or short ID. `account use perso`, `account use wall`, `account use alc` all resolve correctly. Ambiguous input (e.g. `al` matching multiple accounts) returns a clear error prompting for more characters.
- **CLI command `account set-bank`** — now accepts <bank_name> in addition to <bank_id>.
- **CLI command `account set-currency`** — now accepts <currency_code> in addition to <currency_id>.
- **CLI command `transfer`** — <account_id_to> argument now accepts an account name (or prefix/substring) in addition to full or short ID.
- **`CodexiBalanceEntry`** — in `logic/balance/model.rs` to compute balance, debit, credit of the accounts.
- **CLI command `report balance-all`** — display the balance, debit and credit per accounts from `CodexiBalanceEntry`.
- **`RegularKind::Interest`** — new operation kind for interest accrual and cash receipt. Semantically distinct from `Transaction` — enables filtering, reporting, and enforcement of `allows_interest` per account. Used with `Credit` flow to record interest due (option B: accrual as additional debt), voided via `SystemKind::Void` for interest forgiveness.
- **`ComplianceViolation::KindNotAllowed(AccountType)`** — new typed error replacing the former `NotAllowed { reason }` catch-all. Carries the account type for clear error messages.
- **`ComplianceViolation::InitNonZeroOnLoan`** — new typed error for `Init` operations with a non-zero amount on `Loan` accounts.
- **`AccountType::allows_interest()`** — new method returning `true` for `Saving`, `Deposit`, `Income` and `Loan`. Used as default initializer for `AccountContext::allows_interest` in `from_type()`.
- **`AccountType::allows_joint_signers()`** — new method returning `true` for `Joint` and `Business`. Used as default initializer for `AccountContext::allows_joint_signers` in `from_type()`.
- **`AccountContext::from_type()`** — now uses `allows_interest()` and `allows_joint_signers()` methods for initialization. `Loan` and `Income` defaults to `allows_interest: true`. No more hardcoded booleans.
- **`validate_full` signature** — now receives `_kind: &RegularKind` and `_flow: OperationFlow` to allow per-type kind/flow guards in overrides without duplicating the dispatch logic from `validate()`.
- **`LoanPolicy`** — full `validate()` override. Blocks `Init` with non-zero amount (`InitNonZeroOnLoan`). Restricts Regular ops to `Transfer` (both flows), `Interest Credit` (if `allows_interest`), and `Fee` (both flows). All others return `KindNotAllowed`.
- **`SavingPolicy`** — `validate_full` override blocks `Transfer Debit` explicitly (`KindNotAllowed`) regardless of balance. All other debits blocked by `validate_no_overdraft`.
- **`DepositPolicy`** — all debits blocked before `deposit_locked_until` via `NoWithdrawalAllowed`. Credits always allowed. No overdraft after maturity.
- **`validate_no_overdraft`** — shared private helper now used by `SavingPolicy`, `DepositPolicy`, `LoanPolicy` and `IncomePolicy`. Returns `NegativeBalanceNotAllowed(AccountType)`.
- **Compliance matrix** — full kind × flow × account-type decision table established as reference. Covers all `RegularKind` (Transaction, Transfer, Interest, Fee, Refund) and `SystemKind` (Init, Adjust, Checkpoint, Void) combinations across all eight account types.
- **`AccountType::Income`** — new account type for pure accumulation (interest income, revenue pools). Transfer only in both directions, no overdraft, no negative balance. Replaces `Deposit` as the recommended type for interest income accounts.
- **`IncomePolicy`** — compliance policy for `Income` accounts. Only `Transfer` (Credit/Debit) allowed. All other Regular kinds (`Transaction`, `Fee`, `Refund`, `Interest`) return `KindNotAllowed`.
- **`docs/loan.md`** — user guide for microloan management: account setup, operation cycle, interest accrual pattern, full worked example with balance states at each step.
- **`docs/compliance_matrix.md`** — reference table of all allowed/refused/conditional operations by kind, flow, and account type.
- **`docs/context_matrix.md`** — reference table of all configurable context fields by account type with default values.

### Changed
- **ExchangeData removed** — replaced by `ExchangeAccountHeader` + `ExchangeAccountOperations`. Split clarifies responsibilities and produces smaller, more readable export files.
- **ImportSummary.account_name** renamed to **name** — field is now used for any imported entity (account, currencies), not accounts only.
- **export_toml** / **export_json** / **import_toml** / **import_json** — now generic over `T: Exchangeable`. Format dispatch centralized in `ExchangeSerdeFormat`, `json.rs` and `toml.rs` are thin wrappers.
- **import_operations** in **Codexi** — returns `Result<(ImportSummary, Vec<CoreWarning>), CodexiError>` — merge warnings (skipped kinds) propagated to CLI.
- **Validation module** — `validation.rs` renamed to `validator/` with per-entity files (`account.rs, currency.rs, operation.rs)`. `validate_import` split into `validate_import_account_header`, `validate_import_currency`, `validate_import_operations`.
- **CLI commands** — `account list` and `account context` output view updated to be more friendly.
- **`ComplianceAction`** — `Create` variant now owns `OperationKind` and `OperationFlow` by value (was `&'a OperationKind`). Lifetime removed — `OperationKind` is `Copy`.
- **`ComplianceViolation::NotAllowed`** — removed. Replaced by typed variants `KindNotAllowed(AccountType)` and `InitNonZeroOnLoan`. Error messages are now generated by `thiserror` from the type, not from a `&'static str` field.
- **`AccountType::Display`** — padding (`{:<7}`) removed from `Display` implementation. Alignment is now the responsibility of the CLI display layer, not the domain type.
- **`validate_no_overdraft`** — extracted as a shared private function for all no-overdraft policies. Previously duplicated between `LoanPolicy` and `SavingPolicy`.
- **CLI commands** — `admin export-script` take into account the command `transfer` and `interest` in the script generation.

### Fixed
- **export_json** wrote **json.as_bytes()** — now writes `String` directly via `fs::write`, removing the unnecessary `.as_bytes()` conversion.
- **Account::new()** — `account.context` was not properly set as per the account type. It now calls `AccountContext::from_type(account_type)` so the context is initialized correctly for the given account type

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
