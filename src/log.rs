//! Pretty logging with performance optimizations.
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

// Compiled regex patterns cached globally
struct RegexCache {
    ansi: Regex,
    string: Regex,
    number: Regex,
    boolean: Regex,
    key: Regex,
    bracket: Regex,
}

static REGEX_CACHE: OnceLock<RegexCache> = OnceLock::new();

fn get_regex_cache() -> &'static RegexCache {
    REGEX_CACHE.get_or_init(|| RegexCache {
        ansi: Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap(),
        string: Regex::new(r#"("[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')"#).unwrap(),
        number: Regex::new(r"\b\d+\.?\d*\b").unwrap(),
        boolean: Regex::new(r"\b(true|false|null|undefined|None|nil)\b").unwrap(),
        key: Regex::new(r"\b(\w+):").unwrap(),
        bracket: Regex::new(r"[\[\]{}()]").unwrap(),
    })
}

pub fn contains_ansi_codes(text: &str) -> bool {
    let cache = get_regex_cache();
    cache.ansi.is_match(text)
}

pub fn highlight_syntax(text: &str) -> String {
    if text.is_empty() || contains_ansi_codes(text) {
        return text.to_string();
    }

    let cache = get_regex_cache();
    let mut matches: BTreeMap<usize, (usize, String, u8)> = BTreeMap::new();

    // Priority 1: Strings (highest priority to avoid false matches inside strings)
    for mat in cache.string.find_iter(text) {
        let colored = format!("\x1b[92m{}\x1b[0m", mat.as_str());
        matches.insert(mat.start(), (mat.end(), colored, 1));
    }

    // Priority 2: Numbers
    for mat in cache.number.find_iter(text) {
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let colored = format!("\x1b[93m{}\x1b[0m", mat.as_str());
            matches.insert(mat.start(), (mat.end(), colored, 2));
        }
    }

    // Priority 3: Booleans/null values
    for mat in cache.boolean.find_iter(text) {
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let color = if matches!(mat.as_str(), "true" | "false") {
                "\x1b[93;1m"
            } else {
                "\x1b[90m"
            };
            let colored = format!("{}{}\x1b[0m", color, mat.as_str());
            matches.insert(mat.start(), (mat.end(), colored, 3));
        }
    }

    // Priority 4: Keys
    for cap in cache.key.captures_iter(text) {
        let mat = cap.get(0).unwrap();
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let key = cap.get(1).unwrap().as_str();
            let colored = format!("\x1b[96m{}:\x1b[0m", key);
            matches.insert(mat.start(), (mat.end(), colored, 4));
        }
    }

    // Priority 5: Brackets (lowest priority)
    for mat in cache.bracket.find_iter(text) {
        if !is_inside_match(&matches, mat.start(), mat.end()) {
            let colored = format!("\x1b[97m{}\x1b[0m", mat.as_str());
            matches.insert(mat.start(), (mat.end(), colored, 5));
        }
    }

    // Build result string with pre-allocated capacity
    let mut result = String::with_capacity(text.len() + matches.len() * 20);
    let mut last_end = 0;

    for (start, (end, replacement, _)) in matches {
        result.push_str(&text[last_end..start]);
        result.push_str(&replacement);
        last_end = end;
    }

    result.push_str(&text[last_end..]);
    result
}

#[inline]
fn is_inside_match(matches: &BTreeMap<usize, (usize, String, u8)>, start: usize, end: usize) -> bool {
    for (match_start, (match_end, _, _)) in matches.range(..=start) {
        if start < *match_end && end > *match_start {
            return true;
        }
    }
    false
}

// Pre-computed colored strings for categories as ANSI escape codes
static CATEGORY_DEBUG: OnceLock<String> = OnceLock::new();
static CATEGORY_INFO: OnceLock<String> = OnceLock::new();
static CATEGORY_WARN: OnceLock<String> = OnceLock::new();
static CATEGORY_ERROR: OnceLock<String> = OnceLock::new();
static CATEGORY_CRITICAL: OnceLock<String> = OnceLock::new();

pub fn category(level: &str) -> String {
    match level {
        "debug" => CATEGORY_DEBUG.get_or_init(|| "DEBG =>".bright_blue().bold().to_string()).clone(),
        "info" => CATEGORY_INFO.get_or_init(|| "INFO =>".bright_green().bold().to_string()).clone(),
        "warn" => CATEGORY_WARN.get_or_init(|| "WARN =>".bright_yellow().bold().to_string()).clone(),
        "error" => CATEGORY_ERROR.get_or_init(|| "EROR =>".bright_red().bold().to_string()).clone(),
        "critical" => CATEGORY_CRITICAL.get_or_init(|| "CRIT =>".bright_magenta().bold().to_string()).clone(),
        _ => level.normal().to_string(),
    }
}

thread_local! {
    static TIME_BUFFER: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(8));
}

pub fn time() -> String {
    TIME_BUFFER.with(|buf| {
        let mut buffer = buf.borrow_mut();
        buffer.clear();

        let now = chrono::Local::now();
        use std::fmt::Write;
        write!(buffer, "{}", now.format("%H:%M:%S")).unwrap();

        format!("\x1b[90;1m{}\x1b[0m", buffer)
    })
}

pub fn dim(text: &str) -> String {
    format!("\x1b[2m{}\x1b[0m", text)
}

#[inline]
pub fn get_logging_level() -> LogLevel {
    let level = LEVEL.load(Ordering::Relaxed);
    LogLevel::from_u8(level).unwrap_or(LogLevel::Info)
}

#[inline]
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

#[inline]
pub fn should_log(level: LogLevel) -> bool {
    let current_level = LEVEL.load(Ordering::Relaxed);
    level.as_u8() <= current_level
}

thread_local! {
    static CALLER_BUFFER: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(32));
}

pub fn get_caller_info() -> String {
    let caller = std::panic::Location::caller();
    let file = caller.file();

    CALLER_BUFFER.with(|buf| {
        let mut buffer = buf.borrow_mut();
        buffer.clear();

        let short_file = file
            .split('/')
            .last()
            .or_else(|| file.split('\\').last())
            .unwrap_or(file);

        use std::fmt::Write;
        write!(buffer, "{}:{}", short_file, caller.line()).unwrap();
        buffer.clone()
    })
}

#[macro_export]
macro_rules! pretext {
    ($cat:expr) => {{
        let cat = fox::log::category($cat);
        let time = fox::log::time();
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
            let highlighted_text = if text.len() > 1000 {
                text
            } else {
                fox::log::highlight_syntax(&text)
            };
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
            let highlighted_text = if text.len() > 1000 {
                text
            } else {
                fox::log::highlight_syntax(&text)
            };
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
