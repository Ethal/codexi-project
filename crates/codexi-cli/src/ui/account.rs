// src/ui/account.rs

use codexi::core::format_id_short;
use codexi::logic::codexi::{AccountEntry, AccountItem};

use crate::ui::{NOTE_STYLE, TITLE_STYLE};

/// view to list of account
pub fn view_account(items: &AccountEntry) {
    let title_text =
        TITLE_STYLE.apply_to("Accounts - <id> <short id> <name> <type> [currency] [bank]");
    println!();
    println!("{}", title_text);
    for item in items.items.iter() {
        let marker = if item.current {
            "(*)"
        } else if item.close {
            "(c)"
        } else {
            "   "
        };
        println!(
            " {} {} {} - {} {} {} {}",
            marker,
            item.id,
            format_id_short(&item.id),
            item.name,
            item.context.account_type,
            item.currency,
            item.bank
        );
    }
    println!();
    println!(
        "{}",
        NOTE_STYLE.apply_to("Note: (*) Current account, (c) Close Account.")
    );
    println!();
}

/// view to context of the current account
pub fn view_account_context(item: &AccountItem) {
    let title_text_1 = TITLE_STYLE.apply_to("Account");
    let title_text_2 = TITLE_STYLE.apply_to("context");
    println!();
    println!(
        "{}({} {}) {}",
        title_text_1, item.name, item.currency, title_text_2
    );
    println!(" Account Type: {}", item.context.account_type);
    println!(" Overdraft limit: {}", item.context.overdraft_limit);
    println!(" Minimun balance: {}", item.context.min_balance);
    println!(
        " Deposit locked until: {}",
        item.context.deposit_locked_until
    );
    println!(
        " Max monthly transactions: {}",
        item.context.max_monthly_transactions
    );
    println!(" Allows interest: {}", item.context.allows_interest);
    println!(
        " Allows joint signers: {}",
        item.context.allows_joint_signers
    );
    println!();
}
