// src/ui/category.rs

use codexi::core::format_id_short;
use codexi::dto::CategoryCollection;

use crate::ui::{LABEL_STYLE, TERMINATED_STYLE, TITLE_STYLE};

/// view to list the category
pub fn view_category(datas: &CategoryCollection) {
    let title_text = TITLE_STYLE.apply_to("Categories - <short id> <name> [parent] [note]");
    println!();
    println!("{}", title_text);
    for c in &datas.items {
        let id_style = match &c.terminated {
            Some(_) => TERMINATED_STYLE,
            None => LABEL_STYLE,
        };
        let parent_style = match &c.parent_terminated {
            Some(_) => TERMINATED_STYLE,
            None => LABEL_STYLE,
        };
        let id = id_style.apply_to(format!("#{}", format_id_short(&c.id)));
        let parent = match (&c.parent_name, &c.parent_id) {
            (Some(name), Some(pid)) => {
                let styled_pid = parent_style.apply_to(format!("({})", format_id_short(pid)));
                format!("{}{}", name, styled_pid)
            }
            _ => "─(—)".to_string(),
        };
        println!(
            " {} {:<20} {} {}",
            id,
            c.name,
            parent,
            c.note.clone().unwrap_or("─".to_string()),
        );
    }
}
