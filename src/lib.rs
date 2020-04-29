//! Rust Implementation of the J-Link GDB Server RTOS Plug-In for the RTXv5 RTOS.

/* ------------------------------------- Crates and Modules  -------------------------------------------------------- */

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
use bindings::RtosSymbols;

/// Module used for safely interacting with the API provided by the J-Link GDB Server.
#[macro_use]
mod host;
use host::{api, GdbAllocator};

/// Representation of RTX Objects.
mod device;
use device::{RtxInfo, Thread};

/* ------------------------------------- Static Variables ----------------------------------------------------------- */

#[global_allocator]
static ALLOCATOR: GdbAllocator = GdbAllocator;

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

fn get_current_running_thread() -> Result<&'static Thread, i32> {
    if let Some(rtx_info) = rtx_info_get() {
        if rtx_info.threads.len() > 0 {
            return Ok(&rtx_info.threads[0]);
        }
    }

    Err(api::GDB_ERR)
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
/// # Notes
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
/// # Return value
/// * `== 0` - Initialization failed.
/// * `== 1` - Initialized successfully.
///
/// # Notes
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
    info!("Initializing RTX Plugin. core: {}", core);

    if let Err(_) = device::core::init(core) {
        return 0;
    }

    info!("Successfully initialized RTX Plugin");

    1
}

/// Returns a pointer to the RTOS symbol table.
///
/// # Return value
/// Pointer to the RTOS symbol table.
///
/// # Notes
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
/// # Return value
/// * `== 0` - Updating threads OK.
/// * `<  0` - Updating threads failed.
///
/// # Notes
/// * For efficiency purposes, the plug-in should read all required information within this function at once, so later
///   requests can be served without further communication to the target.
/// * IMO - We should only read the minimal data we need to know the state of all the threads, any further data should
///   be read only when requested.
#[no_mangle]
pub extern "C" fn RTOS_UpdateThreads() -> c_int {
    trace!("RTOS_UpdateThreads");

    let rtx_info = ensure!(RtxInfo::new(unsafe { RTOS_SYMBOLS_ARR[0].address }));

    rtx_info_set(rtx_info);

    api::GDB_OK
}

/// Returns the number of threads.
///
/// # Return value
/// Number of threads.
///
/// # Notes
/// After calling this function, the GDB server will request the thread ID by calling `RTOS_GetThreadId()` for every
/// thread.
#[no_mangle]
pub extern "C" fn RTOS_GetNumThreads() -> c_uint {
    trace!("RTOS_GetNumThreads");

    if let Some(rtx_info) = rtx_info_get() {
        rtx_info.threads.len() as u32
    } else {
        0
    }
}

/// Returns the ID of the currently running thread.
///
/// # Return value
/// ID of the currently running thread.
#[no_mangle]
pub extern "C" fn RTOS_GetCurrentThreadId() -> c_uint {
    trace!("RTOS_GetCurrentThreadId");

    if let Some(rtx_info) = rtx_info_get() {
        rtx_info.threads[0].id
    } else {
        0
    }
}

/// Returns the ID of the thread with index number n.
///
/// # Parameters
/// * `index` - Index number of the thread.
///
/// # Return value
/// ID of the thread.
///
/// # Notes
/// Index numbers for threads run from `0..=[n-1]`, where `n` is the number of threads returned by
/// `RTOS_GetNumThreads()`.
#[no_mangle]
pub extern "C" fn RTOS_GetThreadId(index: c_uint) -> c_uint {
    trace!("RTOS_GetThreadId. index: {}", index);

    if let Some(rtx_info) = rtx_info_get() {
        rtx_info.threads[index as usize].id
    } else {
        0
    }
}

/// Prints the thread’s name to `p_display`. The name may contain extra information about the thread’s status (running/
/// suspended, priority, etc.).
///
/// # Parameters
/// * `p_display` - Pointer to the string, the name has to be copied to
/// * `thread_id` - ID of the thread
///
/// # Return value
/// Length of the name string.alloc
///
/// # Notes
/// The space reserved for the name is 256 bytes (including terminating zero), as defined in
/// `RTOS_PLUGIN_BUF_SIZE_THREAD_DISPLAY`.
#[no_mangle]
pub extern "C" fn RTOS_GetThreadDisplay(p_display: *mut c_char, thread_id: c_uint) -> c_int {
    trace!("RTOS_GetThreadDisplay, thread_id: {:#010X}", thread_id);

    let thread = ensure!(find_thread_by_id(thread_id));
    let thread_name = thread.to_string();

    ensure!(api::write_string_to_buff(p_display, &thread_name)) as i32
}

