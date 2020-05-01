//! Provides information about the state of the CPU of the debugged device.

use crate::api;
use crate::bindings::jlink;

/* ------------------------------------- Types ---------------------------------------------------------------------- */

struct Core {
    has_fpu: bool,
}

type Reg = u32;

type FloatReg = f32;

#[repr(C)]
#[derive(Copy, Clone)]
/// General registers of Cortex-M0 to M3 MCUs
pub struct GeneralRegs {
    r0: Reg,
    r1: Reg,
    r2: Reg,
    r3: Reg,
    r4: Reg,
    r5: Reg,
    r6: Reg,
    r7: Reg,
    r8: Reg,
    r9: Reg,
    r10: Reg,
    r11: Reg,
    r12: Reg,
    sp: Reg,
    lr: Reg,
    pc: Reg,
    xpsr: Reg,
    msp: Reg,
    psp: Reg,
    primask: Reg,
    basepri: Reg,
    faultmask: Reg,
    control: Reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// General registers of Cortex-M4 MCUs
pub struct GeneralRegsFpu {
    general: GeneralRegs,
    s0: FloatReg,
    s1: FloatReg,
    s2: FloatReg,
    s3: FloatReg,
    s4: FloatReg,
    s5: FloatReg,
    s6: FloatReg,
    s7: FloatReg,
    s8: FloatReg,
    s9: FloatReg,
    s10: FloatReg,
    s11: FloatReg,
    s12: FloatReg,
    s13: FloatReg,
    s14: FloatReg,
    s15: FloatReg,
    s16: FloatReg,
    s17: FloatReg,
    s18: FloatReg,
    s19: FloatReg,
    s20: FloatReg,
    s21: FloatReg,
    s22: FloatReg,
    s23: FloatReg,
    s24: FloatReg,
    s25: FloatReg,
    s26: FloatReg,
    s27: FloatReg,
    s28: FloatReg,
    s29: FloatReg,
    s30: FloatReg,
    s31: FloatReg,
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
