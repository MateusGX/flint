//! Global log-level configuration for the Flint runtime.
//!
//! The CLI reads `server.log` from `flint.toml` and calls [`set`] once at
//! startup. Every other part of the runtime (dispatch, router) reads [`get`]
//! to decide what to emit.

use std::sync::atomic::{AtomicU8, Ordering};

static LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Info as u8);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

impl std::str::FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "error" => Ok(Self::Error),
            "warn" => Ok(Self::Warn),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            _ => Err(()),
        }
    }
}

pub fn set(level: LogLevel) {
    LEVEL.store(level as u8, Ordering::Relaxed);
}

pub fn get() -> LogLevel {
    match LEVEL.load(Ordering::Relaxed) {
        0 => LogLevel::Off,
        1 => LogLevel::Error,
        2 => LogLevel::Warn,
        4 => LogLevel::Debug,
        _ => LogLevel::Info,
    }
}

pub const BOLD: &str = "\x1b[1m";
pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[2m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const CYAN: &str = "\x1b[36m";

pub fn status_color(status: u16) -> &'static str {
    if status < 300 {
        GREEN
    } else if status < 400 {
        CYAN
    } else if status < 500 {
        YELLOW
    } else {
        RED
    }
}
