// src/ui/account.rs

use thousands::Separable;

use codexi::core::format_id_short;
use codexi::logic::codexi::{AccountEntry, AccountItem};

use crate::ui::{NOTE_STYLE, TITLE_STYLE, truncate_text};

/// view to list of account
pub fn view_account(items: &AccountEntry) {
    let title_text =
        TITLE_STYLE.apply_to("Accounts - <id> <short id> <name> <type> [currency] [bank]");
    println!();
    println!("{}", title_text);
    for item in items.items.iter() {
        let marker = match (item.current, item.close) {
            (false, false) => "      ".to_string(),
            (true, false) => "   (*)".to_string(),
            (false, true) => "(c)   ".to_string(),
            (true, true) => "(c)(*)".to_string(),
        };
        println!(
            " {} {} {} - {:<10} {:<7} {} {}",
            marker,
            item.id,
            format_id_short(&item.id),
            truncate_text(&item.name, 10),
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
    let title_text = TITLE_STYLE.apply_to(format!(
        "name:{} currency:{} - Account context ",
        item.name, item.currency
    ));
    println!();
    println!("{}) ", title_text);
    println!(" Account Type: {}", item.context.account_type);
    println!(
        " Overdraft limit: {}",
        format!("{:.2}", item.context.overdraft_limit).separate_with_commas()
    );
    println!(
        " Minimun balance: {}",
        format!("{:.2}", item.context.min_balance).separate_with_commas()
    );
    println!(
        " Deposit locked until: {}",
        item.context
            .deposit_locked_until
            .clone()
            .unwrap_or("-".into())
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
