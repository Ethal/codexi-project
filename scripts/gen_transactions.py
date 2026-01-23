#
# For testing purpose only
# Generate randomly codexi command for transactions(debit, credit) and adjust
#
import random
from datetime import date, timedelta

# Parameters
start_date = date(2025, 1, 1)
end_date = date(2025, 3, 31)
max_transactions_per_day = 10  # minimum 5 transactions par jour
min_system_cmd = 30
max_system_cmd = 50
min_timedelta_day = 1
max_timedelta_day = 1

descriptions_debit = [
    "Groceries", "Transport", "Utilities", "Dining", "Entertainment",
    "Gift", "Insurance", "Rent", "Medical", "Shopping"
]

descriptions_credit = [
    "Bonus", "Salary", "Refund"
]

# Function to generate a transaction
def generate_transaction(d):
    type_tx = random.choices(["debit", "credit"], weights=[0.8, 0.2])[0]
    if type_tx == "debit":
        description = random.choice(descriptions_debit)
        amount = round(random.uniform(1, 50), 2)
    else:
        description = random.choice(descriptions_credit)
        amount = round(random.uniform(30, 80), 2)

    cmd = f'./target/release/codexi {type_tx} {d} {amount} "{description}"'
    return cmd

# Generate a system adjust command
def generate_system_adjust(d):
    amount = round(random.uniform(50.0, 100.0), 2)
    return f'./target/release/codexi system adjust {d} {amount}'

# Generate all dates
current_date = start_date
all_commands = []

init_amount = round(random.uniform(50.0, 100.0), 2)
cmd = f'./target/release/codexi system init {start_date} {init_amount}'
all_commands.append(cmd)

random_system_command = random.randint(min_system_cmd, max_system_cmd)


while current_date <= end_date:
    transactions_per_day = random.randint(1, max_transactions_per_day)
    timedelta_day = random.randint(min_timedelta_day, max_timedelta_day)
    for _ in range(transactions_per_day):
        all_commands.append(generate_transaction(current_date))

        random_system_command -= 1

        # If the counter reaches zero, the system adjust command is created.
        if random_system_command <= 0:
            all_commands.append(generate_system_adjust(current_date))
            # Counter Reset
            random_system_command = random.randint(min_system_cmd, max_system_cmd)


    current_date += timedelta(days=timedelta_day)

# Sauvegarder dans un fichier
with open("generate_transactions.sh", "w") as f:
    f.write("#!/bin/bash\n\n")
    for cmd in all_commands:
        f.write(cmd + "\n")

print(f"{len(all_commands)} Transactions generated for 2025 in generate_transactions.sh")
print(f"Performed a cargo build --release before launch the bash script")
