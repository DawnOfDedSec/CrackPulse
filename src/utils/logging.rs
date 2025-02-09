use std::fmt;

use chrono::Local;

pub struct LogManager {
    location: String,
}

impl LogManager {
    pub fn new(location: String) -> LogManager {
        LogManager { location }
    }

    pub fn ask(category: Option<&str>, msg: &str) {
        println!("[Crack-Pulse] [{}] : {}", category.unwrap_or("Input"), msg);
    }

    pub fn print(category: Option<&str>, msg: &str) {
        println!(
            "[Crack-Pulse] [{}] : ({}) -> {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            category.unwrap_or("INFO"),
            msg
        );
    }

    pub fn eprint<E: fmt::Debug + fmt::Display>(category: Option<&str>, err: E) {
        println!(
            "[Crack-Pulse] [{}] (Error | {}) -> {:?}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            category.unwrap_or("Others"),
            err
        )
    }
}
