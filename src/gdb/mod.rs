#[macro_use]
pub mod api;
pub mod log;
pub mod allocator;


extern crate log as ext_log;

use api::GdbApi;
use ext_log::Level;
/* ------------------------------------- Global Config--------------------------------------------------------------- */

pub fn init(p_api: *const GdbApi, log_level: Level) -> Result<(), ()> {
    // Initialize the GDB Server API, also enabling the allocator.
    api::init(p_api)?;

    // Once the API is initialized, we can initialize the logger.
    match log::init_with_level(log_level) {
        Err(_) => return Err(()),
        _ => (),
    };

    Ok(())
}
