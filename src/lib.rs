mod api;

use api::jlink::{GDB_API, RTOS_SYMBOLS};
use api::rtx::osRtxInfo_t;
use std::os::raw::{c_char, c_int, c_uint};

#[macro_use]
extern crate const_cstr;

// Static & constant c strings necessary to get symbols from the GDB server.
const_cstr! {
    RTX_INFO_CSTR = "osRtxInfo";
}

// Static array of structs, each containing the requested symbol from the GDB server.
// When `RTOS_GetSymbols` is called, it fills in the empty pointers and sends the request to the GDB server.
static mut RTOS_SYMBOLS_TABLE: [RTOS_SYMBOLS; 1] = [RTOS_SYMBOLS {
    name: 0 as *const c_char, // Fill with `osRtxInfo`.
    optional: 0,              // This symbol is mandatory.
    address: 0,               // This will hold the symbol address.
}];

static mut API: *const GDB_API = 0 as *const GDB_API;

#[no_mangle]
pub extern "C" fn RTOS_Init(p_api: *const GDB_API, _core: c_uint) -> c_int {
    unsafe {
        API = p_api;
    }

    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetVersion() -> c_uint {
    0
}

#[no_mangle]
pub extern "C" fn RTOS_GetSymbols() -> *mut RTOS_SYMBOLS {
    let rtx_info_cstr_ptr = RTX_INFO_CSTR.as_ptr();
    unsafe {
        RTOS_SYMBOLS_TABLE[0].name = rtx_info_cstr_ptr;
        RTOS_SYMBOLS_TABLE.as_mut_ptr()
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
