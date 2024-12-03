use colored::*;

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

#[macro_export]
macro_rules! pretext {
    ($cat:expr) => {{
        let cat = log::cl::category($cat);
        let time = log::cl::dim(&log::cl::time());

        let caller = std::panic::Location::caller();
        let file = caller.file();

        let short_file = file.split_once('/').unwrap().1;
        let line = caller.line();
        let caller = log::cl::dim(&format!("{}:{}", short_file, line));

        format!("{} {} {}", cat, time, caller)
    }};
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let pre = pretext!("debug");
        println!("{} {}", pre, text);
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let pre = pretext!("info");
        println!("{} {}", pre, text);
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let pre = pretext!("warn");
        println!("{} {}", pre, text);
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let pre = pretext!("error");
        println!("{} {}", pre, text);
    };
}

#[macro_export]
macro_rules! critical {
    ($($args:tt)*) => {
        let text = format!($($args)*);
        let pre = pretext!("critical");
        println!("{} {}", pre, text);
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
