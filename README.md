# 📔 Codexi CLI

**A high-integrity, append-only personal financial ledger**

Codexi is a command-line financial ledger focused on **auditability**, **traceability**, and **long-term data integrity**.

- Every financial operation is recorded permanently.
- Money movements are never edited or deleted after the fact.
- Corrections are performed through explicit compensating entries, preserving a complete and verifiable audit trail.


> Financial history is never rewritten.
>
> If a mistake is discovered, a new operation is recorded to correct it.


**The goal is simple:**

- Know where your money came from.
- Know where it went.
- Verify it years later.

---

## ✨ Key Features

- **Multi-Account** – Manage multiple accounts with per-account compliance rules (Current, Joint, Saving, Deposit, Loan, Business, Student, Income).
- **Anchor-Based Integrity** – Operation dates validated against history anchors (`INIT`, `CLOSE`, `ADJUST`) with same-day precision.
- **Period Closing & Archival** – Formally close periods into archive files with carried-forward balance.
- **Cross-Account Transfers** – Cross-currency transfers with automatic exchange rate calculation.
- **Exact Arithmetic** – Fixed-point decimal (`rust_decimal`), no floating-point errors.
- **Immutable Operations** – Financial fields (amount, date, kind, flow) are immutable; errors are corrected via explicit void operations.
- **Rich Analytics** – Savings rate, daily burn rate, top expenses, and system health metrics.
- **Data Safety** – Snapshots, backups, and trash recovery for disaster prevention.

---

## 💰 Balance Semantics

Codexi distinguishes between:
- **Global balance** – The actual account balance (use `overview`).
- **Filtered balance** – A running balance recalculated from the currently displayed operations (e.g., in `search`).
  *Note: Filtered balance does **not** reflect the real account balance.*

---

## 📚 Documentation

- [Loan & microloan management](docs/loan.md)
- [Compliance matrix](docs/compliance_matrix.md)
- [Context matrix](docs/context_matrix.md)

---

## 🛡️ Data Safety Layers

[ Active Ledger ]  --snapshot-->  [ snapshots/ (.snp) ]
       |
  system close
       |
[ archives/ (.cld) ]  --backup-->   [ backup.gz ]

Always run `codexi-cli data snapshot create` before imports or major changes.

---

## 🚀 Typical Workflow

```bash
# Initialize
codexi-cli account create 2025-01-01 "My Bank Account" --type Current
codexi-cli account set-bank <bank_id|name>
codexi-cli account set-currency <currency_id|code>
codexi-cli history init 2025-01-01 1500.00

# Record operations
codexi-cli credit  2025-01-05 2400.00 "Monthly salary" -c employer -g salary
codexi-cli debit   2025-01-06 45.00   "Groceries"     -c supermarket -g groceries
codexi-cli transfer 2025-01-10 100.00 95.00 wallet "ATM withdrawal" -g cash

# Analyze
codexi-cli overview
codexi-cli report financial --from 2025-01-01 --to 2025-01-31
codexi-cli report statement --open

# Backup before risky operations
codexi-cli data snapshot create
codexi-cli admin backup --target-dir ~/backups

```

---

## 📝 Real-world Usage

Codexi was designed around a very simple workflow:

1. Record operations.
2. Review balances and activity.
3. Generate reports.
4. Create backups before major changes.

In practice, most daily usage revolves around:

```bash
codexi-cli overview
codexi-cli report tree
codexi-cli report statement
```

Operations can be prepared in shell scripts and reviewed before execution,
making ledger updates reproducible and easy to audit.
