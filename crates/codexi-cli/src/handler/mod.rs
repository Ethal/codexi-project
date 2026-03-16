// src/handler/mod.rs

mod account;
mod admin;
mod bank;
mod category;
mod currency;
mod data;
mod history;
mod report;
mod root;

pub use root::handle_root_command;
