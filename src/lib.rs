//! Rust Implementation of the J-Link GDB Server RTOS Plug-In for the RTXv5 RTOS.

/// J-Link GDB Server and RTXv5 c bindings.
mod bindings;

/// Module used for safely interacting with the API provided by the J-Link GDB Server.
mod api;

/// Crate used for working with static and constant C strings, necessary for requesting symbols from the GDB Server.
#[macro_use]
extern crate const_cstr;

use bindings::jlink::{GDB_API as GdbApi, RTOS_SYMBOLS as RtosSymbols};
use std::os::raw::{c_char, c_int, c_uint};

/* ------------------------------------- Constants ------------------------------------------------------------------ */

const_cstr! {
    // Symbol for the main OS control block.
    RTX_INFO_CSTR = "osRtxInfo";
}

/// Static array of structs, each containing the requested symbol from the GDB server.
/// When `RTOS_GetSymbols` is called, it fills in the empty pointers and sends the request to the GDB Server.
static mut RTOS_SYMBOLS_ARR: [RtosSymbols; 2] = [
    // osRtxInfo
    RtosSymbols {
        name: 0 as *const c_char, // Fill with `osRtxInfo`.
        optional: 0,              // This symbol is mandatory.
        address: 0,               // This will hold the symbol address.
    },
    // End Marker
    RtosSymbols {
        name: 0 as *const c_char, // Fill with `osRtxInfo`.
        optional: 0,              // This symbol is mandatory.
        address: 0,               // This will hold the symbol address.
    },
];

/* ------------------------------------- Constants ------------------------------------------------------------------ */

/// Returns the RTOS plugin version.
///
/// # Return value
/// The plugin version number as unsigned integer: 100 * [major] + 10 * [minor] + [patch].

/// # Notes:
/// Will be called before any other function. The J-Link GDB server only checks the RTOS plugin’s major version number.
/// The minor version number is freely choosable, it is printed in the GDB server’s log file but it is not evaluated.
#[no_mangle]
pub extern "C" fn RTOS_GetVersion() -> c_uint {
    const MAJOR: c_uint = pkg_version::pkg_version_major!();
    const MINOR: c_uint = pkg_version::pkg_version_minor!();
    const PATCH: c_uint = pkg_version::pkg_version_patch!();

    (MAJOR * 100) + (MINOR * 10) + (PATCH)
}

/// Returns a person with the name given them
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person
///
/// # Example
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to Rustdoc, it will even test it for you!
/// use doc::Person;
/// let person = Person::new("name");
/// ```
#[no_mangle]
pub extern "C" fn RTOS_Init(p_api: *const GdbApi, _core: c_uint) -> c_int {
    match api::init(p_api) {
        Err(_) => return 0,
        _ => (),
    }

    return 1;
}

#[no_mangle]
pub extern "C" fn RTOS_GetSymbols() -> *mut RtosSymbols {
    unsafe {
        RTOS_SYMBOLS_ARR[0].name = RTX_INFO_CSTR.as_ptr();
        RTOS_SYMBOLS_ARR.as_mut_ptr()
    }
}

#[no_mangle]
pub extern "C" fn RTOS_GetNumThreads() -> c_uint {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetCurrentThreadId() -> c_uint {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadId(_n: c_uint) -> c_uint {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadDisplay(_p_display: *mut c_char, _thread_id: c_uint) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadReg(
    _p_hex_reg_val: *mut c_char,
    _reg_index: c_uint,
    _thread_id: c_uint,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadRegList(_p_hex_reg_list: *mut c_char, _thread_id: c_uint) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_SetThreadReg(
    _p_hex_reg_val: *mut c_char,
    _reg_index: c_uint,
    _thread_id: c_uint,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_SetThreadRegList(_p_hex_reg_list: *mut c_char, _thread_id: c_uint) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_UpdateThreads() -> c_int {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
