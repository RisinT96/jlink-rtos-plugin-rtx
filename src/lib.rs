//! Rust Implementation of the J-Link GDB Server RTOS Plug-In for the RTXv5 RTOS.

/* ------------------------------------- Crates and Modules  -------------------------------------------------------- */

/// For easier log/string creation.
#[macro_use]
extern crate ifmt;

/// For custom logger that logs through the GDB Server.
#[macro_use]
extern crate log;

/// J-Link GDB Server and RTXv5 c bindings.
mod bindings;
use bindings::jlink::RTOS_SYMBOLS as RtosSymbols;

/// Module used for safely interacting with the API provided by the J-Link GDB Server.
mod gdb;
use gdb::api;

use std::os::raw::{c_char, c_int, c_uint};

/* ------------------------------------- Constants ------------------------------------------------------------------ */

/// Symbols we want the GDB Server to find on the debugged device.
static mut RTOS_SYMBOLS_ARR: [RtosSymbols; 2] = [
    // osRtxInfo - Mandatory Symbol
    RtosSymbols {
        name: b"osRtxInfo\0".as_ptr() as *const c_char,
        optional: 0,
        address: 0,
    },
    // End Marker
    RtosSymbols {
        name: std::ptr::null(),
        optional: 0,
        address: 0,
    },
];

/* ------------------------------------- API Implementation --------------------------------------------------------- */

/// Returns the RTOS plugin version.
///
/// # Return value
/// The plugin version number as unsigned integer: `(100 * [major]) + (10 * [minor]) + [patch]`.

/// # Notes:
/// * __Will be called before any other function.__
/// * The J-Link GDB server only checks the RTOS plugin’s major version number.
/// The minor version number is freely choosable, it is printed in the GDB server’s log file but it is not evaluated.
#[no_mangle]
pub extern "C" fn RTOS_GetVersion() -> c_uint {
    let major: c_uint = pkg_version::pkg_version_major!();
    let minor: c_uint = pkg_version::pkg_version_minor!();
    let patch: c_uint = pkg_version::pkg_version_patch!();

    (major * 100) + (minor * 10) + (patch)
}

/// Initializes RTOS plug-in for further usage.
///
/// # Parameters
///
/// * `pAPI` - Pointer to API functions table.
/// * `core` - JLINK_CORE_# constant identifying the target’s core.
///
/// # Return value:
/// * `== 0` - Initialization failed.
/// * `== 1` - Initialized successfully.
///
/// # Notes:
/// If the plug-in does not support the CPU architecture, it should return 0.
/// The pointer to the API functions table should be stored locally.
/// API funtions can be used later by the plug-in to perform special functions like reading or writing to target
/// memory.
#[no_mangle]
pub extern "C" fn RTOS_Init(p_api: *const api::GdbApi, core: c_uint) -> c_int {
    match api::init(p_api) {
        Err(_) => return 0,
        _ => (),
    };

    match gdb::log::init_with_level(log::Level::Info) {
        Err(_) => return 0,
        _ => (),
    };

    info!("Initializing RTX Plugin!");

    match core {
        bindings::jlink::JLINK_CORE_CORTEX_M0
        | bindings::jlink::JLINK_CORE_CORTEX_M1
        | bindings::jlink::JLINK_CORE_CORTEX_M3
        | bindings::jlink::JLINK_CORE_CORTEX_M4 => (),
        _ => return 0,
    };

    return 1;
}

/// Returns a pointer to the RTOS symbol table.
///
/// # Return value:
/// Pointer to the RTOS symbol table.
///
/// # Notes:
/// The J-Link GDB server tries to find addresses of all of the symbols specified in the RTOS symbol table.
/// If a symbol is found, its address is written into RTOS_SYMBOLS.address, otherwise NULL is written into the address
/// field.
/// If any of the symbols, declared as mandatory, is not found, the plug-in is not being used by the GDB server.
/// It is ensured for the following functions, that every mandatory symbol has a valid address entry.
#[no_mangle]
pub extern "C" fn RTOS_GetSymbols() -> *mut RtosSymbols {
    unsafe { RTOS_SYMBOLS_ARR.as_mut_ptr() }
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
