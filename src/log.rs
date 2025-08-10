//! Pretty logging.
//!
//! ```rs
//! use fox::*;
//! let msg = "something bad happened!";
//! error!("An error occurred: {msg}");
//! ```

use colored::*;
use std::{collections::BTreeMap, sync::atomic::{AtomicU8, Ordering}};
use regex::Regex;
use std::sync::OnceLock;

pub static LEVEL: AtomicU8 = AtomicU8::new(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
    Critical = 1,
    Error = 2,
    Warn = 3,
    Info = 4,
    Debug = 5,
}

impl LogLevel {
    /// Convert from u8 to LogLevel
    pub fn from_u8(level: u8) -> Option<LogLevel> {
        match level {
            1 => Some(LogLevel::Critical),
            2 => Some(LogLevel::Error),
            3 => Some(LogLevel::Warn),
            4 => Some(LogLevel::Info),
            5 => Some(LogLevel::Debug),
            _ => None,
        }
    }

    /// Convert LogLevel to u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Get the string representation of the log level
    pub fn as_str(self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Critical => "critical",
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "critical" | "crit" => Ok(LogLevel::Critical),
            _ => Err(()),
        }
    }
}

// Add this function to check if text contains ANSI codes
pub fn contains_ansi_codes(text: &str) -> bool {
    static ANSI_RE: OnceLock<Regex> = OnceLock::new();
    let ansi_re = ANSI_RE.get_or_init(|| {
        Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap()
    });

    ansi_re.is_match(text)
}

pub fn highlight_syntax(text: &str) -> String {
    if contains_ansi_codes(text) {
        return text.to_string();
    }

    use std::collections::BTreeMap;

    let mut matches: BTreeMap<usize, (usize, String, u8)> = BTreeMap::new();

    let string_re = Regex::new(r#"("[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')"#).unwrap();
    for mat in string_re.find_iter(text) {
        let colored = format!("\x1b[92m{}\x1b[0m", mat.as_str()); // bright green
        matches.insert(mat.start(), (mat.end(), colored, 1));
    }

    let number_re = Regex::new(r"\b\d+\.?\d*\b").unwrap();
    for mat in number_re.find_iter(text) {
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let colored = format!("\x1b[93m{}\x1b[0m", mat.as_str()); // bright yellow
            matches.insert(mat.start(), (mat.end(), colored, 2));
        }
    }

    let bool_re = Regex::new(r"\b(true|false|null|undefined|None|nil)\b").unwrap();
    for mat in bool_re.find_iter(text) {
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let color = if matches!(mat.as_str(), "true" | "false") {
                "\x1b[93;1m" // bright yellow + bold for booleans
            } else {
                "\x1b[90m" // gray for null values
            };
            let colored = format!("{}{}\x1b[0m", color, mat.as_str());
            matches.insert(mat.start(), (mat.end(), colored, 3));
        }
    }

    let key_re = Regex::new(r"\b(\w+):").unwrap();
    for cap in key_re.captures_iter(text) {
        let mat = cap.get(0).unwrap();
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let key = cap.get(1).unwrap().as_str();
            let colored = format!("\x1b[96m{}:\x1b[0m", key); // bright cyan
            matches.insert(mat.start(), (mat.end(), colored, 4));
        }
    }

    let bracket_re = Regex::new(r"[\[\]{}()]").unwrap();
    for mat in bracket_re.find_iter(text) {
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let colored = format!("\x1b[97m{}\x1b[0m", mat.as_str()); // bright white
            matches.insert(mat.start(), (mat.end(), colored, 5));
        }
    }

    let mut result = String::new();
    let mut last_end = 0;

    for (start, (end, replacement, _priority)) in matches {
        result.push_str(&text[last_end..start]);
        result.push_str(&replacement);
        last_end = end;
    }

    result.push_str(&text[last_end..]);

    result
}

fn is_inside_match(matches: &BTreeMap<usize, (usize, String, u8)>, start: usize, end: usize) -> bool {
    for (match_start, (match_end, _, _)) in matches {
        if start < *match_end && end > *match_start {
            return true;
        }
    }
    false
}