/// Copies the thread’s register value into `p_hex_reg_val` as a HEX string.
/// If the register value has to be read directly from the CPU, the function must return a value <0, the register value
/// is then read from the CPU by the GDB server itself.
///
/// # Parameters
/// * `p_hex_reg_val` - Pointer to the string, the value has to be copied to.
/// * `reg_index`     - Index of the register.
/// * `thread_id`     - ID of the thread.
///
/// # Return value
/// * `== 0` - Reading register OK.
/// * `<  0` - Reading register failed.
#[no_mangle]
pub extern "C" fn RTOS_GetThreadReg(
    p_hex_reg_val: *mut c_char,
    reg_index: c_uint,
    thread_id: c_uint,
) -> c_int {
    trace!(
        "RTOS_GetThreadReg. reg_index: {}, thread_id {:#010X}",
        reg_index,
        thread_id
    );

    if ensure!(get_current_running_thread()).id == thread_id {
        return api::GDB_ERR;
    }

    api::GDB_ERR
}

/// Copies the thread’s general registers' values into `p_hex_reg_list` as a HEX string.
/// If the register values have to be read directly from the CPU, the function must return a value <0, the register
/// values are then read from the CPU by the GDB server itself.
///
/// # Parameters
/// * `p_hex_reg_list` - Pointer to the string, the values have to be copied to.
/// * `thread_id`      - ID of the thread.
///
/// # Return value
/// * `== 0` - Reading registers OK.
/// * `<  0` - Reading registers failed.
#[no_mangle]
pub extern "C" fn RTOS_GetThreadRegList(p_hex_reg_list: *mut c_char, thread_id: c_uint) -> c_int {
    trace!("RTOS_GetThreadRegList. thread_id {:#010X}", thread_id);

    if ensure!(get_current_running_thread()).id == thread_id {
        return api::GDB_ERR;
    }

    api::GDB_ERR
}

/// Sets the thread’s register's value to `p_hex_reg_val`, given as a HEX string.
/// If the register value has to be written directly to the CPU, the function must return a value <0, the register
/// value is then written to the CPU by the GDB server itself.
///
/// # Parameters
/// * `p_hex_reg_val` - Pointer to the string, containing the value to write.
/// * `reg_index`     - Index of the register.
/// * `thread_id`     - ID of the thread.
///
/// # Return value
/// * `== 0` - Writing register OK.
/// * `<  0` - Writing register failed.
#[no_mangle]
pub extern "C" fn RTOS_SetThreadReg(
    p_hex_reg_val: *mut c_char,
    reg_index: c_uint,
    thread_id: c_uint,
) -> c_int {
    trace!(
        "RTOS_SetThreadReg. reg_index: {}, thread_id {:#010X}",
        reg_index,
        thread_id
    );

    if ensure!(get_current_running_thread()).id == thread_id {
        return api::GDB_ERR;
    }

    api::GDB_ERR
}

/// Sets the thread’s general registers' values to `p_hex_reg_list`, given as a HEX string.
/// If the register values have to be written directly to the CPU, the function must return a value <0, the register
/// values are then written to the CPU by the GDB server itself.
///
/// # Parameters
/// * `p_hex_reg_list` - Pointer to the string, containing the values to write.
/// * `thread_id`      - ID of the thread.
///
/// # Return value
/// * `== 0` - Writing registers OK.
/// * `<  0` - Writing registers failed.
#[no_mangle]
pub extern "C" fn RTOS_SetThreadRegList(p_hex_reg_list: *mut c_char, thread_id: c_uint) -> c_int {
    trace!("RTOS_SetThreadRegList. thread_id {:#010X}", thread_id);

    if ensure!(get_current_running_thread()).id == thread_id {
        return api::GDB_ERR;
    }

    api::GDB_ERR
}
