// src/ui/loan.rs

use chrono::Duration;
use thousands::Separable;

use codexi::logic::loan::{LoanPolicySettings, LoanSummary};

use crate::ui::{LABEL_STYLE, TITLE_STYLE, VALUE_STYLE};

pub fn view_loan_policy_setting(data: &LoanPolicySettings) {
    println!();
    println!("{}", TITLE_STYLE.apply_to("Loan policy setting"));
    println!();
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("type of interest type:"),
        VALUE_STYLE.apply_to(&data.type_interest)
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Daily Rate of the loan:"),
        VALUE_STYLE.apply_to(&data.rate)
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Free days:"),
        VALUE_STYLE.apply_to(data.free_days)
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Max days for a loan:"),
        VALUE_STYLE.apply_to(&data.max_days.unwrap_or_default())
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Max interest cap (% of capital):"),
        VALUE_STYLE.apply_to(&data.max_cap.clone().unwrap_or_default())
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Max penality (% of capital, applied to the final due):"),
        VALUE_STYLE.apply_to(data.max_penalty.clone().unwrap_or_default())
    );
    println!(
        "{} {:?}",
        LABEL_STYLE.apply_to("Min capital to loaned:"),
        VALUE_STYLE.apply_to(&data.min_capital.clone().unwrap_or_default())
    );
    println!();
}

pub fn view_loan_summary(data: &LoanSummary) {
    println!();
    println!("{}", TITLE_STYLE.apply_to("Loan summary"));
    println!();
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Amount due:"),
        VALUE_STYLE.apply_to(data.final_due.separate_with_commas())
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Total interest:"),
        VALUE_STYLE.apply_to(data.total_interest.separate_with_commas())
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("Start date:"),
        VALUE_STYLE.apply_to(data.start_date)
    );
    println!(
        "{} {}",
        LABEL_STYLE.apply_to("First interest date:"),
        VALUE_STYLE.apply_to(data.first_interest_date)
    );

    if !data.cumulative_interest.is_empty() {
        println!();
        println!("{}", TITLE_STYLE.apply_to("Interest per late day"));
        println!();
        let mut current_date = data.first_interest_date;
        for day_interest in data.cumulative_interest.iter() {
            println!(
                "{} {} {}",
                LABEL_STYLE.apply_to("interest"),
                LABEL_STYLE.apply_to(current_date.to_string()),
                VALUE_STYLE.apply_to(day_interest.separate_with_commas()),
            );
            current_date += Duration::days(1);
        }
    }
    println!();
}
