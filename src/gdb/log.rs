//! A logger that prints all messages to the GDB Server.

/// Log crate
use log::{Level, Metadata, Record, SetLoggerError};

/// GDB Server api.
use crate::gdb::api;

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

        let msg = iformat!(record.level() " - " record.args());

        match record.level() {
            Level::Trace | Level::Info => api::print(&msg),
            Level::Debug => api::print_debug(&msg),
            Level::Warn => api::print_warning(&msg),
            Level::Error => api::print_error(&msg),
        };
    }

    fn flush(&self) {}
}

/// Initializes the global logger with a GdbLogger instance with
/// `max_log_level` set to a specific log level.
///
/// ```
/// # #[macro_use] extern crate log;
/// # mod gdb_logger;
/// #
/// # fn main() {
/// gdb_logger::init_with_level(log::Level::Warn).unwrap();
///
/// warn!("This is an example message.");
/// info!("This message will not be logged.");
/// # }
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
/// ```
/// # #[macro_use] extern crate log;
/// # mod gdb_logger;
/// #
/// # fn main() {
/// gdb_logger::init().unwrap();
/// warn!("This is an example message.");
/// # }
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}
