# 📔 Codexi CLI

**A high-integrity, anchor-based financial ledger built in Rust.**
> 🌐 [codexi.ethal.fr](https://codexi.ethal.fr)

![Rust](https://img.shields.io/badge/Rust-1.90.0-c5a059?logo=rust&style=flat-square)![License](https://img.shields.io/badge/License-MIT-gray?style=flat-square) ![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-black?style=flat-square)

![Codexi Financial Analytics Dashboard](docs/screenshots/stats_dashboard.png)

---

## 📔 Description

Codexi is a robust, command-line personal finance management application built in Rust. It focuses on maintaining an accurate, auditable, and secure ledger of transactions through a system of anchor-based integrity checks and automatic archival.
Codexi favors auditability over restriction: negative balances, corrective adjustments, and void operations are allowed but always explicit, traceable, and verifiable.

## 🧠 Design Philosophy

Codexi does not prevent financial states — it documents them.
Negative balances, large adjustments, and corrections are allowed by design.
Integrity is enforced through explicit operations, validation rules, and full auditability — never by silent constraints.

## ✨ Features

* **Anchor-Based Integrity:** Ensures transaction history is tamper-proof by checking operation dates against system anchors (`INIT`, `CLOSE`, `ADJUST`).
* **Auditable Closing:** Periods can be formally closed, archiving all transactions into external files (`.cld`) while replacing them with a single **Carried Forward Balance** in the active ledger.
* **Advanced Analytics:** All-in-one dashboard with savings rate, daily burning rates, and smart expense tracking.
* **Multi-format Interoperability:** Export and import your data using JSON, TOML, or CSV formats.
* **Snapshot Recovery:** Offers internal snapshot functionality for quick rollback before risky operations (like bulk imports).
* **Data Security:** Full backup to external ZIP archives and internal snapshot functionality for quick rollback.
* **Exact Monetary Arithmetic:** All amounts are stored using fixed-point decimal arithmetic (no floating-point errors).
* **Explicit Void Semantics:** Operations are never deleted. Voids create compensating entries, preserving full historical traceability.
* **Versioned Storage (V2):** Binary storage based on CBOR with magic header, versioning, and checksum verification.

## 🚀 Installation

### Prerequisites

You need to have **Rust** and **Cargo** installed. You need **Rust 1.80+** (1.90.0 recommended).

### Build from Source

1.  Clone the repository:
    ```bash
    git clone [https://github.com/ethal/codexi.git](https://github.com/ethal/codexi.git)
    cd codexi
    ```
2.  Build and run the application using Cargo:
    ```bash
    cargo build --release
    ./target/release/codexi [COMMAND]
    ```
*(Note: For simplicity, all subsequent commands assume you run them via `./target/release/codexi`)*

## 📖 Usage

### Core Operations

| Command | Description | Example |
| :----- | :----- | :----- |
| `credit [date] [amount] [description]` | Adds funds to the ledger. | `codexi credit 2025-11-02 1500.00 Monthly Salary` |
| `debit [date] [amount] [description]` | Records an expense. | `codexi debit 2025-11-02 34.50 Grocery` |
| `search [Criteria]` | Displays the active transaction ledger with cumulative balances as per search criteria or all active transactions if no criteria | `codexi search` |

### Report Commands

| Command | Description | Example |
| :--- | :--- | :--- |
| `report balance [Criteria]` | Displays the balance of the active transaction ledger. | `codexi report balance` |
|` report resume` | Displays a resume of the active transaction ledger. | `codexi report resume` |
|` report stats [Criteria]` | **(New)** Displays a full visual dashboard with analytics. | `codexi report stats` |

### System Commands

These commands manage the integrity and security of the ledger.

#### 1. Initialize, Ajustment, Void 

| Command | Description | Example |
| :--- | :--- | :--- |
| `system init [date] [amount]` | Initialize the ledger with a initial amount. | `codexi system init 2026-01-01 150.00` |
| `system adjust [date] [amount]` | Adjusts the balance to a given physical amount. | `codexi system adjust 2026-01-05 60.00` |
| `system void [index]` | Void an existing operation without deleting it. A compensating operation is created to preserve history. | `codexi system void 10` |


#### 2. Period Closing and Archival

| Command | Description | Example |
| :--- | :--- | :--- |
| `system close [date]` | Archives transactions and replaces them with a Carried Forward Balance entry (`CLOSE`). | `codexi system close 2025-11-30` |
| `system list` | Lists all closed archive files (`.cld`) in the data directory. | `codexi system list` |
| `system view [filename]` | Displays the operations contained within a specific archive file. | `codexi system view codexi_2025-11-30.cld` |

#### 3. Backup and Restore

Backups are created as compressed ZIP files containing the active ledger (`codexi.dat`) and all historical archives (`archives/`).

| Command | Description | Example |
| :--- | :--- | :--- |
| `system backup` | Creates a full backup ZIP file. Stores it in your system's **Documents** folder by default. | `codexi system backup` |
| `system backup --target-dir [path]` | Creates a full backup ZIP file at the specified location. | `codexi system backup --target-dir /media/usb/my_codexi.zip` |
| `system restore [path_to_zip]` | Restores the active ledger and archives from a backup ZIP file. **⚠️ Warning: This will overwrite current data.** | `codexi system restore /home/user/my_backup.zip` |

#### 4. Snapshots (Quick Recovery)

Snapshots are lightweight backups of **only** the active `codexi.dat` file, primarily used for quick rollback.

| Command | Description | Example |
| :--- | :--- | :--- |
| `data snapshot` | Creates a timestamped copy of the current `codexi.dat` file. (Used before `import` or bulk changes). | `codexi data snapshot` |
| `data list` | Lists all available snapshots in the internal directory. | `codexi data list` |
| `data restore [filename]` | Restores the active ledger from a specific snapshot file. | `codexi data restore codexi_20251208_101727.snp` |
| `data clean` | Remove old snapshot files, keeping only the 5 most recent ones by default. | `codexi data clean` |

#### 5. Data Portability (Import & Export) - Active ledger only 

| Command | Description | Example |
| :--- | :--- | :--- |
| `data export json` | Exports the active ledger to JSON format. | `codexi data export json` |
| `data export toml` | Exports the active ledger to TOML format. | `codexi data export toml` |
| `data export csv` | Exports the active ledger to CSV format. | `codexi data export csv` |
| `data import json` | Imports operations to active ledger from a JSON file. | `codexi data import json` |
| `data import toml` | Imports operations to active ledger from a TOML file. | `codexi data import toml` |
| `data import csv` | Imports operations to active ledger from a CSV file. | `codexi data import csv ` |

### Maintenance Commands

⚠️ The commands **migrate** and **clear** should be used with caution. A backup is **strongly** recommended before to used it.

| Command | Description | Example |
| :--- | :--- | :--- |
| `maintenance migrate [version]` | Migrate the cureent ledger and all the associated archive files(.cld). | `codexi maintenance migrate 2` |
| `maintenance clear` | Delete the file codexi.dat, all the snaphot files (.snp) and all the archive files(.cld). | `codexi maintenance clear` |
| `maintenance ledger-infos` | Provide some information to the . | `codexi maintenance clear` |

- Current storage format version: **V2 (CBOR)**

## 📂 Import/Export Specifications

To ensure simplicity, Codexi uses **fixed filenames** for data exchange.

- **Export:** Creates a file named `codexi.[extension]` in your current folder.

- **Import:** Your source file must be named `codexi.[extension]` to be recognized.


To ensure simplicity and speed, Codexi uses **fixed filenames** for data exchange.

- **Export:** When you run an export command, Codexi creates a file named `codexi.[extension]` in your current folder.

- **Import:** To import data, you must name your source file `codexi.json`, `codexi.toml`, or `codexi.csv` before running the command.

ℹ️ Import / Export formats (JSON, TOML, CSV) are **interchange formats only**.

JSON and TOML exports include a logical `export_version` field (currently **V1**) to ensure forward compatibility of the data model.
They are not used for internal storage and do not include storage-level metadata (magic header, checksum, or binary format version).


**⚠️ Warning Data Integrity:** It is highly recommended to run `data snapshot` before performing an `import` operation. This allows you to roll back easily if the imported data contains errors or formatting issues.

## 📊 Financial Analytics Dashboard
Codexi includes a powerful statistics engine designed to give you a clear view of your financial health.

**Smart Filtering Logic**
To ensure data accuracy, the statistics engine applies an automated filtering layer:

- **Structural Exclusion:** System operations `INIT` (initialization) and `CLOSE` (period closing) are stricly excluded from all calculations to avoid bloating totals.

- **Behavioral Filtering:** `ADJUST` operations are excluded from behavioral metrics but still tracked for system health.

- **Void Semantics (`--net` flag):**
  - By default, VOID operations do **not** affect totals, and the original voided operation is ignored (historical view).
  - With `--net`, VOID operations are treated as real financial flows and included in totals at the date of the VOID, providing a net-impact view over a period.

- **Expense Analysis:** `ADJUST` (balancing) and `VOID` operations are automatically hidden from your **Top 5 Expenses** and **Max Single Expense** metrics to focus purely on your consumption behavior.

**Visual Indicators**
- **Savings Rate Bar:** A dynamic progress bar showing your capacity to save. It turns into a danger bar (`!!!`) if expenses exceed income.

- **Daily Burning Rate:** it tells you exactly how much you spend on average per day.

- **System Health:** Tracks the percentage of adjustments in your ledger to help you maintain high data integrity.

**Time-based statistics**
Codexi statistics are **strictly time-based**.

When an operation is voided, the reversal is recorded at the date of the VOID.
The original operation is not removed retroactively.
To compute the net financial impact within a specific period, use:
    `codexi report stats --from YYYY-MM-DD --to YYYY-MM-DD --net`

## 🛡️ Data Integrity Workflow

Codexi manages your data through three distinct layers of safety:
```text
[ Active Operations ] --(snapshot)--> [ snapshots/ (.snp) ]
           |
     (report stats) --------> [ Visual Dashboard ]
           |
     (system close)
           v
 [ archives/ (.cld) ] --(system backup)--> [ Full_Backup.zip ]
 ```
 
## 🗃️ Data Location

Codexi uses standard OS directories for storing its data to ensure compatibility and ease of access.

* **Active Ledger Format:** CBOR (versioned, checksummed)
* **Active Ledger:** `codexi.dat`
* **Archives:** `[Data Directory]/archives/`
* **Snapshots:** `[Data Directory]/snapshots/`

The exact data directory path varies by OS:

| OS | Path |
| :--- | :--- |
| **Linux** | `~/.local/share/fr.ethal.codexi/` |
| **macOS** | `~/Library/Application Support/fr.ethal.codexi/` |
| **Windows**| `%AppData%\Roaming\fr.ethal.codexi\` |

## 🧭 Versioning

- **Application version** follows Semantic Versioning (v2.0.0)
- **Storage version** is independent and currently at V2
- **Export formats** (JSON / TOML) use their own `export_version` (currently V1)

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome! Feel free to open an issue or submit a pull request on GitHub.

## 📄 License

This project is licensed under the MIT License.

## 📬 Author

    ethal <ethal@ethal.fr>
