// src/handler/category.rs

use anyhow::Result;

use codexi::{
    core::DataPaths,
    dto::CategoryCollection,
    file_management::FileManagement,
    logic::{
        category::{Category, CategoryError},
        utils::resolve_by_id_or_name,
    },
};

use crate::{command::CategoryCommand, ui::view_category};

pub fn handle_category_command(command: CategoryCommand, paths: &DataPaths) -> Result<()> {
    let mut codexi = FileManagement::load_current_state(paths)?;
    match command {
        CategoryCommand::List => {
            let items = CategoryCollection::build(&codexi.categories);
            view_category(&items);
        }
        CategoryCommand::Add {
            name,
            parent_id,
            note,
        } => {
            let name_n = name.join(" ");

            let parent_id_n = parent_id
                .map(|name| {
                    resolve_by_id_or_name::<Category, CategoryError>(&name, &codexi.categories.list)
                })
                .transpose()?;

            let note = note.map(|n| n.join(" "));
            let note_n = note.as_deref();
            codexi.categories.create(&name_n, parent_id_n, note_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
        }
        CategoryCommand::Terminate { id } => {
            let id_n =
                resolve_by_id_or_name::<Category, CategoryError>(&id, &codexi.categories.list)?;
            codexi.categories.terminate(&id_n)?;
            FileManagement::save_current_state(&codexi, paths)?;
        }
    }
    Ok(())
}
