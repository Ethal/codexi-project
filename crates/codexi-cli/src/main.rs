// src/main.rs

#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::struct_excessive_bools)]

use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use std::env;

mod command;
mod export;
mod handler;
mod msg;
mod prompts;
mod ui;

use codexi::core::DataPaths;
use codexi::logic::codexi::CodexiSettings;

use crate::command::Cli;
use crate::handler::handle_root_command;

fn init_logger(lvl: bool) {
    // Configuration of the logger
    let log_level = if lvl { LevelFilter::Debug } else { LevelFilter::Info };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp_millis()
        .init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // init logger, true = debug
    init_logger(true);

    let settings = CodexiSettings::load_or_create()?;
    let paths = DataPaths::new(&settings.data_dir);
    let cwd = env::current_dir()?;

    handle_root_command(cli, &paths, &cwd)?;
    Ok(())
}
