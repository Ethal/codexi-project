// src/main.rs

#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::struct_excessive_bools)]

use anyhow::Result;
use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

use std::env;

mod command;
mod export;
mod handler;
mod msg;
mod prompts;
mod tui;
mod ui;

use codexi::core::DataPaths;
use codexi::logic::codexi::CodexiSettings;

use crate::command::Cli;
use crate::handler::handle_root_command;

pub fn clear_terminal() {
    let mut stdout = stdout();

    let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let settings = CodexiSettings::load_or_create()?;
    let paths = DataPaths::new(&settings.data_dir);
    let cwd = env::current_dir()?;

    if cli.tui {
        clear_terminal();
        tui::root::tui_root(&paths)?;
    } else {
        handle_root_command(cli, &paths, &cwd)?;
    }

    Ok(())
}
