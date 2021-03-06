//! Wrapper of the API provided by the J-Link GDB Server.

#![allow(dead_code)]

use std::option::Option::None;
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr::null_mut;

use std::ffi::{CStr, CString};

extern crate byteorder;
extern crate hex;
extern crate memchr;

use byteorder::{ByteOrder, NativeEndian};

pub use crate::bindings::jlink::GdbApi;

pub const GDB_OK: i32 = 0;
pub const GDB_ERR: i32 = -1;

/// Segger expect only the registers found in `org.gnu.gdb.arm.m-profile`.
pub const GDB_REG_LIST_LEN: usize = 17;

macro_rules! ensure {
    ($fun:expr) => {
        match ($fun) {
            Ok(v) => v,
            Err(_) => return crate::host::api::GDB_ERR,
        }
    };
}

/// Pointer to struct holding the API provided by the GDB Server, initialized in `RTOS_Init`.jlink
/// Must not be changed after that.
static mut GDB_API: Option<GdbApi> = None;

fn gdb_api_get() -> Option<&'static GdbApi> {
    unsafe { GDB_API.as_ref() }
}

unsafe fn gdb_api_set(gdb_api: GdbApi) {
    GDB_API = Some(gdb_api);
}

pub fn init(p_api: *const GdbApi) -> Result<(), ()> {
    if p_api.is_null() {
        return Err(());
    }

    unsafe {
        gdb_api_set(*p_api);
    }

    Ok(())
}

/* ------------------------------------- GDB Server API Wrappers ---------------------------------------------------- */

pub unsafe fn free(ptr: *mut u8) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.free {
            f(ptr as *mut c_void);
        }
    }
}

pub unsafe fn alloc(size: usize) -> *mut u8 {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.malloc {
            return f(size as c_uint) as *mut u8;
        }
    }

    null_mut()
}

pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.realloc {
            return f(ptr as *mut c_void, size as c_uint) as *mut u8;
        }
    }

    null_mut()
}

pub fn print(s: &str) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.output {
            if let Ok(c_string) = CString::new(s) {
                unsafe {
                    f(c_string.as_ptr());
                }
            }
        }
    }
}

pub fn print_debug(s: &str) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.output_debug {
            if let Ok(c_string) = CString::new(s) {
                unsafe {
                    f(c_string.as_ptr());
                }
            }
        }
    }
}

pub fn print_warning(s: &str) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.output_warning {
            if let Ok(c_string) = CString::new(s) {
                unsafe {
                    f(c_string.as_ptr());
                }
            }
        }
    }
}

pub fn print_error(s: &str) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.output_error {
            if let Ok(c_string) = CString::new(s) {
                unsafe {
                    f(c_string.as_ptr());
                }
            }
        }
    }
}

pub fn read_mem<T>(addr: u32) -> Result<T, i32> {
    trace!("read_mem. addr: {}", addr);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.read_byte_array {
            let mut value = std::mem::MaybeUninit::uninit();
            let size = std::mem::size_of::<T>();

            unsafe {
                if f(addr, value.as_mut_ptr() as *mut c_char, size as c_uint) as usize == size {
                    return Ok(value.assume_init());
                }
            }
        }
    }

    Err(GDB_ERR)
}

pub fn read_mem_by_len(addr: u32, len: usize) -> Result<Vec<u8>, i32> {
    trace!("read_mem_by_len. addr: {}, len: {}", addr, len);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.read_byte_array {
            let mut buff: Vec<u8> = vec![0u8; len];

            if unsafe { f(addr, buff.as_mut_ptr() as *mut c_char, len as c_uint) } as usize == len {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn read_u8(addr: u32) -> Result<u8, i32> {
    trace!("read_u8. addr: {}", addr);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.read_u8 {
            let mut buff: u8 = 0;

            if unsafe { f(addr, &mut buff) } as i32 == GDB_OK {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn read_u16(addr: u32) -> Result<u16, i32> {
    trace!("read_u16. addr: {}", addr);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.read_u16 {
            let mut buff: u16 = 0;

            if unsafe { f(addr, &mut buff) } as i32 == GDB_OK {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn read_u32(addr: u32) -> Result<u32, i32> {
    trace!("read_u32. addr: {}", addr);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.read_u32 {
            let mut buff: u32 = 0;

            if unsafe { f(addr, &mut buff) } as i32 == GDB_OK {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn write_mem<T: std::fmt::Debug>(addr: u32, data: &T) -> Result<(), i32> {
    trace!("write_mem. addr: {}, data: {:?}", addr, data);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.write_byte_array {
            if unsafe {
                f(
                    addr,
                    data as *const T as *const c_char,
                    std::mem::size_of::<T>() as c_uint,
                )
            } == GDB_OK
            {
                return Ok(());
            }
        }
    }

    Err(GDB_ERR)
}

pub fn write_u8(addr: u32, data: u8) {
    trace!("write_u8. addr: {}, data: {}", addr, data);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.write_u8 {
            unsafe {
                f(addr, data);
            }
        }
    }
}

pub fn write_u16(addr: u32, data: u16) {
    trace!("write_u16. addr: {}, data: {}", addr, data);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.write_u16 {
            unsafe {
                f(addr, data);
            }
        }
    }
}

pub fn write_u32(addr: u32, data: u32) {
    trace!("write_u32. addr: {}, data: {}", addr, data);
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.write_u32 {
            unsafe {
                f(addr, data);
            }
        }
    }
}

pub fn convert_u16(data: u16) -> Result<u16, i32> {
    trace!("convert_u16. data: {}", data);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.convert_u16 {
            return Ok(unsafe { f(&data as *const u16 as *const u8) } as u16);
        }
    }

    Err(GDB_ERR)
}

pub fn convert_u32(data: u32) -> Result<u32, i32> {
    trace!("convert_u32. data: {}", data);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.convert_u32 {
            return Ok(unsafe { f(&data as *const u32 as *const u8) });
        }
    }

    Err(GDB_ERR)
}

pub fn read_reg(reg_index: u32) -> Result<u32, i32> {
    trace!("read_reg. reg_index: {}", reg_index);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.read_register {
            return Ok(unsafe { f(reg_index) });
        }
    }

    Err(GDB_ERR)
}

pub fn write_reg(reg_index: u32, data: u32) {
    trace!("write_reg. reg_index: {}, data: {}", reg_index, data);

    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.write_register {
            unsafe {
                f(reg_index, data);
            }
        }
    }
}

/* ------------------------------------- GDB Server API Extensions/Helpers ------------------------------------------ */

pub fn read_string(addr: u32, max_len: usize) -> Result<String, i32> {
    trace!("read_string. addr: {}, max_len: {}", addr, max_len);

    if let Ok(buff) = read_mem_by_len(addr, max_len) {
        // Find null terminator - '\0'
        if let Some(pos) = memchr::memchr(0, &buff) {
            if let Ok(string) = std::str::from_utf8(&buff[..pos]) {
                return Ok(string.to_owned());
            }
        }
    }

    Err(GDB_ERR)
}

pub fn write_string_to_buff(p_buff: *mut c_char, string: &str) -> Result<usize, i32> {
    trace!("write_string_to_buff. string: {}", string);

    let c_string_size = string.len() + 1;

    if let Ok(c_string) = CString::new(string) {
        let write_size = c_string_size;

        unsafe {
            std::ptr::copy_nonoverlapping(c_string.as_ptr(), p_buff, write_size);
        }

        return Ok(write_size);
    }

    Err(GDB_ERR)
}

pub fn hex_str_to_vec_u32(hex_str: *const c_char) -> Result<Vec<u32>, i32> {
    let c_str: &CStr = unsafe { CStr::from_ptr(hex_str) };
    trace!("hex_arr_to_vec_u32. hex_str: {:?}", &c_str.to_bytes());

    if let Ok(bytes) = hex::decode(&c_str.to_bytes()) {
        let len = bytes.len();

        if len % 4 != 0 {
            return Err(GDB_ERR);
        }

        let mut out = vec![0; len / 4];

        NativeEndian::read_u32_into(&bytes, &mut out);

        return Ok(out);
    }

    Err(GDB_ERR)
}
