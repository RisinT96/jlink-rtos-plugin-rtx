//! A logger that prints all messages to the GDB Server.

use chrono::Local;

/// Log crate
use log::{Level, Metadata, Record, SetLoggerError};

/// GDB Server api.
use crate::host::api;

struct GdbLogger {
    level: Level,
}

impl log::Log for GdbLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = iformat!("[{Local::now().format(\"%Y-%m-%d %H:%M:%S.%3f\")}] {record.level():<5} - {record.args()}\n");

        api::print(&message);
    }

    fn flush(&self) {}
}

/// Initializes the global logger with a GdbLogger instance with
/// `max_log_level` set to a specific log level.
///
/// ### Usage
/// ```
/// mod host;
///
/// fn main() {
///     host::log::init_with_level(log::Level::Warn).unwrap();
///
///     warn!("This is an example message.");
///     info!("This message will not be logged.");
/// }
/// ```
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = GdbLogger { level };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}

/// Initializes the global logger with a GdbLogger instance with
/// `max_log_level` set to `LogLevel::Trace`.
///
/// ### Usage
/// ```
/// mod host;
///
/// fn main() {
///     host::log::init().unwrap();
///
///     info!("This is an example message.");
/// }
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}
