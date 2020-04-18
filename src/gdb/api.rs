use std::os::raw::{c_char, c_uint, c_void};
use std::ptr::null_mut;

use std::ffi::CString;

pub use crate::bindings::jlink::GDB_API as GdbApi;

pub const OK: i32 = 0;
pub const ERR: i32 = -1;

macro_rules! ensure_ok {
    ($op:expr) => {
        if($op != crate::gdb::api::OK) {
            return crate::gdb::api::ERR;
        }
    };
}

/// Pointer to struct holding the API provided by the GDB Server, initialized in `RTOS_Init`.jlink
/// Must not be changed after that.
static mut GDB_API: GdbApi = GdbApi {
    pfFree: ::std::option::Option::None,
    pfAlloc: ::std::option::Option::None,
    pfRealloc: ::std::option::Option::None,
    pfLogOutf: ::std::option::Option::None,
    pfDebugOutf: ::std::option::Option::None,
    pfWarnOutf: ::std::option::Option::None,
    pfErrorOutf: ::std::option::Option::None,
    pfReadMem: ::std::option::Option::None,
    pfReadU8: ::std::option::Option::None,
    pfReadU16: ::std::option::Option::None,
    pfReadU32: ::std::option::Option::None,
    pfWriteMem: ::std::option::Option::None,
    pfWriteU8: ::std::option::Option::None,
    pfWriteU16: ::std::option::Option::None,
    pfWriteU32: ::std::option::Option::None,
    pfLoad16TE: ::std::option::Option::None,
    pfLoad24TE: ::std::option::Option::None,
    pfLoad32TE: ::std::option::Option::None,
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

pub fn read_mem(addr: u32, size: usize) -> Result<Vec<u8>, i32> {
    // Heap allocate empty vector for data
    let mut buff = vec![0u8; size];

    unsafe {
        match GDB_API.pfReadMem {
            Some(f) => match f(addr, buff.as_mut_ptr() as *mut c_char, size as c_uint) {
                OK => Ok(buff),
                e => Err(e),
            },
            None => Err(ERR),
        }
    }
}

pub fn read_u8(addr: u32) -> Result<u8, i32> {
    let mut buff: u8 = 0;

    unsafe {
        match GDB_API.pfReadU8 {
            Some(f) => match f(addr, &mut buff) {
                OK => Ok(buff),
                e => Err(e),
            },
            None => Err(ERR),
        }
    }
}

pub fn read_u16(addr: u32) -> Result<u16, i32> {
    let mut buff: u16 = 0;

    unsafe {
        match GDB_API.pfReadU16 {
            Some(f) => match f(addr, &mut buff) {
                OK => Ok(buff),
                e => Err(e),
            },
            None => Err(ERR),
        }
    }
}

pub fn read_u32(addr: u32) -> Result<u32, i32> {
    let mut buff: u32 = 0;

    unsafe {
        match GDB_API.pfReadU32 {
            Some(f) => match f(addr, &mut buff) {
                OK => Ok(buff),
                e => Err(e),
            },
            None => Err(ERR),
        }
    }
}

pub unsafe fn write_mem(addr: u32, data: &[u8]) -> Result<(), i32> {
    {
        match GDB_API.pfWriteMem {
            None => Err(ERR),
            Some(f) => match f(addr, data.as_ptr() as *const c_char, data.len() as c_uint) {
                OK => Ok(()),
                e => Err(e),
            },
        }
    }
}

pub unsafe fn write_u8(addr: u32, data: u8) {
    {
        match GDB_API.pfWriteU8 {
            Some(f) => f(addr, data),
            None => (),
        }
    }
}

pub unsafe fn write_u16(addr: u32, data: u16) {
    {
        match GDB_API.pfWriteU16 {
            Some(f) => f(addr, data),
            None => (),
        }
    }
}

pub unsafe fn write_u32(addr: u32, data: u32) {
    {
        match GDB_API.pfWriteU32 {
            Some(f) => f(addr, data),
            None => (),
        }
    }
}
