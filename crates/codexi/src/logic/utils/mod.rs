// src/logic/utils/mod.rs

mod resolve_id;

pub use resolve_id::{HasName, HasNulid, ResolveError, resolve_by_id_or_name, resolve_id};
