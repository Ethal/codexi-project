// src/lib.rs

pub mod core;
pub mod types;
pub mod exchange;
pub mod file_management;
pub mod logic;

pub const CODEXI_MAGIC: [u8; 6] = *b"CODEXI";
// V1 to V2 : add FileEnvelope and add next_op_id in codexi structure.
// V2 to V3 : implementation multi account.
pub const CODEXI_DATA_FORMAT_VERSION: u16 = 3;
// V1 to V2 : implementation multi account.
pub const CODEXI_EXCHANGE_FORMAT_VERSION: u16 = 2;
