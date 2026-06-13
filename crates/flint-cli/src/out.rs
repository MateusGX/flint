//! Terminal output helpers — minimal ANSI without a crate dependency.

pub const BOLD: &str = "\x1b[1m";
pub const RESET: &str = "\x1b[0m";
pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";
pub const DIM: &str = "\x1b[2m";

pub fn created(path: impl std::fmt::Display) {
    println!("  {GREEN}+{RESET} {path}");
}

pub fn step(label: &str, detail: impl std::fmt::Display) {
    println!("  {DIM}{label:<12}{RESET} {detail}");
}

pub fn done(label: &str, detail: impl std::fmt::Display) {
    println!("  {BOLD}{GREEN}{label:<12}{RESET} {detail}");
}

pub fn error(msg: impl std::fmt::Display) {
    eprintln!("  {RED}error{RESET}  {msg}");
}
