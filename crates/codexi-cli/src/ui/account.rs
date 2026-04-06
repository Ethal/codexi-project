// src/ui/account.rs

use thousands::Separable;

use codexi::core::{format_id_short, format_optional_u32, format_text, yes_no};
use codexi::dto::{AccountCollection, AccountItem};

use crate::ui::{
    CREDIT_STYLE, DEBIT_STYLE, NOTE_STYLE, STYLE_CAUTION, STYLE_DANGER, STYLE_HIGHLIGHT,
    STYLE_MUTED, TITLE_STYLE, VALUE_STYLE, format_optional_bank_item,
    format_optional_currency_item, truncate_text,
};

/// view to list of account
pub fn view_account(items: &AccountCollection) {
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
            format_optional_currency_item(&item.currency),
            format_optional_bank_item(&item.bank),
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
        item.name,
        format_optional_currency_item(&item.currency)
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
        format_optional_u32(item.context.max_monthly_transactions)
    );
    println!(" Allows interest: {}", yes_no(item.context.allows_interest));
    println!(
        " Allows joint signers: {}",
        yes_no(item.context.allows_joint_signers)
    );
    println!();
}

/// Account overview
pub fn overview_account(account: &AccountCollection) {
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
            (false, false) => STYLE_MUTED.apply_to(id_txt),
            (true, false) => STYLE_HIGHLIGHT.apply_to(id_txt),
            (false, true) => STYLE_DANGER.apply_to(id_txt),
            (true, true) => STYLE_CAUTION.apply_to(id_txt),
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
            truncate_text(&format_optional_bank_item(&item.bank), 9),
            format_optional_currency_item(&item.currency),
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
    let cu_acc = STYLE_HIGHLIGHT.apply_to("#XXXXX");
    let cl_acc = STYLE_DANGER.apply_to("#XXXXX");
    let cu_cl_acc = STYLE_CAUTION.apply_to("#XXXXX ⚠");
    println!(
        "{} Current account, {} Closed account, {} Current but closed",
        cu_acc, cl_acc, cu_cl_acc
    );
    println!();
}
