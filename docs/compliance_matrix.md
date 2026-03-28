# Compliance matrix

## Legend

| Symbol | Meaning |
| :----: | :------ |
| ✓ | Always allowed |
| ✗ | Always refused |
| ~ | Conditional — see note |

---

## Regular operations

| Kind | Flow | Current | Joint | Business | Student | Saving | Deposit | Loan | Income |
| :--- | :--- | :-----: | :---: | :------: | :-----: | :----: | :-----: | :--: | :----: |
| **Transaction** | Credit | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ |
| | Debit | ~ ¹ | ~ ¹ | ~ ¹ | ~ ¹ | ✗ | ✗ | ✗ | ✗ |
| **Transfer** | Credit | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| | Debit | ~ ¹ | ~ ¹ | ~ ¹ | ~ ¹ | ~ ² | ~ ³ | ✓ | ~ ² |
| **Interest** | Credit | ✗ | ✗ | ✗ | ✗ | ~ ⁴ | ~ ⁴ | ~ ⁴ | ✗ |
| | Debit | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ |
| **Fee** | Credit | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |
| | Debit | ✓ | ✓ | ✓ | ✓ | ~ ² | ~ ³ | ~ ² | ✗ |
| **Refund** | Credit | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ |
| | Debit | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ |

**Notes:**

¹ **Overdraft** — allowed if `balance - amount >= -overdraft_limit`. Default limits:
Current 500, Joint 1,000, Business 10,000, Student 100.

² **No negative** — allowed if `balance - amount >= 0`. No overdraft permitted.

³ **Unlocked + no negative** — allowed only after `deposit_locked_until` date has passed,
and if `balance - amount >= 0`.

⁴ **`allows_interest`** — allowed only if the account context has `allows_interest = true`
(default: true for Saving, Deposit, Loan; false for all others).

**Loan specific rules:**
- `Transaction` and `Refund` Credit are refused — use `Transfer` for all cash movements.
- `Interest` Debit is refused — use `void` on the `Interest Credit` to forgive interest.
- `Fee` Credit and Debit are allowed (e.g. late fees, fee refunds).

**Deposit specific rules:**
- All credits (`Transaction`, `Transfer`, `Fee`, `Refund`) are always allowed regardless of lock date.
- All debits are blocked before `deposit_locked_until`. Only `Transfer` Debit is allowed after maturity.

**Refund Debit** is refused on all account types — use `void` on the original `Refund Credit` instead.

**Interest Debit** is refused on all account types — use `void` on the original `Interest Credit` instead.

---

## System operations

| Kind | Flow | Current | Joint | Business | Student | Saving | Deposit | Loan | Income |
| :--- | :--- | :-----: | :---: | :------: | :-----: | :----: | :-----: | :--: | :----: |
| **Init** | Credit / Debit | ~ ¹ | ~ ¹ | ~ ¹ | ~ ¹ | ~ ² | ~ ² | ✗ ⁵ | ~ ² |
| | None (amount = 0) | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Adjust** | Credit / Debit | ~ ¹ | ~ ¹ | ~ ¹ | ~ ¹ | ~ ² | ~ ² | ~ ² | ~ ² |
| **Checkpoint** | Credit / Debit / None | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Void** | Credit / Debit / None | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

**Notes:**

¹ **Overdraft** — see Regular note ¹ above.

² **No negative** — see Regular note ² above. Applies to Init and Adjust on no-overdraft types.

⁵ **Loan Init** — Init with a non-zero amount is refused on Loan accounts. A Loan account
always starts at 0. Init with `amount = 0` is allowed and required before any other operation.
