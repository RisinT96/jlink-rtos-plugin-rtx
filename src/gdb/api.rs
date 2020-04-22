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
            Err(_) => return crate::gdb::api::GDB_ERR,
        }
    };
}

/// Only data that is C FFI safe should have this trait.
pub unsafe trait ReprC {}

/// Pointer to struct holding the API provided by the GDB Server, initialized in `RTOS_Init`.jlink
/// Must not be changed after that.
static mut GDB_API: GdbApi = GdbApi {
    pfFree: None,
    pfAlloc: None,
    pfRealloc: None,
    pfLogOutf: None,
    pfDebugOutf: None,
    pfWarnOutf: None,
    pfErrorOutf: None,
    pfReadMem: None,
    pfReadU8: None,
    pfReadU16: None,
    pfReadU32: None,
    pfWriteMem: None,
    pfWriteU8: None,
    pfWriteU16: None,
    pfWriteU32: None,
    pfLoad16TE: None,
    pfLoad24TE: None,
    pfLoad32TE: None,
    pfReadReg: None,
    pfWriteReg: None,
    Dummy: null_mut(),
};

pub fn init(p_api: *const GdbApi) -> Result<(), ()> {
    if p_api.is_null() {
        return Err(());
    }

    unsafe {
        GDB_API = *p_api;
    }

    Ok(())
}

pub unsafe fn free(ptr: *mut u8) {
    match GDB_API.pfFree {
        Some(f) => f(ptr as *mut c_void),
        None => (),
    }
}

pub unsafe fn alloc(size: usize) -> *mut u8 {
    match GDB_API.pfAlloc {
        Some(f) => f(size as c_uint) as *mut u8,
        None => null_mut(),
    }
}

pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    match GDB_API.pfRealloc {
        Some(f) => f(ptr as *mut c_void, size as c_uint) as *mut u8,
        None => null_mut(),
    }
}

pub fn print(s: &str) {
    let c_to_print = CString::new(s).unwrap();

    unsafe {
        match GDB_API.pfLogOutf {
            Some(f) => f(c_to_print.as_ptr()),
            None => (),
        }
    }
}

pub fn print_debug(s: &str) {
    let c_to_print = CString::new(s).unwrap();

    unsafe {
        match GDB_API.pfDebugOutf {
            Some(f) => f(c_to_print.as_ptr()),
            None => (),
        }
    }
}

pub fn print_warning(s: &str) {
    let c_to_print = CString::new(s).unwrap();

    unsafe {
        match GDB_API.pfWarnOutf {
            Some(f) => f(c_to_print.as_ptr()),
            None => (),
        }
    }
}

pub fn print_error(s: &str) {
    let c_to_print = CString::new(s).unwrap();

    unsafe {
        match GDB_API.pfErrorOutf {
            Some(f) => f(c_to_print.as_ptr()),
            None => (),
        }
    }
}

pub fn read_mem<T>(addr: u32) -> Result<T, i32> {
    let mut value = std::mem::MaybeUninit::uninit();

    unsafe {
        match GDB_API.pfReadMem {
            Some(f) => match f(
                addr,
                value.as_mut_ptr() as *mut c_char,
                std::mem::size_of::<T>() as c_uint,
            ) {
                GDB_OK => Ok(value.assume_init()),
                e => Err(e),
            },
            None => Err(GDB_ERR),
        }
    }
}

pub fn read_u8(addr: u32) -> Result<u8, i32> {
    let mut buff: u8 = 0;

    unsafe {
        match GDB_API.pfReadU8 {
            Some(f) => match f(addr, &mut buff) as i32 {
                GDB_OK => Ok(buff),
                e => Err(e),
            },
            None => Err(GDB_ERR),
        }
    }
}

pub fn read_u16(addr: u32) -> Result<u16, i32> {
    let mut buff: u16 = 0;

    unsafe {
        match GDB_API.pfReadU16 {
            Some(f) => match f(addr, &mut buff) as i32{
                GDB_OK => Ok(buff),
                e => Err(e),
            },
            None => Err(GDB_ERR),
        }
    }
}

pub fn read_u32(addr: u32) -> Result<u32, i32> {
    let mut buff: u32 = 0;

    unsafe {
        match GDB_API.pfReadU32 {
            Some(f) => match f(addr, &mut buff) as i32{
                GDB_OK => Ok(buff),
                e => Err(e),
            },
            None => Err(GDB_ERR),
        }
    }
}

pub fn write_mem<T>(addr: u32, data: &T) -> Result<(), i32> {
    unsafe {
        match GDB_API.pfWriteMem {
            None => Err(GDB_ERR),
            Some(f) => match f(
                addr,
                data as *const T as *const c_char,
                std::mem::size_of::<T>() as c_uint,
            ) {
                GDB_OK => Ok(()),
                e => Err(e),
            },
        }
    }
}

pub fn write_u8(addr: u32, data: u8) {
    unsafe {
        match GDB_API.pfWriteU8 {
            Some(f) => f(addr, data),
            None => (),
        }
    }
}

pub fn write_u16(addr: u32, data: u16) {
    unsafe {
        match GDB_API.pfWriteU16 {
            Some(f) => f(addr, data),
            None => (),
        }
    }
}

pub fn write_u32(addr: u32, data: u32) {
    unsafe {
        match GDB_API.pfWriteU32 {
            Some(f) => f(addr, data),
            None => (),
        }
    }
}

pub fn read_string(addr: u32, max_len: usize) -> Result<String, i32> {
    let mut temp_buff: Vec<u8> = vec![0u8; max_len];

    unsafe {
        if let Some(f) = GDB_API.pfReadMem {
            if GDB_OK
                != f(
                    addr,
                    temp_buff.as_mut_ptr() as *mut c_char,
                    max_len as c_uint,
                )
            {
                return Err(GDB_ERR);
            }
        }
    }

    // Find null terminator - '\0'
    if let Some(pos) = memchr::memchr(0, &temp_buff) {
        if let Ok(v) = std::str::from_utf8(&temp_buff[..=pos]) {
            return Ok(v.to_owned());
        }
    }

    Err(GDB_ERR)
}
