use colored::*;
use std::io::{self, Write};

pub fn info(message: &str) {
    println!("{} {}", "::".cyan(), message.bright_white().bold());
}

pub fn info_ext(message: &str, info: &str) {
    println!(" {} {} {}", "->".cyan(), message.bright_white(), info.cyan().bold());
}

pub fn info_arrow(message: &str) {
    println!(" {} {}", "->".cyan(), message.bright_white());
}

pub fn error_fatal(message: &str) {
    println!(" {} {}", "==> ERROR:".red(), message.bold().bright_white());
}

pub fn error_warn(message: &str) {
    println!("{} {}", "::".red(), message.bright_yellow());
}

pub fn line_clear() {
    print!("\x1B[2K\r");
}

pub fn flush() {
    let _ = io::stdout().flush();
}