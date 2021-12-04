// For SyncLazy
#![feature(once_cell)]

mod log_level;

pub use log_level::LogLevel;
use std::sync::Mutex;
use std::time::SystemTime;
use std::{fs::File, lazy::SyncLazy};
use std::{fs::OpenOptions, io::Write};
use time::macros::format_description;
use time::OffsetDateTime;

#[macro_export]
macro_rules! trace {
    ($message:expr) => {
        log!(LogLevel::Trace, $message.to_string())
    };
    ($message:expr, $($arg:expr),+) => {
        log!(LogLevel::Trace, format!($message, $($arg),+))
    };
}

#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        log!(LogLevel::Debug, $message.to_string())
    };
    ($message:expr, $($arg:expr),+) => {
        log!(LogLevel::Debug, format!($message, $($arg),+))
    };
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        log!(LogLevel::Info, $message.to_string())
    };
    ($message:expr, $($arg:expr),+) => {
        log!(LogLevel::Info, format!($message, $($arg),+))
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        log!(LogLevel::Warn, $message.to_string())
    };
    ($message:expr, $($arg:expr),+) => {
        log!(LogLevel::Warn, format!($message, $($arg),+))
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr) => {
        log!(LogLevel::Error, $message.to_string())
    };
    ($message:expr, $($arg:expr),+) => {
        log!(LogLevel::Error, format!($message, $($arg),+))
    };
}

#[macro_export]
macro_rules! fatal {
    ($message:expr) => {
        log_fatal!($message.to_string())
    };
    ($message:expr, $($arg:expr),+) => {
        log_fatal!(format!($message, $($arg),+))
    };
}

#[macro_export]
macro_rules! log {
    ($log_level:expr, $message:expr) => {
        logger::log(file!(), line!(), $log_level, $message);
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($message:expr) => {
        logger::log_fatal(file!(), line!(), $message);
    };
}

pub static LOG_LEVEL: SyncLazy<Mutex<LogLevel>> = SyncLazy::new(|| Mutex::new(LogLevel::Trace));
static FILE: SyncLazy<Mutex<File>> = SyncLazy::new(|| {
    Mutex::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("koala_chess.log")
            .unwrap(),
    )
});

pub fn set_log_level(log_level: LogLevel) {
    *LOG_LEVEL.lock().unwrap() = log_level;
}

pub fn log(file: &str, line: u32, log_level: LogLevel, message: String) {
    let formatted_message = format_message(file, line, log_level, message);

    log_to_file(&formatted_message);

    let global_log_level = LOG_LEVEL.lock().unwrap();

    if log_level >= *global_log_level {
        if log_level != LogLevel::Error {
            println!("{}", formatted_message);
        } else {
            eprintln!("{}", formatted_message);
        }
    }
}

pub fn log_fatal(file: &str, line: u32, message: String) -> ! {
    let formatted_message = format_message(file, line, LogLevel::Fatal, message);

    log_to_file(&formatted_message);

    panic!("{}", formatted_message);
}

fn format_message(file: &str, line: u32, log_level: LogLevel, message: String) -> String {
    let system_time = SystemTime::now();
    let date_time: OffsetDateTime = system_time.into();
    let format =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");
    let formatted_date_time = date_time.format(format).unwrap();

    format!(
        "[{}] [{}:{}] [{:?}] {}",
        formatted_date_time, file, line, log_level, message
    )
}

fn log_to_file(full_message: &str) {
    FILE.lock()
        .unwrap()
        .write_fmt(format_args!("{}\n", full_message))
        .unwrap();
}
