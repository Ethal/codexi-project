#
# For testing purpose only
# Generate randomly codexi commands for transactions (debit, credit) and adjust
# Counterparty and category are assigned from coherent tuples.
#
import random
from datetime import date, timedelta

# Parameters
start_date = date(2025, 1, 1)
end_date = date(2025, 5, 31)
max_transactions_per_day = 10  # maximun transactions per day
min_system_cmd = 30
max_system_cmd = 50
min_timedelta_day = 1
max_timedelta_day = 1

# Coherent tuples: (description, counterparty, category)
DEBIT_TUPLES = [
    ("Groceries",       "Supermarket",      "Groceries"),
    ("Bread and milk",  "Supermarket",      "Groceries"),
    ("Weekly shopping", "Supermarket",      "Groceries"),
    ("Dining out",      "Restaurant & Bar", "Restaurant & Bar"),
    ("Lunch",           "Restaurant & Bar", "Restaurant & Bar"),
    ("Drinks",          "Restaurant & Bar", "Leisure"),
    ("Fuel",            "Gas Station",      "Transport"),
    ("Bus ticket",      "Transport Service","Transport"),
    ("Taxi",            "Transport Service","Transport"),
    ("Ride hailing",    "Transport Service","Transport"),
    ("Electricity",     "Utilities",        "Housing"),
    ("Water bill",      "Utilities",        "Housing"),
    ("Internet",        "Telecom",          "Telecom"),
    ("Phone bill",      "Telecom",          "Telecom"),
    ("Rent",            "Landlord",         "Housing"),
    ("Doctor",          "Health Services",  "Health"),
    ("Pharmacy",        "Health Services",  "Health"),
    ("Sport session",   "Gym",              "Leisure"),
    ("Cinema",          "Entertainment",    "Leisure"),
    ("Event ticket",    "Entertainment",    "Leisure"),
    ("Clothes",         "Clothing Store",   "Clothing"),
    ("Shoes",           "Clothing Store",   "Clothing"),
    ("Books",           "Education",        "Education"),
    ("Online course",   "Education",        "Education"),
    ("Streaming",       "Subscriptions",    "Subscriptions"),
    ("Software",        "Subscriptions",    "Subscriptions"),
    ("Bank fee",        "Bank",             "Bank & Fees"),
    ("Tax payment",     "Government",       "Taxes"),
    ("Tobacco",         "Tobacco Shop",     "Tobacco"),
    ("Cigarettes",      "Tobacco Shop",     "Tobacco"),
    ("Gift",            "Family",           "Gift"),
    ("Tips",            "Family",           "Miscellaneous"),
    ("Tips",            "Friend",           "Miscellaneous"),
    ("Household help",  "Household Help",   "Miscellaneous"),
    ("Travel booking",  "Travel Agency",    "Travel"),
    ("Hotel",           "Travel Agency",    "Travel"),
    ("Donation",        "Church",           "Donations"),
]

CREDIT_TUPLES = [
    ("Salary",          "Employer",         "Salary"),
    ("Monthly salary",  "Employer",         "Salary"),
    ("Bonus",           "Employer",         "Side Income"),
    ("Freelance",       "Side Income",      "Side Income"),
    ("Refund",          "Refund Source",    "Refunds"),
    ("Reimbursement",   "Refund Source",    "Refunds"),
    ("Rent received",   "Landlord",         "Rental income"),
    ("Interest",        "Bank",             "Interest income"),
    ("Family transfer", "Family",           "Miscellaneous"),
    ("Friend transfer", "Friend",           "Miscellaneous"),
]


def generate_transaction(d: date) -> str:
    type_tx = random.choices(["debit", "credit"], weights=[0.8, 0.2])[0]
    if type_tx == "debit":
        desc, counterparty, category = random.choice(DEBIT_TUPLES)
        amount = round(random.uniform(1, 50), 2)
    else:
        desc, counterparty, category = random.choice(CREDIT_TUPLES)
        amount = round(random.uniform(30, 80), 2)
    return (
        f'./target/debug/codexi-cli {type_tx} {d} {amount} "{desc}"'
        f' -c "{counterparty}"'
        f' -g "{category}"'
    )


def generate_system_adjust(d: date) -> str:
    amount = round(random.uniform(50.0, 100.0), 2)
    return f'./target/debug/codexi-cli history adjust {d} {amount}'


# Build command list
all_commands = []

init_amount = round(random.uniform(50.0, 100.0), 2)
all_commands.append(f'./target/debug/codexi-cli account create {start_date} "testing"')
all_commands.append(f'./target/debug/codexi-cli history init {start_date} {init_amount}')

random_system_command = random.randint(min_system_cmd, max_system_cmd)
current_date = start_date

while current_date <= end_date:
    transactions_per_day = random.randint(min_timedelta_day, max_transactions_per_day)
    for _ in range(transactions_per_day):
        all_commands.append(generate_transaction(current_date))
        random_system_command -= 1

        # If the counter reaches zero, the system adjust command is created.
        if random_system_command <= 0:
            all_commands.append(generate_system_adjust(current_date))
            # Counter Reset
            random_system_command = random.randint(min_system_cmd, max_system_cmd)
    current_date += timedelta(days=random.randint(min_timedelta_day, max_timedelta_day))

# Write shell script
with open("generate_transactions.sh", "w") as f:
    f.write("#!/bin/bash\n\n")
    for cmd in all_commands:
        f.write(cmd + "\n")

print(f"{len(all_commands)} commands generated in generate_transactions.sh")
print("Performed a cargo build before launching the bash script.")
