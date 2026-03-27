// src/ui.balance.rs
//
use thousands::Separable;

use codexi::{
    core::format_id_short,
    logic::balance::{BalanceItem, CodexiBalanceEntry},
};

use crate::ui::{CREDIT_STYLE, DEBIT_STYLE, LABEL_STYLE, TITLE_STYLE, VALUE_STYLE};

/// view the balance (credit/debit/balance)
pub fn view_balance(balance: &BalanceItem) {
    let title_text = TITLE_STYLE.apply_to("codexi balance summary");
    let credit_value =
        CREDIT_STYLE.apply_to(format!("{:.2}", balance.credit).separate_with_commas());
    let debit_value = DEBIT_STYLE.apply_to(format!("{:.2}", balance.debit).separate_with_commas());
    let balance_value =
        VALUE_STYLE.apply_to(format!("{:.2}", balance.total).separate_with_commas());

    println!();
    println!("{}", title_text);
    println!(" {:<10}{:>18}", "Credit:", credit_value);
    println!(" {:<10}{:>18}", "Debit:", debit_value);
    println!(" {:<10}{:>18}", "Balance:", balance_value);
    println!();
}

pub fn view_balance_account(balance: &CodexiBalanceEntry) {
    println!(
        "┌───────┬──────────────────┬──────────────────┬──────────────────┬──────────────────┐"
    );
    println!(
        "│Id     │Account           │             Debit│            Credit│           Balance│"
    );
    println!(
        "├───────┼──────────────────┼──────────────────┼──────────────────┼──────────────────┤"
    );

    for bal in &balance.balances {
        let id_txt = LABEL_STYLE.apply_to(format!("#{}", format_id_short(&bal.id)));
        let deb_txt =
            DEBIT_STYLE.apply_to(format!("{:.2}", bal.balance.debit).separate_with_commas());
        let cre_txt =
            CREDIT_STYLE.apply_to(format!("{:.2}", bal.balance.credit).separate_with_commas());
        let bal_txt =
            VALUE_STYLE.apply_to(format!("{:.2}", bal.balance.total).separate_with_commas());
        println!(
            "│{:<7}│{:<18}│{:>18}│{:>18}│{:>18}│",
            id_txt, bal.name, deb_txt, cre_txt, bal_txt,
        );
    }
    println!(
        "└───────┴──────────────────┴──────────────────┴──────────────────┴──────────────────┘"
    );
    println!();
}
