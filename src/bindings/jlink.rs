//! Bindings auto generated from SEGGER RTOS Plug-in S

use std::option::Option;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void};

use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Core {
    NONE = 0x00000000,
    ANY = 0xFFFFFFFF,
    CORTEX_M1 = 0x010000FF,
    COLDFIRE = 0x02FFFFFF,
    CORTEX_M3 = 0x030000FF,
    CORTEX_M3_R1P0 = 0x03000010,
    CORTEX_M3_R1P1 = 0x03000011,
    CORTEX_M3_R2P0 = 0x03000020,
    SIM = 0x04FFFFFF,
    XSCALE = 0x05FFFFFF,
    CORTEX_M0 = 0x060000FF,
    CORTEX_M_V8BASEL = 0x060100FF,
    ARM7 = 0x07FFFFFF,
    ARM7TDMI = 0x070000FF,
    ARM7TDMI_R3 = 0x0700003F,
    ARM7TDMI_R4 = 0x0700004F,
    ARM7TDMI_S = 0x070001FF,
    ARM7TDMI_S_R3 = 0x0700013F,
    ARM7TDMI_S_R4 = 0x0700014F,
    CORTEX_A8 = 0x080000FF,
    CORTEX_A7 = 0x080800FF, // Same family as Cortex-A9. Only low-level differences
    CORTEX_A9 = 0x080900FF, // Cortex-A9. Cortex-A8 compatible only small differences for future multi-core debugging support.
    CORTEX_A12 = 0x080A00FF, // Same family as Cortex-A9. Only low-level differences
    CORTEX_A15 = 0x080B00FF, // Same family as Cortex-A9. Only low-level differences
    CORTEX_A17 = 0x080C00FF, // Same family as Cortex-A9. Only low-level differences
    ARM9 = 0x09FFFFFF,
    ARM9TDMI_S = 0x090001FF,
    ARM920T = 0x092000FF,
    ARM922T = 0x092200FF,
    ARM926EJ_S = 0x092601FF,
    ARM946E_S = 0x094601FF,
    ARM966E_S = 0x096601FF,
    ARM968E_S = 0x096801FF,
    ARM11 = 0x0BFFFFFF,
    ARM1136 = 0x0B36FFFF,
    ARM1136J = 0x0B3602FF,
    ARM1136J_S = 0x0B3603FF,
    ARM1136JF = 0x0B3606FF,
    ARM1136JF_S = 0x0B3607FF,
    ARM1156 = 0x0B56FFFF,
    ARM1176 = 0x0B76FFFF,
    ARM1176J = 0x0B7602FF,
    ARM1176J_S = 0x0B7603FF,
    ARM1176JF = 0x0B7606FF,
    ARM1176JF_S = 0x0B7607FF,
    CORTEX_R4 = 0x0C0000FF, // Device family: 0x0D => Cortex-R, sub-family 0x00: Cortex-R4
    CORTEX_R5 = 0x0C0100FF, // Device family: 0x0D => Cortex-R, sub-family 0x01: Cortex-R5
    RX = 0x0DFFFFFF,        // Device family: 0x0D, sub family not specified, revision not specified
    RX610 = 0x0D00FFFF,     // Device family: 0x0D, sub family 0x00, revision not specified
    RX62N = 0x0D01FFFF,     // Device family: 0x0D, sub family 0x01, revision not specified
    RX62T = 0x0D02FFFF,
    RX63N = 0x0D03FFFF,
    RX630 = 0x0D04FFFF,
    RX63T = 0x0D05FFFF,
    RX621 = 0x0D06FFFF,
    RX62G = 0x0D07FFFF,
    RX631 = 0x0D08FFFF,
    RX65N = 0x0D09FFFF,
    RX210 = 0x0D10FFFF,
    RX21A = 0x0D11FFFF,
    RX220 = 0x0D12FFFF,
    RX230 = 0x0D13FFFF,
    RX231 = 0x0D14FFFF,
    RX23T = 0x0D15FFFF,
    RX24T = 0x0D16FFFF,
    RX111 = 0x0D20FFFF, // Device family: 0x0D, sub family 0x20, revision not specified
    RX110 = 0x0D21FFFF,
    RX113 = 0x0D22FFFF,
    RX130 = 0x0D23FFFF,
    RX64M = 0x0D30FFFF,
    RX71M = 0x0D31FFFF,
    CORTEX_M4 = 0x0E0000FF,
    CORTEX_M7 = 0x0E0100FF, // Device family: 0x0E (M4), sub family: 0x01
    CORTEX_M_V8MAINL = 0x0E0200FF,
    CORTEX_A5 = 0x0F0000FF,
    POWER_PC = 0x10FFFFFF,
    POWER_PC_N1 = 0x10FF00FF,     // Core with Nexus-1  support
    POWER_PC_N2 = 0x10FF01FF,     // Core with Nexus-2+ support
    MIPS = 0x11FFFFFF,            // Dev family: MIPS, sub family not specified
    MIPS_M4K = 0x1100FFFF,        // Dev family: MIPS, sub family: 0x00 (M4K core)
    MIPS_MICROAPTIV = 0x1101FFFF, // Dev family: MIPS, sub family: 0x01 (microAptiv core)
    EFM8_UNSPEC = 0x12FFFFFF, // Dev family: SiLabs EFM8, sub family 0xFF (exact core not specified)
    CIP51 = 0x1200FFFF,       // Dev family: SiLabs EFM8, sub family 0x00 (CIP51 core)
}

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
