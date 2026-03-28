# Context matrix

## Legend

| Symbol | Meaning |
| :----: | :------ |
| ✓ | Applicable — value is used |
| ✗ | Not applicable — ignored with a warning |
| ~ | Conditional — applied only when another condition is met |
| ∞ | Unlimited (`None`) |
| — | Not defined (no value) |

---

## Field applicability by account type

### `overdraft_limit`

Allows the balance to go below zero up to the configured limit. Not applicable to
no-overdraft account types — any attempt to set it is ignored with a warning.

| Account type | Applicability |
| :----------- | :-----------: |
| Current      | ✓ |
| Joint        | ✓ |
| Business     | ✓ |
| Student      | ✓ |
| Saving       | ✗ |
| Deposit      | ✗ |
| Loan         | ✗ |
| Income       | ✗ |

---

### `min_balance`

Minimum balance required after each Regular operation. Ignored when `overdraft_limit > 0`
since a negative balance is already permitted up to the overdraft limit.

| Account type | Applicability |
| :----------- | :-----------: |
| All types    | ~ (≥ 0, ignored if `overdraft_limit > 0`) |

---

### `max_monthly_transactions`

Maximum number of Regular operations per calendar month. `None` means unlimited.

| Account type | Applicability |
| :----------- | :-----------: |
| All types    | ✓ (`None` = unlimited) |

---

### `deposit_locked_until`

Date before which all debit operations are blocked. Only applicable to `Deposit` accounts.

| Account type | Applicability |
| :----------- | :-----------: |
| Deposit      | ✓ |
| All others   | ✗ |

---

### `allows_interest`

Enables `RegularKind::Interest` operations on the account. Automatically set from the
account type at creation — can be overridden individually via `account set-context`.

| Account type | Applicability |
| :----------- | :-----------: |
| Saving       | ✓ |
| Deposit      | ✓ |
| Loan         | ✓ |
| Income       | ✓ |
| Current      | ✗ |
| Joint        | ✗ |
| Business     | ✗ |
| Student      | ✗ |

---

### `allows_joint_signers`

Marks the account as shared between multiple holders. Automatically set from the
account type at creation.

| Account type | Applicability |
| :----------- | :-----------: |
| Joint        | ✓ |
| Business     | ✓ |
| All others   | ✗ |

---

## Summary — default values by account type

| Field | Current | Joint | Business | Student | Saving | Deposit | Loan | Income |
| :---- | :-----: | :---: | :------: | :-----: | :----: | :-----: | :--: | :----: |
| `overdraft_limit` | ✓ 500 | ✓ 1,000 | ✓ 10,000 | ✓ 100 | ✗ 0 | ✗ 0 | ✗ 0 | ✗ 0 |
| `min_balance` | ~ 0 | ~ 0 | ~ 0 | ~ 0 | ~ 10 | ~ 0 | ~ 0 | ~ 0 |
| `max_monthly_tx` | ✓ ∞ | ✓ ∞ | ✓ ∞ | ✓ 30 | ✓ 6 | ✓ ∞ | ✓ ∞ | ✓ ∞ |
| `deposit_locked_until` | ✗ — | ✗ — | ✗ — | ✗ — | ✗ — | ✓ — | ✗ — | ✗ — |
| `allows_interest` | ✗ | ✗ | ✗ | ✗ | ✓ | ✓ | ✓ | ✓ |
| `allows_joint_signers` | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ |
