#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
