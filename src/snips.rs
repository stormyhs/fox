//! Simple CLI visual snippets

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{io, thread};
use std::time::Duration;
use std::io::Write;

use colored::Colorize;

pub struct Spinner {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            running: Arc::new(AtomicBool::new(true)),
            handle: None,
        }
    }

    pub fn start(&mut self, message: &str) {
        self.running = Arc::new(AtomicBool::new(true));
        let running = Arc::clone(&self.running);
        let message = message.to_string();

        let handle = thread::spawn(move || {
            let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let mut i = 0;

            while running.load(Ordering::Relaxed) {
                print!("\r{} {} ", spinner_chars[i], message);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                thread::sleep(Duration::from_millis(80));
                i = (i + 1) % spinner_chars.len();
            }

            print!("\r{}\r", " ".repeat(message.len() + 2));
        });

        self.handle = Some(handle);
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop();
    }
}

pub struct Loader {
    amount: u8,
    width: usize,
}

impl Loader {
    pub fn new() -> Self {
        Loader {
            amount: 0,
            width: 30,
        }
    }

    pub fn set_amount(&mut self, amount: u8) {
        let amount = amount.min(100);
        self.amount = amount;

        let filled_width = (amount as f32 / 100.0 * self.width as f32).round() as usize;

        print!("\r[");

        for i in 0..self.width {
            if i < filled_width {
                print!("█");
            } else {
                print!(" ");
            }
        }
        print!("] {}/100", amount);

        std::io::stdout().flush().unwrap();
    }

    pub fn clear(&mut self) {
        print!("\r{}\r", " ".repeat(self.width + 10));

        std::io::stdout().flush().unwrap();
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        self.clear();
    }
}

pub fn confirm<S: Into<String>>(message: S, default: bool) -> bool {
    let message = message.into();
    let default_hint = if default { "[Y/n]" } else { "[y/N]" };

    loop {
        print!("{} {} {} ", "INPUT =>".blue().bold(), message, default_hint);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_lowercase();

                if input.is_empty() {
                    return default;
                }

                match input.as_str() {
                    "y" | "yes" => return true,
                    "n" | "no" => return false,
                    _ => {
                        println!("Please enter 'y' or 'n'");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Failed to read input");
                return default;
            }
        }
    }
}

pub fn select<S: AsRef<str>>(message: S, options: &[S]) -> Option<usize> {
    if options.is_empty() {
        return None;
    }

    let message = message.as_ref();

    loop {
        println!("{} {}", "INPUT =>".blue().bold(), message);
        for (i, option) in options.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().cyan(), option.as_ref());
        }
        print!("Enter choice (1-{}): ", options.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if let Ok(choice) = input.parse::<usize>() {
                    if choice >= 1 && choice <= options.len() {
                        return Some(choice - 1);
                    }
                }
                println!("Invalid choice. Please enter a number between 1 and {}\n", options.len());
            }
            Err(_) => {
                println!("Failed to read input");
                return None;
            }
        }
    }
}

pub fn select_with_default<S: AsRef<str>>(message: S, options: &[S], default: usize) -> Option<usize> {
    if options.is_empty() || default >= options.len() {
        return None;
    }

    let message = message.as_ref();

    loop {
        println!("{} {}", "INPUT =>".blue().bold(), message);
        for (i, option) in options.iter().enumerate() {
            let prefix = if i == default {
                format!("  {}. {} {}", (i + 1).to_string().cyan(), option.as_ref(), "(default)".dimmed())
            } else {
                format!("  {}. {}", (i + 1).to_string().cyan(), option.as_ref())
            };
            println!("{}", prefix);
        }
        print!("Enter choice (1-{}) [{}]: ", options.len(), default + 1);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    return Some(default);
                }
                if let Ok(choice) = input.parse::<usize>() {
                    if choice >= 1 && choice <= options.len() {
                        return Some(choice - 1);
                    }
                }
                println!("Invalid choice. Please enter a number between 1 and {}\n", options.len());
            }
            Err(_) => {
                println!("Failed to read input");
                return Some(default);
            }
        }
    }
}