pub fn category(level: &str) -> ColoredString {
    match level {
        "debug" => "DEBG =>".bright_blue().bold(),
        "info" => "INFO =>".bright_green().bold(),
        "warn" => "WARN =>".bright_yellow().bold(),
        "error" => "EROR =>".bright_red().bold(),
        "critical" => "CRIT =>".bright_magenta().bold(),
        _ => level.normal(),
    }
}

pub fn time() -> ColoredString {
    let time = chrono::Local::now();
    let time = time.format("%H:%M:%S").to_string();
    time.bright_black().bold()
}

pub fn dim(text: &str) -> ColoredString {
    text.dimmed()
}

pub fn get_logging_level() -> LogLevel {
    let level = LEVEL.load(Ordering::Relaxed);
    LogLevel::from_u8(level).unwrap_or(LogLevel::Info)
}

pub fn set_logging_level(level: LogLevel) {
    LEVEL.store(level.as_u8(), Ordering::SeqCst);
}

pub fn set_logging_level_from_str(level: &str) -> Result<(), ()> {
    let level: LogLevel = level.parse()?;
    set_logging_level(level);
    Ok(())
}

pub fn set_logging_level_from_env() {
    if let Ok(level_str) = std::env::var("LOG_LEVEL") {
        let _ = set_logging_level_from_str(&level_str);
    }
}

pub fn should_log(level: LogLevel) -> bool {
    let current_level = LEVEL.load(Ordering::Relaxed);
    level.as_u8() <= current_level
}

pub fn get_caller_info() -> String {
    let caller = std::panic::Location::caller();
    let file = caller.file();

    let short_file = file
        .split('/')
        .last()
        .or_else(|| file.split('\\').last())
        .unwrap_or(file);

    let line = caller.line();
    format!("{}:{}", short_file, line)
}

#[macro_export]
macro_rules! pretext {
    ($cat:expr) => {{
        let cat = fox::log::category($cat);
        let time = fox::log::dim(&fox::log::time().to_string());
        let caller = fox::log::dim(&fox::log::get_caller_info());
        format!("{} {} {}", cat, time, caller)
    }};
}

#[macro_export]
macro_rules! log_impl {
    ($level:expr, $level_num:expr, $($args:tt)*) => {{
        let current_level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if current_level >= $level_num {
            let text = format!($($args)*);
            let highlighted_text = fox::log::highlight_syntax(&text);
            let pre = fox::pretext!($level);
            println!("{} {}", pre, highlighted_text);
        }
    }};
}

#[macro_export]
macro_rules! slog_impl {
    ($level:expr, $level_num:expr, $($args:tt)*) => {{
        let current_level = fox::log::LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if current_level >= $level_num {
            let text = format!($($args)*);
            let highlighted_text = fox::log::highlight_syntax(&text);
            let cat = fox::log::category($level);
            println!("{} {}", cat, highlighted_text);
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => {
        fox::log_impl!("debug", 5, $($args)*)
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        fox::log_impl!("info", 4, $($args)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        fox::log_impl!("warn", 3, $($args)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
        fox::log_impl!("error", 2, $($args)*)
    };
}

#[macro_export]
macro_rules! critical {
    ($($args:tt)*) => {
        fox::log_impl!("critical", 1, $($args)*)
    };
}

#[macro_export]
macro_rules! sdebug {
    ($($args:tt)*) => {
        fox::slog_impl!("debug", 5, $($args)*)
    };
}

#[macro_export]
macro_rules! sinfo {
    ($($args:tt)*) => {
        fox::slog_impl!("info", 4, $($args)*)
    };
}

#[macro_export]
macro_rules! swarn {
    ($($args:tt)*) => {
        fox::slog_impl!("warn", 3, $($args)*)
    };
}

#[macro_export]
macro_rules! serror {
    ($($args:tt)*) => {
        fox::slog_impl!("error", 2, $($args)*)
    };
}

#[macro_export]
macro_rules! scritical {
    ($($args:tt)*) => {
        fox::slog_impl!("critical", 1, $($args)*)
    };
}

