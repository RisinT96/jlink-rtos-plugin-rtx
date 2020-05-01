//! Provides information about the state of the CPU of the debugged device.

use num_derive::FromPrimitive;

use crate::api;
use crate::bindings::jlink;

/* ------------------------------------- Types ---------------------------------------------------------------------- */

struct Core {
    has_fpu: bool,
}

pub type Reg = u32;

pub type FloatReg = f32;

/// General registers of Cortex-M0 to M3 MCUs
pub struct GeneralRegs {
    pub r0: Reg,
    pub r1: Reg,
    pub r2: Reg,
    pub r3: Reg,
    pub r4: Reg,
    pub r5: Reg,
    pub r6: Reg,
    pub r7: Reg,
    pub r8: Reg,
    pub r9: Reg,
    pub r10: Reg,
    pub r11: Reg,
    pub r12: Reg,
    pub sp: Reg,
    pub lr: Reg,
    pub pc: Reg,
    pub xpsr: Reg,
    pub msp: Reg,
    pub psp: Reg,
    pub primask: Reg,
    pub basepri: Reg,
    pub faultmask: Reg,
    pub control: Reg,
}

/// General registers of Cortex-M4 MCUs
pub struct GeneralRegsFpu {
    pub general: GeneralRegs,
    pub s0: FloatReg,
    pub s1: FloatReg,
    pub s2: FloatReg,
    pub s3: FloatReg,
    pub s4: FloatReg,
    pub s5: FloatReg,
    pub s6: FloatReg,
    pub s7: FloatReg,
    pub s8: FloatReg,
    pub s9: FloatReg,
    pub s10: FloatReg,
    pub s11: FloatReg,
    pub s12: FloatReg,
    pub s13: FloatReg,
    pub s14: FloatReg,
    pub s15: FloatReg,
    pub s16: FloatReg,
    pub s17: FloatReg,
    pub s18: FloatReg,
    pub s19: FloatReg,
    pub s20: FloatReg,
    pub s21: FloatReg,
    pub s22: FloatReg,
    pub s23: FloatReg,
    pub s24: FloatReg,
    pub s25: FloatReg,
    pub s26: FloatReg,
    pub s27: FloatReg,
    pub s28: FloatReg,
    pub s29: FloatReg,
    pub s30: FloatReg,
    pub s31: FloatReg,
}

#[derive(FromPrimitive)]
pub enum HaltReason {
    Halted = (1 << 0),
    Bkpt = (1 << 1),
    DwtTrap = (1 << 2),
    VCatch = (1 << 3),
    External = (1 << 4),
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
pub fn init(core: u32) -> Result<(), i32> {
    match core {
        jlink::CORTEX_M0 => info!("Core: Cortex-M0"),
        jlink::CORTEX_M1 => info!("Core: Cortex-M1"),
        jlink::CORTEX_M3 => info!("Core: Cortex-M3"),
        jlink::CORTEX_M4 => info!("Core: Cortex-M4"),
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
    let xpsr =
        api::read_reg(jlink::RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_XPSR)?;

    let ipsr = xpsr & 0b111111111;

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
fn find_fpu(core: u32) -> Result<bool, i32> {
    match core {
        jlink::CORTEX_M0 | jlink::CORTEX_M1 | jlink::CORTEX_M3 => return Ok(false),

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
