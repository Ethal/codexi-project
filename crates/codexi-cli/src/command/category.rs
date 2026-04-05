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
    /// Add a category
    Add {
        /// Category name
        #[arg(value_name = "NAME", required = true, num_args = 1.., help = "Category name")]
        name: Vec<String>,

        /// Parent id of the category as needed
        #[arg(
            long = "parent",
            value_name = "PARENT_ID",
            help = "parent id of the category as needd, accept id\name"
        )]
        parent_id: Option<String>,

        /// Category note
        #[arg(long, value_name = "NOTE", num_args = 1.., help = "note of the category")]
        note: Option<Vec<String>>,
    },
    /// Teminate a category
    Terminate {
        /// Id of the category
        #[arg(value_name = "ID", required = true, help = "id\name of the category")]
        id: String,
    },
}
