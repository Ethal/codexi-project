// src/command/category.rs

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct CategoryArgs {
    #[command(subcommand)]
    pub command: CategoryCommand,
}

#[derive(Subcommand, Debug)]
pub enum CategoryCommand {
    /// List the categories
    List,
}
