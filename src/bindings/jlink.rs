//! Bindings auto generated from SEGGER RTOS Plug-in S

use std::option::Option;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void};

use num_derive::FromPrimitive;

pub const CORE_NONE: u32 = 0;
pub const CORE_ANY: u32 = 4294967295;
pub const CORE_CORTEX_M1: u32 = 16777471;
pub const CORE_COLDFIRE: u32 = 50331647;
pub const CORE_CORTEX_M3: u32 = 50331903;
pub const CORE_CORTEX_M3_R1P0: u32 = 50331664;
pub const CORE_CORTEX_M3_R1P1: u32 = 50331665;
pub const CORE_CORTEX_M3_R2P0: u32 = 50331680;
pub const CORE_SIM: u32 = 83886079;
pub const CORE_XSCALE: u32 = 100663295;
pub const CORE_CORTEX_M0: u32 = 100663551;
pub const CORE_CORTEX_M_V8BASEL: u32 = 100729087;
pub const CORE_ARM7: u32 = 134217727;
pub const CORE_ARM7TDMI: u32 = 117440767;
pub const CORE_ARM7TDMI_R3: u32 = 117440575;
pub const CORE_ARM7TDMI_R4: u32 = 117440591;
pub const CORE_ARM7TDMI_S: u32 = 117441023;
pub const CORE_ARM7TDMI_S_R3: u32 = 117440831;
pub const CORE_ARM7TDMI_S_R4: u32 = 117440847;
pub const CORE_CORTEX_A8: u32 = 134217983;
pub const CORE_CORTEX_A7: u32 = 134742271;
pub const CORE_CORTEX_A9: u32 = 134807807;
pub const CORE_CORTEX_A12: u32 = 134873343;
pub const CORE_CORTEX_A15: u32 = 134938879;
pub const CORE_CORTEX_A17: u32 = 135004415;
pub const CORE_ARM9: u32 = 167772159;
pub const CORE_ARM9TDMI_S: u32 = 150995455;
pub const CORE_ARM920T: u32 = 153092351;
pub const CORE_ARM922T: u32 = 153223423;
pub const CORE_ARM926EJ_S: u32 = 153485823;
pub const CORE_ARM946E_S: u32 = 155582975;
pub const CORE_ARM966E_S: u32 = 157680127;
pub const CORE_ARM968E_S: u32 = 157811199;
pub const CORE_ARM11: u32 = 201326591;
pub const CORE_ARM1136: u32 = 188153855;
pub const CORE_ARM1136J: u32 = 188089087;
pub const CORE_ARM1136J_S: u32 = 188089343;
pub const CORE_ARM1136JF: u32 = 188090111;
pub const CORE_ARM1136JF_S: u32 = 188090367;
pub const CORE_ARM1156: u32 = 190251007;
pub const CORE_ARM1176: u32 = 192348159;
pub const CORE_ARM1176J: u32 = 192283391;
pub const CORE_ARM1176J_S: u32 = 192283647;
pub const CORE_ARM1176JF: u32 = 192284415;
pub const CORE_ARM1176JF_S: u32 = 192284671;
pub const CORE_CORTEX_R4: u32 = 201326847;
pub const CORE_CORTEX_R5: u32 = 201392383;
pub const CORE_RX: u32 = 234881023;
pub const CORE_RX610: u32 = 218169343;
pub const CORE_RX62N: u32 = 218234879;
pub const CORE_RX62T: u32 = 218300415;
pub const CORE_RX63N: u32 = 218365951;
pub const CORE_RX630: u32 = 218431487;
pub const CORE_RX63T: u32 = 218497023;
pub const CORE_RX621: u32 = 218562559;
pub const CORE_RX62G: u32 = 218628095;
pub const CORE_RX631: u32 = 218693631;
pub const CORE_RX65N: u32 = 218759167;
pub const CORE_RX210: u32 = 219217919;
pub const CORE_RX21A: u32 = 219283455;
pub const CORE_RX220: u32 = 219348991;
pub const CORE_RX230: u32 = 219414527;
pub const CORE_RX231: u32 = 219480063;
pub const CORE_RX23T: u32 = 219545599;
pub const CORE_RX24T: u32 = 219611135;
pub const CORE_RX111: u32 = 220266495;
pub const CORE_RX110: u32 = 220332031;
pub const CORE_RX113: u32 = 220397567;
pub const CORE_RX130: u32 = 220463103;
pub const CORE_RX64M: u32 = 221315071;
pub const CORE_RX71M: u32 = 221380607;
pub const CORE_CORTEX_M4: u32 = 234881279;
pub const CORE_CORTEX_M7: u32 = 234946815;
pub const CORE_CORTEX_M_V8MAINL: u32 = 235012351;
pub const CORE_CORTEX_A5: u32 = 251658495;
pub const CORE_POWER_PC: u32 = 285212671;
pub const CORE_POWER_PC_N1: u32 = 285147391;
pub const CORE_POWER_PC_N2: u32 = 285147647;
pub const CORE_MIPS: u32 = 301989887;
pub const CORE_MIPS_M4K: u32 = 285278207;
pub const CORE_MIPS_MICROAPTIV: u32 = 285343743;
pub const CORE_EFM8_UNSPEC: u32 = 318767103;
pub const CORE_CIP51: u32 = 302055423;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RTOS_SYMBOLS {
    pub name: *const c_char,
    pub optional: c_int,
    pub address: c_uint,
}

