# 📔 Codexi CLI

**A high-integrity, anchor-based personal financial ledger built in Rust.**
> 🌐 [codexi.ethal.fr](https://codexi.ethal.fr)

![Rust](https://img.shields.io/badge/Rust-1.94.0-c5a059?logo=rust&style=flat-square) ![Rust Edition](https://img.shields.io/badge/edition-2024-orange?style=flat-square) ![License](https://img.shields.io/badge/License-MIT-gray?style=flat-square) ![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-black?logo=windows&style=flat-square) ![CI](https://img.shields.io/github/actions/workflow/status/ethal/codexi-project/rust.yml?branch=main&style=flat-square&logo=githubactions&label=CI) [![Binaries](https://img.shields.io/badge/Binaries-available-blue?style=flat-square&logo=github)](https://github.com/ethal/codexi-project/releases)

![Codexi Financial Analytics Dashboard](docs/screenshots/stats_dashboard.png)

---

## 📔 Description

Codexi is a command-line personal finance ledger focused on auditability, traceability, and long-term data integrity. It supports multiple accounts, anchor-based integrity checks, period closing with archival, and a rich analytics dashboard — all stored in a versioned, checksummed binary format.

## 📚 Documentation

- [Loan & microloan management](docs/loan.md)
- [Compliance matrix](docs/compliance_matrix.md)
- [Context matrix](docs/context_matrix.md)

---

## 🧠 Design Philosophy

Codexi does not prevent financial states — it documents them.
Negative balances, large adjustments, and corrections are allowed by design.
Integrity is enforced through explicit operations and full auditability,
never by silent constraints.

Operations are **immutable once recorded** — financial fields cannot be
changed after the fact, matching real-world banking practice. Errors are
corrected by explicit void operations, which create compensating entries
dated today. This preserves a complete, tamper-evident audit trail.

---

## ✨ Features

- **Multi-Account** — manage several accounts, switch active account at any time
- **Account Types** — Current, Joint, Business, Student, Saving, Deposit, Loan, Income — each with its own compliance rules
- **Per-Account Policy** — compliance rules (overdraft, minimum balance, monthly quota, deposit lock) configurable per account instance via `account set-context`
- **Anchor-Based Integrity** — operation dates validated against history anchors
  (`INIT`, `CLOSE`, `ADJUST`) with same-day precision using Nulid ordering
- **Period Closing & Archival** — formally close periods into `.cld` archive files
  with a carried-forward balance
- **Transfer Between Accounts** — cross-account and cross-currency transfers with
  automatic net exchange rate calculation; atomic void support
- **Financial Analytics Dashboard** — savings rate, daily burn rate, top expenses,
  system health
- **HTML Statement Export** — rendered report openable directly in your browser
- **Multi-format I/O** — export and import via JSON or TOML
- **Snapshot & Backup** — lightweight snapshots for quick rollback, full ZIP backups
  including archives
- **Exact Arithmetic** — fixed-point decimal (`rust_decimal`), no floating-point errors
- **Explicit Void Semantics** — operations are never deleted; voids create compensating
  entries dated today, matching real-world banking practice
- **Immutable Operations** — once recorded, financial fields (amount, date, kind, flow)
  cannot be modified. Only metadata (description, context, tags) can be updated via import.
- **Counterparty Tracking** — attach a counterparty (person or organisation) to any operation via `-c`; manage counterparties with `counterparty list/add/terminate`
- **Category Tagging** — tag any operation with a category via `-g` including transfers
- **Microloan Calculator** — standalone `loan` command to simulate linear or compound interest with configurable policy (cap, penalty, free period); policy persisted in `tmp/loan_policy.json`
- **Versioned Storage (V3)** — Ciborium binary format with magic header, versioning,
  and SHA-256 checksum
---

## 🚀 Installation

**Prerequisites:** Rust 1.85+ ([rustup.rs](https://rustup.rs))

```bash
git clone https://github.com/ethal/codexi-project.git
cd codexi-project
cargo build --release
./target/release/codexi-cli --help
```
---

## 📖 Typical Workflow

```bash
# 1. Initialize a new account
codexi-cli account create 2025-01-01 "My Bank Account" --type Current
codexi-cli account set-bank <bank_id|bank_name>         # see: codexi-cli bank list
codexi-cli account set-currency <currency_id|code>      # see: codexi-cli currency list
codexi-cli account set-context --overdraft 500 --min-balance 0
codexi-cli history init 2025-01-01 1500.00

# 2. Record daily operations
codexi-cli credit 2025-01-05 2400.00 "Monthly salary"
codexi-cli debit  2025-01-06 45.00  "Groceries"

# 3. Transfer between accounts
codexi-cli transfer 2025-01-10 100.00 1500000 <account_id|name> "ATM withdrawal"

# 4. Consult and analyze
codexi-cli view
codexi-cli report balance
codexi-cli report stats --from 2025-01-01 --to 2025-01-31
codexi-cli report statement

# 5. Protect your data before risky operations
codexi-cli data snapshot create
codexi-cli data import currency json
codexi-cli data import account-header json
codexi-cli data import operation json

# → It is recommended to run: admin audit --rebuild

# 6. Close a period at year end
codexi-cli admin backup
codexi-cli history close 2025-12-31 "Closing Year 2025"
codexi-cli admin backup
```
---

## 🗂️ Command Reference

### Core
| Command | Description |
| :--- | :--- |
| `overview` | Show an overview of the ledger |
| `credit <date> <amount> [desc] [-c counterparty] [-g category]` | Record an incoming flow |
| `debit <date> <amount> [desc] [-c counterparty] [-g category]` | Record an outgoing flow |
| `interest <date> <amount> [desc] [-c counterparty] [-g category]` | Record an interest accrual (Loan/Saving/Deposit/Income accounts) |
| `transfer <date> <amount_from> <amount_to> <account_id\|name> [desc] [-g category]` | Transfer from current account to another |
| `search` (`view`) `[--from] [--to] [--text] [--kind] [--flow] [--a-min] [--a-max] [--last] [--today] [--open]` | Search and filter operations |

### Operation
| Command | Description |
| :--- | :--- |
| `operation view <id> [--raw]` | view an operation  |

### Account
| Command | Description |
| :--- | :--- |
| `account list` | List all accounts (`*` = active, `c` = closed) |
| `account create <date> <name> [--type]` | Create a new account (default: Current) |
| `account use <id>` | Switch active account |
| `account close <id> <date>` | Close an account |
| `account rename <id> <name>` | Rename an account |
| `account context` | View the context of the current account |
| `account set-bank <bank_id>` | Set bank for current account |
| `account set-currency <currency_id> [--update-operation]` | Set currency for current account |
| `account set-context [--overdraft] [--min-balance] [--max-monthly-transactions] [--deposit-locked-until] [--interest] [--signers]` | Configure compliance parameters for current account |

### Bank
| Command | Description |
| :--- | :--- |
| `bank list` | List all the bank available |

### Currency
| Command | Description |
| :--- | :--- |
| `currency list` | List all the currency available |

### Category
| Command | Description |
| :--- | :--- |
| `category list` | List all the category available |
| `category add <name> [--parent] [--note]` | Add a category |
| `category terminate <id\|name>` | Terminate a category |

### Counterparty
| Command | Description |
| :--- | :--- |
| `counterparty list` | List all counterparties |
| `counterparty add <name> <kind> [--note]` | Add a counterparty (`Person` or `Organization`) |
| `counterparty terminate <id\|name>` | Terminate a counterparty |

### Loan
| Command | Description |
| :--- | :--- |
| `loan policy show` | Display current loan policy |
| `loan policy set [--type] [--rate] [--free-days] [--max-cap] [--max-days] [--min-capital] [--max-penalty]` | Update policy fields (unspecified fields unchanged) |
| `loan policy reset` | Reset policy to default values |
| `loan simulate --capital <amount> --start <date> --refund <date> [--type] [--rate] [--free-days]` | Simulate interest and amount due |

### Report
| Command | Description |
| :--- | :--- |
| `report balance [--from] [--to]` | Debit / credit / balance summary |
| `report stats [--from] [--to] [--open]` | Full analytics dashboard |
| `report summary` | Quick overview of the current account |
| `report statement [--from] [--to] [--open]` | Export an HTML statement |

### History
| Command | Description |
| :--- | :--- |
| `history init <date> <amount>` | Initialize ledger with a starting balance |
| `history adjust <date> <amount>` | Adjust balance to a physical amount |
| `history void <id>` | Void an operation (creates a compensating entry) |
| `history close <date> [desc]` | Close a period and archive transactions |
| `history archive` | Manage the archived file |

| Command | Description |
| :--- | :--- |
| `history archive list` | List archive files (`.cld`) |
| `history archive view <account_id> <date>` | View the content of an archive file |

### Data
| Command | Description |
| :--- | :--- |
| `data export <currency\|counterparty\|category\|account-header\|operation> <json\|toml\|csv>` | Export data |
| `data import <currency\|counterparty\|category\|account-header\|operation> <json\|toml\|csv>` | Import data |
| `data snapshot` | Manage the snapshot of the active ledger |

| Command | Description |
| :--- | :--- |
| `data snapshot create` | Lightweight snapshot of the active ledger |
| `data snapshot list` | List available snapshots |
| `data snapshot restore <filename>` | Restore from a snapshot |
| `data snapshot clean [--keep N]` | Remove old snapshots (keeps 5 by default) |

### Maintenance
| Command | Description |
| :--- | :--- |
| `admin infos` | Display ledger metadata and storage info |
| `admin backup [--target-dir]` | Full ZIP backup (ledger + archives) |
| `admin restore <filename>` | Restore from a ZIP backup |
| `admin migrate <version>` | Migrate ledger and archives to a new format version |
| `admin audit [--rebuild]` | Audit the current account and rebuild balance as per option |
| `admin clear-data` | ⚠️ Move ledger files to trash |
| `admin trash` | ⚠️ Manage the trash |
| `admin export-special` | Raw JSON export (no validation) |
| `admin import-special` | ⚠️ Raw JSON import (no validation) |
| `admin export-script` | Export ledger operations in scripts for a replay |

| Command | Description |
| :--- | :--- |
| `admin trash restore <datetime>` | ⚠️ Restore from trash |
| `admin trash purge` | ⚠️ Empty the trash directory |

---

## 📊 Analytics Dashboard (`report stats`)

- **Smart filtering** — `INIT` and `CLOSE` operations always excluded; `ADJUST` excluded from behavioral metrics
- **Void semantics** — voided operations are excluded from calculations; both sides of a void pair must exist within the selected period to be included.
- **Savings Rate Bar** — dynamic indicator, turns to danger mode if expenses exceed income
- **Daily Burn Rate** — average daily spending over the selected period
- **System Health** — tracks adjustment ratio to monitor data quality

---

## 📂 Import / Export

Fixed filenames are used for simplicity:
- **Export** → creates `codexi.<ext>` in the current directory
- **Import** → expects `codexi.<ext>` in the current directory

> ⚠️ Always run `data snapshot create` before an import.

JSON and TOML exports include an `export_version` field (currently **V2**) for forward compatibility. These formats are interchange-only and do not carry internal storage metadata.

---

## 🛡️ Data Safety Layers

```
[ Active Ledger ]  --snapshot-->  [ snapshots/ (.snp) ]
       |
  system close
       |
[ archives/ (.cld) ]  --system backup-->  [ backup.zip ]
```

---

## 🗃️ Data Location

| OS | Path |
| :--- | :--- |
| **Linux** | `~/.local/share/fr.ethal.codexi/` |
| **macOS** | `~/Library/Application Support/fr.ethal.codexi/` |
| **Windows**| `%AppData%\Roaming\fr.ethal.codexi\` |

Files: `codexi.dat` (active ledger) · `archives/` · `snapshots/` · `tmp/` · `trash/`

---

## 🏗️ Project Structure

Codexi is organized as a **Cargo workspace** with two crates:

- **`crates/codexi`** — the core library: domain logic, storage, analytics, import/export. No CLI dependency.
- **`crates/codexi-cli`** — the command-line interface built on top of the library.

This separation keeps the business logic independently testable and reusable.

A companion **`www/`** directory contains the static website hosted at [codexi.ethal.fr](https://codexi.ethal.fr).

---

## 🧭 Versioning

| Layer | Version | Notes |
| :--- | :--- | :--- |
| Application (CLI) | `0.4.0` | Semantic versioning — active development |
| Core library | `0.4.0` | Semantic versioning — active development |
| Storage format | `V3` | Ciborium binary, magic header, SHA-256 checksum |
| Export format (JSON/TOML) | `V2` | Interchange only, no storage metadata |

> **Note**: CLI versions `1.0.0` → `2.0.1` correspond to an earlier
> single-binary architecture, kept as git tags for reference.
> `serde_cbor` (V3 legacy) files remain readable for backward compatibility.
---

## 🤝 Contributing

Bug reports and pull requests are welcome via GitHub.

## 📄 License

MIT

## 📬 Author

**ethal** — [codexi@ethal.fr](mailto:codexi@ethal.fr)
