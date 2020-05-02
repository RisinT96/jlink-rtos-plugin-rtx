//! A simple interface to the J-Link GDB Server
//!
//! This module provides the following:
//! * Memory allocator that's based on the GDB Server's API
//! * Wrapper functions of the GDB Server's API
//! * Logger that prints to the GDB Server's output

#[macro_use]
pub mod api;

mod allocator;
mod log;

extern crate log as ext_log;

pub use allocator::GdbAllocator;

use api::GdbApi;
use ext_log::Level;
/* ------------------------------------- Global Config--------------------------------------------------------------- */

/// Initializes the GDB Server API, enabling the allocator, logger and API wrapper functions.
pub fn init(p_api: *const GdbApi, log_level: Level) -> Result<(), ()> {
    // Initialize the allocator and API wrapper.
    api::init(p_api)?;

    // Once the API is initialized (an memory allocation is enabled), we can initialize the logger.
    match log::init_with_level(log_level) {
        Err(_) => return Err(()),
        _ => (),
    };

    Ok(())
}
