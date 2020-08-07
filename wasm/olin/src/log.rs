use crate::sys;
use log::{Level as LogLevel, LevelFilter, Metadata, Record, SetLoggerError};

/// See Level enum defined in https://github.com/CommonWA/cwa-spec/blob/master/ns/log.md#write
#[repr(i32)]
#[derive(Debug)]
/// Logging facade for olin. This will be exposed with the [log](https://docs.rs/log)
/// crate, but you can do this manually if you really want.

pub enum Level {
    Error = 1,
    Debug = 2,
    Warning = 3,
    Trace = 4,
    Info = 6,
}

impl From<LogLevel> for Level {
    fn from(ll: LogLevel) -> Level {
        match ll {
            LogLevel::Error => Level::Error,
            LogLevel::Warn => Level::Warning,
            LogLevel::Info => Level::Info,
            LogLevel::Debug => Level::Debug,
            LogLevel::Trace => Level::Trace,
        }
    }
}

/// Writes a line of text with the specified level to the host logger.
pub fn write(level: Level, text: &str) {
    let text = text.as_bytes();
    unsafe { sys::log_write(level as i32, text.as_ptr(), text.len()) }
}

/// Convenience wrapper for the error level.
pub fn error(text: &str) {
    write(Level::Error, text)
}

/// Convenience wrapper for the warning level.
pub fn warning(text: &str) {
    write(Level::Warning, text)
}

/// Convenience wrapper for the info level.
pub fn info(text: &str) {
    write(Level::Info, text)
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        write(
            record.level().into(),
            &format!(r#"{} -- {}"#, record.target(), record.args()),
        )
    }

    fn flush(&self) {}
}

pub(crate) static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
