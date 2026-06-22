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
    /// List all categories
    List,

    /// Add a new category
    Add {
        /// Category name (1+ words)
        #[arg(
            value_name = "NAME",
            required = true,
            num_args = 1..,
            help = "Category name (accepts multiple words)"
        )]
        name: Vec<String>,

        /// Parent category
        #[arg(
            long = "parent",
            value_name = "PARENT_ID",
            help = "Parent category ID, short ID, or name (optional)"
        )]
        parent_id: Option<String>,

        /// Optional note
        #[arg(long, value_name = "NOTE", help = "Note about the category")]
        note: Option<String>,
    },

    /// [WARN] Terminate a category
    Terminate {
        /// Category ID or name
        #[arg(value_name = "ID", required = true, help = "Category ID, short ID, or name")]
        id: String,
    },
}
