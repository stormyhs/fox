use colored::*;

pub static LEVEL: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(5);

pub mod LOG_LEVEL {
    pub const DEBUG: u8 = 5;
    pub const INFO: u8 = 4;
    pub const WARN: u8 = 3;
    pub const ERROR: u8 = 2;
    pub const CRITICAL: u8 = 1;
}

pub fn category(level: &str) -> ColoredString {
    let level = match level {
        "debug" => "DEBUG   ".bright_blue().bold(),
        "info" => "INFO    ".bright_green().bold(),
        "warn" => "WARN    ".bright_yellow().bold(),
        "error" => "ERROR   ".bright_red().bold(),
        "critical" => "CRITICAL".bright_magenta().bold(),
        _ => level.into()
    };


    format!("{}", level).into()
}

pub fn time() -> ColoredString {
    let time = chrono::Local::now();
    let time = time.format("%H:%M:%S").to_string();
    time.bright_black().bold()
}

pub fn dim(text: &str) -> ColoredString {
    text.dimmed()
}

/// Only print out logs from the given level and above.
///
/// Use the `cl::LOG_LEVEL` constants for human readable usage.
///
/// ```rs
/// fox::log::cl::set_logging_level(fox::log::cl::LOG_LEVEL::DEBUG);
/// ```
pub fn set_logging_level(level: u8) {
    LEVEL.store(level, std::sync::atomic::Ordering::Relaxed);
}

#[macro_export]
macro_rules! pretext {
    ($cat:expr) => {{
        let cat = fox::log::cl::category($cat);
        let time = fox::log::cl::dim(&fox::log::cl::time());

        let caller = std::panic::Location::caller();
        let file = caller.file();

        let short_file = file.split_once('/').unwrap().1;
        let line = caller.line();
        let caller = fox::log::cl::dim(&format!("{}:{}", short_file, line));

        format!("{} {} {}", cat, time, caller)
    }};
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => {
        let level = fox::log::cl::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 5 {
            let text = format!($($args)*);
            let pre = pretext!("debug");
            println!("{} {}", pre, text);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        let level = fox::log::cl::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 4 {
            let text = format!($($args)*);
            let pre = pretext!("info");
            println!("{} {}", pre, text);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        let level = fox::log::cl::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 3 {
            let text = format!($($args)*);
            let pre = pretext!("warn");
            println!("{} {}", pre, text);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
        let level = fox::log::cl::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 2 {
            let text = format!($($args)*);
            let pre = pretext!("error");
            println!("{} {}", pre, text);
        }
    };
}

#[macro_export]
macro_rules! critical {
    ($($args:tt)*) => {
        let level = fox::log::cl::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 1 {
            let text = format!($($args)*);
            let pre = pretext!("critical");
            println!("{} {}", pre, text);
        }
    };
}

#[macro_export]
macro_rules! sdebug {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let cat = log::cl::category("debug");
        println!("{} {}", cat, text);
    };
}

#[macro_export]
macro_rules! sinfo {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let cat = log::cl::category("info");
        println!("{} {}", cat, text);
    };
}

#[macro_export]
macro_rules! swarn {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let cat = log::cl::category("warn");
        println!("{} {}", cat, text);
    };
}

#[macro_export]
macro_rules! serror {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let cat = log::cl::category("error");
        println!("{} {}", cat, text);
    };
}

#[macro_export]
macro_rules! scritical {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let cat = log::cl::category("critical");
        println!("{} {}", cat, text);
    };
}
