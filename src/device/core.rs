//! Provides information about the state of the CPU of the debugged device.

use byteorder::{ByteOrder, NativeEndian};
use num_derive::FromPrimitive;

use crate::api;
use crate::bindings::jlink;

/* ------------------------------------- Types ---------------------------------------------------------------------- */

struct Core {
    has_fpu: bool,
}

#[derive(Debug)]
#[repr(C)]
/// General registers of Cortex-M0 to M3 MCUs
pub struct GeneralRegs {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub xpsr: u32,
    pub msp: u32,
    pub psp: u32,
    pub primask: u32,
    pub basepri: u32,
    pub faultmask: u32,
    pub control: u32,
}

#[derive(Debug)]
#[repr(C)]
/// General registers of Cortex-M4 MCUs
pub struct GeneralRegsFpu {
    pub general: GeneralRegs,
    pub fpscr: u32,
    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,
    pub s8: u32,
    pub s9: u32,
    pub s10: u32,
    pub s11: u32,
    pub s12: u32,
    pub s13: u32,
    pub s14: u32,
    pub s15: u32,
    pub s16: u32,
    pub s17: u32,
    pub s18: u32,
    pub s19: u32,
    pub s20: u32,
    pub s21: u32,
    pub s22: u32,
    pub s23: u32,
    pub s24: u32,
    pub s25: u32,
    pub s26: u32,
    pub s27: u32,
    pub s28: u32,
    pub s29: u32,
    pub s30: u32,
    pub s31: u32,
}

#[derive(FromPrimitive)]
pub enum HaltReason {
    Halted = (1 << 0),
    Bkpt = (1 << 1),
    DwtTrap = (1 << 2),
    VCatch = (1 << 3),
    External = (1 << 4),
}

/* ------------------------------------- Implementations ------------------------------------------------------------ */

impl std::fmt::Display for GeneralRegs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let self_words_ptr = (self as *const GeneralRegs) as *const u32;
        let self_words_slice =
            unsafe { std::slice::from_raw_parts(self_words_ptr, api::GDB_REG_LIST_LEN) };

        let mut self_bytes_vec = vec![0; api::GDB_REG_LIST_LEN * 4];
        NativeEndian::write_u32_into(self_words_slice, &mut self_bytes_vec);

        let self_str = hex::encode(self_bytes_vec);

        f.write_str(&self_str)
    }
}

impl std::fmt::Display for GeneralRegsFpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let self_words_ptr = (self as *const GeneralRegsFpu) as *const u32;
        let self_words_slice =
            unsafe { std::slice::from_raw_parts(self_words_ptr, api::GDB_REG_LIST_LEN) };

        let mut self_bytes_vec = vec![0; api::GDB_REG_LIST_LEN * 4];
        NativeEndian::write_u32_into(self_words_slice, &mut self_bytes_vec);

        let self_str = hex::encode(self_bytes_vec);

        f.write_str(&self_str)
    }
}

/* ------------------------------------- State variable and getter/setter ------------------------------------------- */
static mut CORE: Option<Core> = None;

/// Get non-mutable reference to the core information.
fn core_get() -> Option<&'static Core> {
    unsafe { CORE.as_ref() }
}

/// Override core information.
unsafe fn core_set(core: Core) {
    CORE = Some(core);
}

/* ------------------------------------- Public Functions ----------------------------------------------------------- */

/// Initialize the core submodule.
pub fn init(core: jlink::Core) -> Result<(), i32> {
    match core {
        jlink::Core::CortexM0 => info!("Core: Cortex-M0"),
        jlink::Core::CortexM1 => info!("Core: Cortex-M1"),
        jlink::Core::CortexM3
        | jlink::Core::CortexM3r1p0
        | jlink::Core::CortexM3r1p1
        | jlink::Core::CortexM3r2p0 => info!("Core: Cortex-M3"),
        jlink::Core::CortexM4 => info!("Core: Cortex-M4"),
        _ => return Err(api::GDB_ERR),
    };

    let has_fpu = find_fpu(core)?;

    if has_fpu {
        info!("FPU: present")
    } else {
        info!("FPU: not present")
    }

    unsafe {
        core_set(Core { has_fpu: has_fpu });
    };

    Ok(())
}

/// Check if the core is in interrupt state
pub fn is_in_irq() -> Result<bool, i32> {
    let xpsr = api::read_reg(jlink::Registers::XPSR as u32)?;

    let ipsr = xpsr & 0x1FF;

    Ok(ipsr > 0)
}

/// Check if the core has an FPU.
pub fn has_fpu() -> Result<bool, i32> {
    if let Some(core) = core_get() {
        return Ok(core.has_fpu);
    }

    Err(api::GDB_ERR)
}

pub fn get_halt_reason() -> Result<u32, i32> {
    const DFSR: u32 = 0xE000ED30;
    Ok(api::read_u32(DFSR)?)
}

/* ------------------------------------- Private Functions ---------------------------------------------------------- */

/// Find out whether the core has an FPU or not.
fn find_fpu(core: jlink::Core) -> Result<bool, i32> {
    match core {
        jlink::Core::CortexM0
        | jlink::Core::CortexM1
        | jlink::Core::CortexM3
        | jlink::Core::CortexM3r1p0
        | jlink::Core::CortexM3r1p1
        | jlink::Core::CortexM3r2p0 => return Ok(false),

        _ => (),
    };

    // Coprocessor Access Control Register
    const CPACR: u32 = 0xE000ED88;
    const CPACR_CP10_CP11_MASK: u32 = (3 << 20) | (3 << 22);

    // Backup original value of CPACR
    let cpacr_orig = api::read_u32(CPACR)?;

    // Manually enable CPACR FPU bits
    let cpacr = cpacr_orig | CPACR_CP10_CP11_MASK;
    api::write_u32(CPACR, cpacr);

    // If the target has no FPU, it will reset the bits, read.
    let cpacr = api::read_u32(CPACR)?;
    let has_fpu = (cpacr & CPACR_CP10_CP11_MASK) != 0;

    // Restore CPACR
    api::write_u32(CPACR, cpacr_orig);

    Ok(has_fpu)
}
