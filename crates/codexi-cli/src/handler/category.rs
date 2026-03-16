// src/handler/category.rs

use anyhow::Result;

use codexi::{core::DataPaths, file_management::FileManagement};

use crate::command::CategoryCommand;
use crate::ui::view_category;

pub fn handle_category_command(command: CategoryCommand, paths: &DataPaths) -> Result<()> {
    let codexi = FileManagement::load_current_state(paths)?;
    match command {
        CategoryCommand::List => {
            let items = codexi.categories.category_entry();
            view_category(&items);
        }
    }
    Ok(())
}
