pub mod jlink;
pub mod rtx;

use jlink::{GDB_API as GdbApi};

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

pub fn init_gdb_server_api(p_api : *const GdbApi) -> Result<(),()>{
    if p_api.is_null() {
        return Err(());
    }

    unsafe {
        GDB_API = *p_api;
    }

    Ok(())
}