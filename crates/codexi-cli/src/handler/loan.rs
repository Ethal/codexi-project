// src/handler/loan.rs

use anyhow::Result;

use chrono::Duration;
use codexi::{
    core::{DataPaths, parse_date, parse_decimal},
    logic::loan::{CompoundInterest, LinearInterest, Loan, LoanKind, LoanPolicySettings},
};

use crate::{
    command::{LoanCommand, LoanPolicyAction},
    ui::{view_loan_policy_setting, view_loan_summary},
};

pub fn handle_loan_command(command: LoanCommand, paths: &DataPaths) -> Result<()> {
    let tmp_dir = &paths.tmp_dir;
    match command {
        LoanCommand::Policy { action } => match action {
            LoanPolicyAction::Reset => {
                LoanPolicySettings::reset(tmp_dir)?;
            }
            LoanPolicyAction::Show => {
                let loan_policy_settings = LoanPolicySettings::load_or_create(tmp_dir);
                view_loan_policy_setting(&loan_policy_settings);
            }
            LoanPolicyAction::Set {
                type_interest,
                rate,
                free_days,
                max_cap,
                max_days,
                min_capital,
                max_penalty,
            } => {
                let mut loan_policy_settings = LoanPolicySettings::load_or_create(tmp_dir);
                if let Some(v) = type_interest {
                    loan_policy_settings.type_interest = v;
                }
                if let Some(v) = rate {
                    loan_policy_settings.rate = v;
                }
                if let Some(v) = free_days {
                    loan_policy_settings.free_days = v;
                }
                if let Some(v) = max_cap {
                    loan_policy_settings.max_cap = Some(v);
                }
                if let Some(v) = max_days {
                    loan_policy_settings.max_days = Some(v);
                }
                if let Some(v) = min_capital {
                    loan_policy_settings.min_capital = Some(v);
                }
                if let Some(v) = max_penalty {
                    loan_policy_settings.max_penalty = Some(v);
                }
                loan_policy_settings.save(tmp_dir)?;
            }
        },

        LoanCommand::Simulate {
            capital,
            start,
            refund,
            type_interest,
            rate,
            free_days,
        } => {
            let loan_policy_settings = LoanPolicySettings::load_or_create(tmp_dir);
            let loan_policy = loan_policy_settings.to_loan_policy()?;
            let capital_d = parse_decimal(&capital, "capital")?;
            let start_n = parse_date(&start)?;
            let refund_n = parse_date(&refund)?;
            let rate_d = match rate {
                Some(v) => parse_decimal(&v, "rate")?,
                None => parse_decimal(&loan_policy_settings.rate, "rate")?,
            };
            let free_days_n = match free_days {
                Some(v) => Duration::days(v as i64),
                None => Duration::days(loan_policy_settings.free_days as i64),
            };
            let type_interest_n = match type_interest {
                Some(v) => v.as_str().parse()?,
                None => loan_policy_settings.type_interest.as_str().parse()?,
            };

            let loan = match type_interest_n {
                LoanKind::Compound => {
                    let compound = CompoundInterest::new(free_days_n, capital_d, rate_d, loan_policy)?;
                    Loan::Compound(compound)
                }
                LoanKind::Linear => {
                    let linear = LinearInterest::new(free_days_n, capital_d, rate_d, loan_policy)?;
                    Loan::Linear(linear)
                }
            };

            let loan_summary = loan.amount_due(start_n, refund_n)?;
            view_loan_summary(&loan_summary);
        }
    }
    Ok(())
}
