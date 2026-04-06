// src/prompts.rs

use anyhow::Result;
use console::style;
use dialoguer::{Confirm, Input};

pub struct Prompt;

impl Prompt {
    /// A standand confirmation
    pub fn confirm(message: &str, default: bool) -> Result<bool> {
        Ok(Confirm::new()
            .with_prompt(style(message).yellow().to_string())
            .default(default)
            .interact()?)
    }

    /// A confirmation with input texte requirement
    pub fn critical_confirm(action: &str, expected: &str) -> Result<bool> {
        println!("{}", style(format!("!!! DANGER: {} !!!", action)).red().bold());

        let input: String = Input::new()
            .with_prompt(format!(
                "Type {} to confirm (Leave empty to cancel)",
                style(expected).yellow().underlined()
            ))
            .allow_empty(true)
            .interact_text()?;

        Ok(input == expected)
    }
}
