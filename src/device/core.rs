//! Provides information about the state of the CPU of the debugged device.

use crate::api;
use crate::bindings::jlink;

struct Core {
    has_fpu: bool,
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
