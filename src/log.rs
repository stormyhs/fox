//! Logging, made pretty.
//!
//! ```rs
//! use fox::*;
//! let msg = "something bad happened!";
//! error!("An error occurred: {msg}");
//! ```

use colored::*;

pub static LEVEL: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(5);

pub enum LogLevel {
    Debug, Info, Warn, Error, Critical
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
/// Use the `LOG_LEVEL` constants for human readable usage.
///
/// ```rs
/// fox::log::set_logging_level(fox::log::cl::LogLevel::Error);
/// ```
pub fn set_logging_level(level: LogLevel) {
    match level {
        LogLevel::Debug => LEVEL.store(5, std::sync::atomic::Ordering::SeqCst),
        LogLevel::Info => LEVEL.store(4, std::sync::atomic::Ordering::SeqCst),
        LogLevel::Warn => LEVEL.store(3, std::sync::atomic::Ordering::SeqCst),
        LogLevel::Error => LEVEL.store(2, std::sync::atomic::Ordering::SeqCst),
        LogLevel::Critical => LEVEL.store(1, std::sync::atomic::Ordering::SeqCst),
    }
}

#[macro_export]
macro_rules! pretext {
    ($cat:expr) => {{
        let cat = fox::log::category($cat);
        let time = fox::log::dim(&fox::log::time());

        let caller = std::panic::Location::caller();
        let file = caller.file();

        let short_file = file.split_once('/').unwrap().1;
        let line = caller.line();
        let caller = fox::log::dim(&format!("{}:{}", short_file, line));

        format!("{} {} {}", cat, time, caller)
    }};
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => {
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
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
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
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
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
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
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
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
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
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
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 5 {
            let text = format!($($args)*);
            let cat = log::category("debug");
            println!("{} {}", cat, text);
        }
    };
}

#[macro_export]
macro_rules! sinfo {
    ($($args:tt)*) => {
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 4 {
            let text = format!($($args)*);
            let cat = log::category("info");
            println!("{} {}", cat, text);
        }
    };
}

#[macro_export]
macro_rules! swarn {
    ($($args:tt)*) => {
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 3 {
            let text = format!($($args)*);
            let cat = log::category("warn");
            println!("{} {}", cat, text);
        }
    };
}

#[macro_export]
macro_rules! serror {
    ($($args:tt)*) => {
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 2 {
            let text = format!($($args)*);
            let cat = log::category("error");
            println!("{} {}", cat, text);
        }
    };
}

#[macro_export]
macro_rules! scritical {
    ($($args:tt)*) => {
        let level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level >= 1 {
            let text = format!($($args)*);
            let cat = log::category("critical");
            println!("{} {}", cat, text);
        }
    };
}
