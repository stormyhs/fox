//! Simple CLI visual snippets

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::Write;

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
            // Clear the spinner line when done
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
