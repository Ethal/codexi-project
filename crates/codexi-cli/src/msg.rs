// src/msg.rs

use console::Style;

const INFO_STYLE: Style = Style::new().green();
const WARN_STYLE: Style = Style::new().true_color(255, 165, 0); //orange

pub fn info(msg: &str) {
    println!("{}", INFO_STYLE.apply_to(msg));
}

pub fn warning(msg: &str) {
    println!("{}", WARN_STYLE.apply_to(msg));
}

#[macro_export]
macro_rules! msg_info {
    ($fmt:literal $(, $arg:expr)*) => {
        $crate::msg::info(&format!($fmt $(, $arg)*))
    };
}

#[macro_export]
macro_rules! msg_warn {
    ($fmt:literal $(, $arg:expr)*) => {
        $crate::msg::warning(&format!($fmt $(, $arg)*))
    };
}
