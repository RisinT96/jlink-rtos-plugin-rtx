//! Rust Implementation of the J-Link GDB Server RTOS Plug-In for the RTXv5 RTOS.

/* ------------------------------------- Crates and Modules  -------------------------------------------------------- */

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};

/// Easier log/string creation.
#[macro_use]
extern crate ifmt;
extern crate chrono;

/// Custom logger that logs through the GDB Server.
#[macro_use]
extern crate log;

/// J-Link GDB Server and RTXv5 c bindings.
mod bindings;
use bindings::{RtosSymbols, RtxInfo, Thread};

/// Module used for safely interacting with the API provided by the J-Link GDB Server.
#[macro_use]
mod host;
use host::api;

/* ------------------------------------- Static Variables ----------------------------------------------------------- */

#[global_allocator]
static ALLOCATOR: host::allocator::GdbAllocator = host::allocator::GdbAllocator;

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

static mut RTX_INFO: Option<RtxInfo> = None;

/* ------------------------------------- Private Helper Functions --------------------------------------------------- */
fn rtx_info_get() -> Option<&'static RtxInfo> {
    unsafe { (RTX_INFO).as_ref() }
}

fn rtx_info_set(rtx_info: RtxInfo) {
    unsafe { RTX_INFO = Some(rtx_info) };
}

fn find_thread_by_id(id: u32) -> Result<&'static Thread, i32> {
    if let Some(rtx_info) = rtx_info_get() {
        for thread in &rtx_info.threads {
            if thread.id == id {
                return Ok(&thread);
            }
        }
    }
    Err(api::GDB_ERR)
}

/* ------------------------------------- API Implementation --------------------------------------------------------- */

/// Returns the RTOS plugin version.
///
/// # Return value
/// The plugin version number as unsigned integer: `(100 * [major]) + [patch]`.
///
/// # Notes:
/// * __Will be called before any other function.__
/// * The J-Link GDB server only checks the RTOS plugin’s major version number.
/// The minor version number is freely choosable, it is printed in the GDB server’s log file but it is not evaluated.
#[no_mangle]
pub extern "C" fn RTOS_GetVersion() -> c_uint {
    101
}

/// Initializes RTOS plug-in for further usage.
///
/// # Parameters
/// * `pAPI` - Pointer to API functions table.
/// * `core` - JLINK_CORE_# constant identifying the target’s core.
///
/// # Return value:
/// * `== 0` - Initialization failed.
/// * `== 1` - Initialized successfully.
///
/// # Notes:
/// * If the plug-in does not support the CPU architecture, it should return 0.
/// * The pointer to the API functions table should be stored locally.
/// * API funtions can be used later by the plug-in to perform special functions like reading or writing to target
///   memory.
#[no_mangle]
pub extern "C" fn RTOS_Init(p_api: *const api::GdbApi, core: c_uint) -> c_int {
    // Initialize the GDB Server interface module
    match host::init(p_api, log::Level::Trace) {
        Err(_) => return 0,
        _ => (),
    };

    // Now the underlying systems should be initialized, we can begin work.
    info!("Initializing RTX Plugin");

    match core {
        bindings::jlink::CORTEX_M0
        | bindings::jlink::CORTEX_M1
        | bindings::jlink::CORTEX_M3
        | bindings::jlink::CORTEX_M4 => (),
        _ => return 0,
    };

    info!("Successfully initialized RTX Plugin");

    1
}

/// Returns a pointer to the RTOS symbol table.
///
/// # Return value:
/// Pointer to the RTOS symbol table.
///
/// # Notes:
/// * The J-Link GDB server tries to find addresses of all of the symbols specified in the RTOS symbol table.
/// * If a symbol is found, its address is written into RTOS_SYMBOLS.address, otherwise NULL is written into the address
///   field.
/// * If any of the symbols, declared as mandatory, is not found, the plug-in is not being used by the GDB server.
/// * __It is ensured for the following functions, that every mandatory symbol has a valid address entry__.
#[no_mangle]
pub extern "C" fn RTOS_GetSymbols() -> *mut RtosSymbols {
    trace!("RTOS_GetSymbols");

    unsafe { RTOS_SYMBOLS_ARR.as_mut_ptr() }
}

/// Updates the thread information from the target.
///
/// # Return value:
/// * `== 0` - Updating threads OK.
/// * `<  0` - Updating threads failed.
///
/// # Notes:
/// * For efficiency purposes, the plug-in should read all required information within this function at once, so later
///   requests can be served without further communication to the target.
/// * IMO - We should only read the minimal data we need to know the state of all the threads, any further data should
///   be read only when requested.
#[no_mangle]
pub extern "C" fn RTOS_UpdateThreads() -> c_int {
    trace!("RTOS_UpdateThreads");

    let rtx_info = ensure!(bindings::RtxInfo::new(unsafe {
        RTOS_SYMBOLS_ARR[0].address
    }));

    rtx_info_set(rtx_info);

    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetNumThreads() -> c_uint {
    trace!("RTOS_GetNumThreads");

    if let Some(rtx_info) = rtx_info_get() {
        rtx_info.threads.len() as u32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn RTOS_GetCurrentThreadId() -> c_uint {
    trace!("RTOS_GetCurrentThreadId");

    if let Some(rtx_info) = rtx_info_get() {
        rtx_info.threads[0].id
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadId(n: c_uint) -> c_uint {
    trace!("RTOS_GetThreadId");

    if let Some(rtx_info) = rtx_info_get() {
        rtx_info.threads[n as usize].id
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadDisplay(p_display: *mut c_char, thread_id: c_uint) -> c_int {
    trace!("RTOS_GetThreadDisplay");

    let thread = ensure!(find_thread_by_id(thread_id));
    let thread_name = thread.to_string();
    let thread_name_len = thread_name.len();

    let write_len = std::cmp::min(
        thread_name_len + 1,
        bindings::jlink::RTOS_PLUGIN_BUF_SIZE_THREAD_DISPLAY as usize,
    );

    let thread_name_cstr = ensure!(CString::new(thread_name));

    unsafe {
        std::ptr::copy_nonoverlapping(thread_name_cstr.as_ptr(), p_display, write_len as usize);
    }

    write_len as i32
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadReg(
    _p_hex_reg_val: *mut c_char,
    _reg_index: c_uint,
    _thread_id: c_uint,
) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn RTOS_GetThreadRegList(_p_hex_reg_list: *mut c_char, _thread_id: c_uint) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn RTOS_SetThreadReg(
    _p_hex_reg_val: *mut c_char,
    _reg_index: c_uint,
    _thread_id: c_uint,
) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn RTOS_SetThreadRegList(_p_hex_reg_list: *mut c_char, _thread_id: c_uint) -> c_int {
    -1
}
