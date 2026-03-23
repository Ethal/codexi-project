# Library Codexi

Core library for the Codexi personal finance ledger.

## Architecture
```plaintext
./crates/codexi/
├── src
│   ├── core           — utilities: formatting, parsing, validation, paths
│   ├── exchange       — import/export (JSON, TOML, CSV mappers)
│   │   ├── mappers
│   │   └── models
│   ├── file_management — persistence: CBOR, snapshots, backup, archive
│   ├── logic
│   │   ├── account    — Account, operations, policy, audit, search, entries
│   │   ├── balance    — Balance calculation and BalanceItem DTO
│   │   ├── bank       — BankList, Bank
│   │   ├── category   — CategoryList, Category
│   │   ├── codexi     — Top-level Codexi struct, multi-account management
│   │   ├── counts     — Counts struct shared across views
│   │   ├── currency   — CurrencyList, Currency
│   │   ├── operation  — Operation, OperationKind, OperationFlow
│   │   └── utils      — Utilities
│   ├── seeds          — Default data (currencies, banks, categories)
│   └── types          — Shared types (DateRange, ...)
└── tests
```

## Error code families

| Prefix | Family | Usage |
| :--- | :--- | :--- |
| SYS_ | System | Critical errors: I/O, locks, file corruption |
| VAL_ | Validation | Format errors: invalid dates, malformed numbers, negative amounts |
| OP_ | Operation | Business logic: void impossible, duplicate ID |
| FIN_ | Financial | Financial policy: closure impossible, already initialized |
| DATA_ | Data/Backup | Snapshots, missing backups, missing folders |
| SRCH_ | Search | Filter errors, inconsistent search periods |

## Key concepts

| Concept | Description |
| :--- | :--- |
| `AccountAnchors` | Cached last dates per operation type (init, checkpoint, adjust, void, regular) |
| `carry_forward_balance` | Opening balance of the current period, updated at each checkpoint |
| `SearchEntry` | Result of a search — base type for all reports and views |
| `OperationContainer` | Trait implemented by `Account` to support generic search |
