//! Wrapper of the API provided by the J-Link GDB Server.

use std::option::Option::None;
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr::null_mut;

use std::ffi::CString;

extern crate memchr;

pub use crate::bindings::jlink::GDB_API as GdbApi;

pub const GDB_OK: i32 = 0;
pub const GDB_ERR: i32 = -1;

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
        if let Some(f) = gdb_api.pfFree {
            f(ptr as *mut c_void);
        }
    }
}

pub unsafe fn alloc(size: usize) -> *mut u8 {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfAlloc {
            return f(size as c_uint) as *mut u8;
        }
    }

    null_mut()
}

pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfRealloc {
            return f(ptr as *mut c_void, size as c_uint) as *mut u8;
        }
    }

    null_mut()
}

pub fn print(s: &str) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfLogOutf {
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
        if let Some(f) = gdb_api.pfDebugOutf {
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
        if let Some(f) = gdb_api.pfWarnOutf {
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
        if let Some(f) = gdb_api.pfErrorOutf {
            if let Ok(c_string) = CString::new(s) {
                unsafe {
                    f(c_string.as_ptr());
                }
            }
        }
    }
}

pub fn read_mem<T>(addr: u32) -> Result<T, i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfReadMem {
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
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfReadMem {
            let mut buff: Vec<u8> = vec![0u8; len];

            if unsafe { f(addr, buff.as_mut_ptr() as *mut c_char, len as c_uint) } as usize == len {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn read_u8(addr: u32) -> Result<u8, i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfReadU8 {
            let mut buff: u8 = 0;

            if unsafe { f(addr, &mut buff) } as i32 == GDB_OK {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn read_u16(addr: u32) -> Result<u16, i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfReadU16 {
            let mut buff: u16 = 0;

            if unsafe { f(addr, &mut buff) } as i32 == GDB_OK {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn read_u32(addr: u32) -> Result<u32, i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfReadU32 {
            let mut buff: u32 = 0;

            if unsafe { f(addr, &mut buff) } as i32 == GDB_OK {
                return Ok(buff);
            }
        }
    }

    Err(GDB_ERR)
}

pub fn write_mem<T>(addr: u32, data: &T) -> Result<(), i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfWriteMem {
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
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfWriteU8 {
            unsafe {
                f(addr, data);
            }
        }
    }
}

pub fn write_u16(addr: u32, data: u16) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfWriteU16 {
            unsafe {
                f(addr, data);
            }
        }
    }
}

pub fn write_u32(addr: u32, data: u32) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfWriteU32 {
            unsafe {
                f(addr, data);
            }
        }
    }
}

pub fn convert_u16(data: u16) -> Result<u16, i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfLoad16TE {
            return Ok(unsafe { f(&data as *const u16 as *const u8) } as u16);
        }
    }

    Err(GDB_ERR)
}

pub fn convert_u32(data: u32) -> Result<u32, i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfLoad32TE {
            return Ok(unsafe { f(&data as *const u32 as *const u8) } as u32);
        }
    }

    Err(GDB_ERR)
}

pub fn read_reg(reg_index: u32) -> Result<u32, i32> {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfReadReg {
            return Ok(unsafe { f(reg_index) });
        }
    }

    Err(GDB_ERR)
}

pub fn write_reg(reg_index: u32, data: u32) {
    if let Some(gdb_api) = gdb_api_get() {
        if let Some(f) = gdb_api.pfWriteReg {
            unsafe {
                f(reg_index, data);
            }
        }
    }
}

/* ------------------------------------- GDB Server API Extensions/Helpers ------------------------------------------ */

pub fn read_string(addr: u32, max_len: usize) -> Result<String, i32> {
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
    let c_string_size = string.len() + 1;

    if let Ok(c_string) = CString::new(string) {
        let write_size = std::cmp::min(
            c_string_size,
            crate::bindings::jlink::RTOS_PLUGIN_BUF_SIZE_THREAD_DISPLAY as usize,
        );

        unsafe {
            std::ptr::copy_nonoverlapping(c_string.as_ptr(), p_buff, write_size);
        }

        return Ok(write_size);
    }

    Err(GDB_ERR)
}
