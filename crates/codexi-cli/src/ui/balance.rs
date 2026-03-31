// src/ui.balance.rs
//
use thousands::Separable;

use codexi::logic::balance::BalanceItem;

use crate::ui::{CREDIT_STYLE, DEBIT_STYLE, TITLE_STYLE, VALUE_STYLE};

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
