// src/ui/account.rs

use thousands::Separable;

use codexi::core::{format_id_short, format_text};
use codexi::logic::codexi::{AccountEntry, AccountItem};

use crate::ui::{
    CREDIT_STYLE, DEBIT_STYLE, LABEL_STYLE, NOTE_STYLE, TITLE_STYLE, VALUE_STYLE, truncate_text,
};

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

pub fn overview_account(account: &AccountEntry) {
    println!(
        "┌───────┬──────────────────┬──────────┬──────────┬──────────┬──────────────────┬──────────────────┬──────────────────┐"
    );
    println!(
        "│Id     │Account           │Type      │Bank      │Currency  │             Debit│            Credit│           Balance│"
    );
    println!(
        "├───────┼──────────────────┼──────────┼──────────┼──────────┼──────────────────┼──────────────────┼──────────────────┤"
    );

    for item in &account.items {
        let id_txt = format!("#{}", format_id_short(&item.id));
        let id_txt_fmt = match (item.current, item.close) {
            (false, false) => LABEL_STYLE.apply_to(id_txt),
            (true, false) => VALUE_STYLE.apply_to(id_txt),
            (false, true) => DEBIT_STYLE.apply_to(id_txt),
            (true, true) => VALUE_STYLE.apply_to(id_txt),
        };

        let deb_txt =
            DEBIT_STYLE.apply_to(format!("{:.2}", item.balance.debit).separate_with_commas());
        let cre_txt =
            CREDIT_STYLE.apply_to(format!("{:.2}", item.balance.credit).separate_with_commas());
        let bal_txt =
            VALUE_STYLE.apply_to(format!("{:.2}", item.balance.total).separate_with_commas());
        println!(
            "│{:<7}│{:<18}│{:<10}│{:<10}│{:<10}│{:>18}│{:>18}│{:>18}│",
            id_txt_fmt,
            truncate_text(&item.name, 17),
            format_text(&item.context.account_type),
            truncate_text(&format_text(&item.bank), 9),
            format_text(&item.currency),
            deb_txt,
            cre_txt,
            bal_txt,
        );
    }
    println!(
        "└───────┴──────────────────┴──────────┴──────────┴──────────┴──────────────────┴──────────────────┴──────────────────┘"
    );
    println!();
    println!("{}", NOTE_STYLE.apply_to("Note:"));
    let cu_acc = VALUE_STYLE.apply_to("#XXXXX");
    let cl_acc = DEBIT_STYLE.apply_to("#XXXXX");
    println!("{} Current account, {} Close account", cu_acc, cl_acc);
    println!();
}
