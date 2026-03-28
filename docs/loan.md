# Loan

Codexi supports microloan tracking through a dedicated `Loan` account type.
Each borrower gets their own `Loan` account that records the outstanding debt,
interest accrual, and repayments. A shared `Interest Income` account accumulates
all interest received across loans.

## Account types

| Account | Type | Per |
| --- | --- | --- |
| Current | `Current` | Shared — your main cash account |
| Loan Mike, Loan Bob, … | `Loan` | One per borrower |
| Interest Income | `Income` | Shared — collects all interest received |

## Operation cycle

```
Current Account (shared)
   │
   │ transfer — loan disbursement (Debit on Current, Credit on Loan)
   ▼
Loan Account (per borrower)
   │
   ├─ interest — accrual (Credit on Loan: records interest due)
   │
   ├─ transfer — repayment (Debit on Loan, Credit on Current)
   │
   └─ transfer — interest payment (Debit on Loan, Credit on Interest Income)
   ▼
Interest Income (shared)
   │
   │ transfer — move interest to Current when needed
   ▼
Current Account (shared)
```

### Key design — interest accrual

Interest is recorded in two steps on the `Loan` account:

1. `interest <date> <amount> "<description>"` — accrues the interest as additional
   debt (Credit on Loan). The balance increases by the interest amount.
2. `transfer <date> <amount> <amount> interest "<description>"` — moves the interest
   payment from the Loan account to Interest Income (Debit on Loan, Credit on Interest Income).

This approach keeps the `Loan` balance accurate at all times: it reflects the total
amount still owed (principal + accrued unpaid interest). A `Loan` account can never
go negative — the compliance policy enforces this.

To forgive interest already accrued, use `void` on the `interest` operation instead
of a manual debit.

---

## Example — microloan cycle

Accounts are created and initialized beforehand with the correct account type.

### Initial state

| Account         | Balance   |
| --------------- | --------- |
| Current         | 2,000,000 |
| Loan Mike       | 0         |
| Loan Bob        | 0         |
| Interest Income | 0         |

---

### Loan disbursements

```bash
account use current

# First loan to Mike
transfer date1 1_000_000 1_000_000 mike "Loan Mike #1"

# Loan to Bob
transfer date1 500_000 500_000 bob "Loan Bob #1"

# Second loan to Mike
transfer date2 300_000 300_000 mike "Loan Mike #2"
```

| Account         | Balance   |
| --------------- | --------- |
| Current         | 200,000   |
| Loan Mike       | 1,300,000 |
| Loan Bob        | 500,000   |
| Interest Income | 0         |

---

### Repayment #1 from Mike with interest (20,000)

The interest is first accrued on the Loan account, then transferred to Interest Income.

```bash
account use mike

# Principal repayment — Loan Mike balance decreases
transfer date3 1_000_000 1_000_000 current "Repayment Mike #1"

# Interest accrual — Loan Mike balance increases by 20,000 (debt recorded)
interest date3 20_000 "Interest Mike #1"

# Interest payment — Loan Mike balance decreases, Interest Income increases
transfer date3 20_000 20_000 interest "Interest Mike #1"
```

| Account         | Balance   |
| --------------- | --------- |
| Current         | 1,200,000 |
| Loan Mike       | 300,000   |
| Loan Bob        | 500,000   |
| Interest Income | 20,000    |

---

### Partial repayment from Bob (200,000)

```bash
account use bob

transfer date4 200_000 200_000 current "Partial repayment Bob"
```

| Account         | Balance   |
| --------------- | --------- |
| Current         | 1,400,000 |
| Loan Mike       | 300,000   |
| Loan Bob        | 300,000   |
| Interest Income | 20,000    |

---

### Late interest accrual for Bob (15,000)

Bob is late — interest is accrued on his Loan account. The balance increases to
reflect the total amount now owed (300,000 principal + 15,000 interest).

```bash
account use bob

interest date4 15_000 "Late interest Bob"
```

| Account         | Balance   |
| --------------- | --------- |
| Current         | 1,400,000 |
| Loan Mike       | 300,000   |
| Loan Bob        | 315,000   |
| Interest Income | 20,000    |

---

### Final repayment from Bob (principal + interest)

```bash
account use bob

# Principal repayment
transfer date5 300_000 300_000 current "Final repayment Bob"

# Interest payment
transfer date5 15_000 15_000 interest "Late interest Bob"
```

| Account         | Balance   |
| --------------- | --------- |
| Current         | 1,700,000 |
| Loan Mike       | 300,000   |
| Loan Bob        | 0         |
| Interest Income | 35,000    |

---

### Move interest income to Current (partial)

```bash
account use interest

transfer date5 20_000 20_000 current "Interest to current"
```

| Account         | Balance   |
| --------------- | --------- |
| Current         | 1,720,000 |
| Loan Mike       | 300,000   |
| Loan Bob        | 0         |
| Interest Income | 15,000    |