#[derive(FromPrimitive)]
pub enum Registers {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,
    LR,
    PC,
    XPSR,
    MSP,
    PSP,
    PRIMASK,
    BASEPRI,
    FAULTMASK,
    CONTROL,
    FPSCR,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    S16,
    S17,
    S18,
    S19,
    S20,
    S21,
    S22,
    S23,
    S24,
    S25,
    S26,
    S27,
    S28,
    S29,
    S30,
    S31,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// GDB Server API that can be used by the plug-in.
pub struct GdbApi {
    /// Frees allocated memory block.
    ///
    /// # Arguments
    /// * `p` - Pointer to the memory block.
    pub free: Option<unsafe extern "C" fn(p: *mut c_void)>,

    /// Allocates a memory block.
    ///
    /// # Arguments
    /// * `size` - Size of requested memory block.
    ///
    /// # Returns
    /// Pointer to the allocated memory block. null if the memory could not be allocated.
    pub malloc: Option<unsafe extern "C" fn(size: c_uint) -> *mut c_void>,

    /// Reallocates a memory block.
    /// If necessary, copies contents of old memory block into new block.
    ///
    /// # Arguments
    /// * `p`    - Pointer to the existing memory block.
    /// * `size` - Size of newly requested memory block.
    ///
    /// # Returns
    /// Pointer to the allocated memory block. null if the memory could not be allocated.
    pub realloc: Option<unsafe extern "C" fn(p: *mut c_void, size: c_uint) -> *mut c_void>,

    /// Prints a formatted log message to the J-Link GDB Server's output.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    pub output: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Prints a formatted log message to the J-Link GDB Server's debug output.
    /// The debug output is disables in non-debug builds of the J-Link GDB Server.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    ///
    /// # Notes
    /// Since debug builds of the J-Link GDB Server are not publicly available, the debug output should not be used.
    pub output_debug: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Prints a formatted log message to the J-Link GDB Server's warning output.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    pub output_warning: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Prints a formatted log message to the J-Link GDB Server's error output.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    pub output_error: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Reads a byte array from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    /// * `size` - Number of bytes to read.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_byte_array:
        Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_char, size: c_uint) -> c_int>,

    /// Reads one byte from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_u8: Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_uchar) -> c_char>,

    /// Reads two bytes from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_u16: Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_ushort) -> c_char>,

    /// Reads four bytes from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_u32: Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_uint) -> c_char>,

    /// Writes a byte array to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    /// * `size` - Number of bytes to write.
    ///
    /// # Returns
    /// * `0`  - Writing memory OK.
    /// * `<0` - Writing memory failed.
    pub write_byte_array:
        Option<unsafe extern "C" fn(addr: c_uint, buff: *const c_char, size: c_uint) -> c_int>,

    /// Writes one byte to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    pub write_u8: Option<unsafe extern "C" fn(Addr: c_uint, Data: c_uchar)>,

    /// Writes two bytes to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    pub write_u16: Option<unsafe extern "C" fn(Addr: c_uint, Data: c_ushort)>,

    /// Writes four bytes to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    pub write_u32: Option<unsafe extern "C" fn(Addr: c_uint, Data: c_uint)>,

    /// Converts two bytes according to the target's endianness.
    ///
    /// # Arguments
    /// * `p` - Pointer to a u16
    ///
    /// # Returns
    /// The converted value.
    pub convert_u16: Option<unsafe extern "C" fn(p: *const c_uchar) -> c_uint>,

    /// Converts three bytes according to the target's endianness.
    ///
    /// # Arguments
    /// * `p` - Pointer to a u24
    ///
    /// # Returns
    /// The converted value.
    pub convert_u24: Option<unsafe extern "C" fn(p: *const c_uchar) -> c_uint>,

    /// Converts four bytes according to the target's endianness.
    ///
    /// # Arguments
    /// * `p` - Pointer to a u32
    ///
    /// # Returns
    /// The converted value.
    pub convert_u32: Option<unsafe extern "C" fn(p: *const c_uchar) -> c_uint>,

    /// Reads a register from the target's CPU.
    ///
    /// # Arguments
    /// * `reg_index` - Index of the register, refer to [Registers](crate::bindings::jlink::Registers)
    ///
    /// # Returns
    /// Value of the register
    pub read_register: Option<unsafe extern "C" fn(reg_index: c_uint) -> c_uint>,

    /// Writes a value to the target's CPU register.
    ///
    /// # Arguments
    /// * `reg_index` - Index of the register, refer to [Registers](crate::bindings::jlink::Registers)
    /// * `value`     - Value of the register
    pub write_register: Option<unsafe extern "C" fn(reg_index: c_uint, value: c_uint)>,

    /// WTF Segger?! Just WTF?
    pub dummy: *mut c_void,
}
