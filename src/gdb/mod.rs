#[macro_use]
pub mod api;
pub mod log;

mod allocator;

extern crate log as ext_log;

use api::GdbApi;
use ext_log::Level;
/* ------------------------------------- Global Config--------------------------------------------------------------- */

#[global_allocator]
static ALLOCATOR: allocator::GdbAllocator = allocator::GdbAllocator;

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
