use num_traits::FromPrimitive;

use crate::bindings::jlink;
use crate::bindings::rtos::{OsRtxThread, OsThreadPriority, OsThreadState};
use crate::device::core;
use crate::device::core::{GeneralRegs, GeneralRegsFpu};

use crate::host::api;

/* ------------------------------------- Types ---------------------------------------------------------------------- */

#[derive(Debug)]
pub enum ThreadRegs {
    Some(GeneralRegs),
    SomeFpu(GeneralRegsFpu),
    None,
}

pub struct Thread {
    pub id: u32,
    pub regs: ThreadRegs,
    name: String,
    priority: OsThreadPriority,
    state: OsThreadState,
    pub(in crate::device) thread_next: u32,
    pub(in crate::device) delay_next: u32,
}

pub(in crate::device) struct ThreadReadyList {
    next_thread_addr: u32,
}

pub(in crate::device) struct ThreadDelayList {
    next_thread_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Structure of thread's registers when they're stacked on IRQ, the rest are kept in the CPU.
struct GeneralRegsStackedHw {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Structure of thread's registers when they're stacked on context switch.
struct GeneralRegsStacked {
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    hw_stacked: GeneralRegsStackedHw,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Structure of thread's registers with FPU enabled, when they're stacked on IRQ, the rest are kept in the CPU.
struct GeneralRegsFpuStackedHw {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    s12: u32,
    s13: u32,
    s14: u32,
    s15: u32,
    fpscr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Structure of thread's registers when they're stacked on context switch.
struct GeneralRegsFpuStacked {
    s16: u32,
    s17: u32,
    s18: u32,
    s19: u32,
    s20: u32,
    s21: u32,
    s22: u32,
    s23: u32,
    s24: u32,
    s25: u32,
    s26: u32,
    s27: u32,
    s28: u32,
    s29: u32,
    s30: u32,
    s31: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    hw_stacked: GeneralRegsFpuStackedHw,
}

/* ------------------------------------- Implementations ------------------------------------------------------------ */

impl Thread {
    pub fn new(address: u32, is_current: bool) -> Result<Thread, i32> {
        trace!("Loading Thread Info from {:#X}", address);

        let thread_info: OsRtxThread = api::read_mem(address)?;

        // Load thread name.
        let name_addr = api::convert_u32(thread_info.name)?;
        let mut name = String::new();
        if name_addr != 0 {
            name = api::read_string(name_addr, 256)?;
        }

        // Load thread priority.
        let priority: OsThreadPriority = match FromPrimitive::from_i8(thread_info.priority) {
            Some(val) => val,
            _ => return Err(api::GDB_ERR),
        };

        // Load thread state.
        let state: OsThreadState = match FromPrimitive::from_u8(thread_info.state) {
            Some(val) => val,
            _ => return Err(api::GDB_ERR),
        };

        //Load thread registers.
        let regs;

        let is_in_irq = core::is_in_irq()?;
        let has_fpu = core::has_fpu()?;

        if is_current && !is_in_irq {
            /* If the thread is currently running, and not in IRQ, the registers should be read directly from the CPU */
            regs = ThreadRegs::None;
        } else if !is_current {
            /* If the thread is currently not running, the registers should be read from its stack */
            let sp = api::convert_u32(thread_info.sp)?;
            let lr = thread_info.stack_frame;
            let is_fpu_used = has_fpu && ((lr & (1 << 4)) == 0);

            if is_fpu_used {
                regs = load_thread_registers_fpu_all(sp)?;
            } else {
                regs = load_thread_registers_all(sp)?;
            }
        } else {
            /* If the thread is currently running, but in IRQ, some registers should be read directly from the CPU,
            while others from the stack */

            // Assume PSP is still valid
            let sp = api::read_reg(jlink::RegName::PSP as u32)?;

            let lr;
            if (core::HaltReason::VCatch as u32 & core::get_halt_reason()?) != 0 {
                // Vector catch just occured, take live LR.
                lr = api::read_reg(jlink::RegName::LR as u32)? as u8;
            } else {
                // Otherwise take LR from previous context switch and hope it's up to date.
                lr = thread_info.stack_frame;
            }

            let is_fpu_used = has_fpu && ((lr & (1 << 4)) == 0);

            if is_fpu_used {
                regs = load_thread_registers_fpu_hw(sp)?;
            } else {
                regs = load_thread_registers_hw(sp)?;
            }
        }

        Ok(Thread {
            name: name,
            id: address,
            priority: priority,
            state: state,
            regs: regs,

            thread_next: api::convert_u32(thread_info.thread_next)?,
            delay_next: api::convert_u32(thread_info.delay_next)?,
        })
    }
}

impl std::fmt::Display for Thread {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.name.len() != 0 {
            write!(
                f,
                "{name} <{id:#010X}> [{priority}] ({state})",
                id = self.id,
                name = self.name,
                priority = self.priority,
                state = self.state
            )
        } else {
            write!(
                f,
                "<{id:#010X}> [{priority}] ({state})",
                id = self.id,
                priority = self.priority,
                state = self.state
            )
        }
    }
}

impl ThreadReadyList {
    pub fn new(address: u32) -> ThreadReadyList {
        ThreadReadyList {
            next_thread_addr: address,
        }
    }
}

impl ThreadDelayList {
    pub fn new(address: u32) -> ThreadDelayList {
        ThreadDelayList {
            next_thread_addr: address,
        }
    }
}

impl Iterator for ThreadReadyList {
    type Item = Thread;

    fn next(&mut self) -> Option<Thread> {
        if self.next_thread_addr as *const u32 == std::ptr::null() {
            return None;
        }

        let thread = Thread::new(self.next_thread_addr, false).unwrap();

        self.next_thread_addr = api::convert_u32(thread.thread_next).unwrap();

        return Some(thread);
    }
}

impl Iterator for ThreadDelayList {
    type Item = Thread;

    fn next(&mut self) -> Option<Thread> {
        if self.next_thread_addr as *const u32 == std::ptr::null() {
            return None;
        }

        let thread = Thread::new(self.next_thread_addr, false).unwrap();

        self.next_thread_addr = api::convert_u32(thread.delay_next).unwrap();

        return Some(thread);
    }
}

/* ------------------------------------- Private Functions ---------------------------------------------------------- */

fn load_thread_registers_all(sp: u32) -> Result<ThreadRegs, i32> {
    let regs: GeneralRegsStacked = api::read_mem(sp)?;

    Ok(ThreadRegs::Some(GeneralRegs {
        r0: api::convert_u32(regs.hw_stacked.r0)?,
        r1: api::convert_u32(regs.hw_stacked.r1)?,
        r2: api::convert_u32(regs.hw_stacked.r2)?,
        r3: api::convert_u32(regs.hw_stacked.r3)?,
        r4: api::convert_u32(regs.r4)?,
        r5: api::convert_u32(regs.r5)?,
        r6: api::convert_u32(regs.r6)?,
        r7: api::convert_u32(regs.r7)?,
        r8: api::convert_u32(regs.r8)?,
        r9: api::convert_u32(regs.r9)?,
        r10: api::convert_u32(regs.r10)?,
        r11: api::convert_u32(regs.r11)?,
        r12: api::convert_u32(regs.hw_stacked.r12)?,
        sp: sp + std::mem::size_of::<GeneralRegsStacked>() as u32,
        lr: api::convert_u32(regs.hw_stacked.lr)?,
        pc: api::convert_u32(regs.hw_stacked.pc)?,
        xpsr: api::convert_u32(regs.hw_stacked.xpsr)?,
        msp: api::read_reg(jlink::RegName::MSP as u32)?,
        psp: api::read_reg(jlink::RegName::PSP as u32)?,
        primask: api::read_reg(jlink::RegName::PRIMASK as u32)?,
        basepri: api::read_reg(jlink::RegName::BASEPRI as u32)?,
        faultmask: api::read_reg(jlink::RegName::FAULTMASK as u32)?,
        control: api::read_reg(jlink::RegName::CONTROL as u32)?,
    }))
}

fn load_thread_registers_hw(sp: u32) -> Result<ThreadRegs, i32> {
    let regs: GeneralRegsStackedHw = api::read_mem(sp)?;

    Ok(ThreadRegs::Some(GeneralRegs {
        r0: api::convert_u32(regs.r0)?,
        r1: api::convert_u32(regs.r1)?,
        r2: api::convert_u32(regs.r2)?,
        r3: api::convert_u32(regs.r3)?,
        r4: api::read_reg(jlink::RegName::R4 as u32)?,
        r5: api::read_reg(jlink::RegName::R5 as u32)?,
        r6: api::read_reg(jlink::RegName::R6 as u32)?,
        r7: api::read_reg(jlink::RegName::R7 as u32)?,
        r8: api::read_reg(jlink::RegName::R8 as u32)?,
        r9: api::read_reg(jlink::RegName::R9 as u32)?,
        r10: api::read_reg(jlink::RegName::R10 as u32)?,
        r11: api::read_reg(jlink::RegName::R11 as u32)?,
        r12: api::convert_u32(regs.r12)?,
        sp: sp + std::mem::size_of::<GeneralRegsStackedHw>() as u32,
        lr: api::convert_u32(regs.lr)?,
        pc: api::convert_u32(regs.pc)?,
        xpsr: api::convert_u32(regs.xpsr)?,
        msp: api::read_reg(jlink::RegName::MSP as u32)?,
        psp: api::read_reg(jlink::RegName::PSP as u32)?,
        primask: api::read_reg(jlink::RegName::PRIMASK as u32)?,
        basepri: api::read_reg(jlink::RegName::BASEPRI as u32)?,
        faultmask: api::read_reg(jlink::RegName::FAULTMASK as u32)?,
        control: api::read_reg(jlink::RegName::CONTROL as u32)?,
    }))
}

fn load_thread_registers_fpu_all(sp: u32) -> Result<ThreadRegs, i32> {
    let regs: GeneralRegsFpuStacked = api::read_mem(sp)?;

    let general_regs = GeneralRegs {
        r0: api::convert_u32(regs.hw_stacked.r0)?,
        r1: api::convert_u32(regs.hw_stacked.r1)?,
        r2: api::convert_u32(regs.hw_stacked.r2)?,
        r3: api::convert_u32(regs.hw_stacked.r3)?,
        r4: api::convert_u32(regs.r4)?,
        r5: api::convert_u32(regs.r5)?,
        r6: api::convert_u32(regs.r6)?,
        r7: api::convert_u32(regs.r7)?,
        r8: api::convert_u32(regs.r8)?,
        r9: api::convert_u32(regs.r9)?,
        r10: api::convert_u32(regs.r10)?,
        r11: api::convert_u32(regs.r11)?,
        r12: api::convert_u32(regs.hw_stacked.r12)?,
        sp: sp + std::mem::size_of::<GeneralRegsFpuStacked>() as u32,
        lr: api::convert_u32(regs.hw_stacked.lr)?,
        pc: api::convert_u32(regs.hw_stacked.pc)?,
        xpsr: api::convert_u32(regs.hw_stacked.xpsr)?,
        msp: api::read_reg(jlink::RegName::MSP as u32)?,
        psp: api::read_reg(jlink::RegName::PSP as u32)?,
        primask: api::read_reg(jlink::RegName::PRIMASK as u32)?,
        basepri: api::read_reg(jlink::RegName::BASEPRI as u32)?,
        faultmask: api::read_reg(jlink::RegName::FAULTMASK as u32)?,
        control: api::read_reg(jlink::RegName::CONTROL as u32)?,
    };

    Ok(ThreadRegs::SomeFpu(GeneralRegsFpu {
        general: general_regs,
        fpscr: api::convert_u32(regs.hw_stacked.fpscr)?,
        s0: api::convert_u32(regs.hw_stacked.s0)?,
        s1: api::convert_u32(regs.hw_stacked.s1)?,
        s2: api::convert_u32(regs.hw_stacked.s2)?,
        s3: api::convert_u32(regs.hw_stacked.s3)?,
        s4: api::convert_u32(regs.hw_stacked.s4)?,
        s5: api::convert_u32(regs.hw_stacked.s5)?,
        s6: api::convert_u32(regs.hw_stacked.s6)?,
        s7: api::convert_u32(regs.hw_stacked.s7)?,
        s8: api::convert_u32(regs.hw_stacked.s8)?,
        s9: api::convert_u32(regs.hw_stacked.s9)?,
        s10: api::convert_u32(regs.hw_stacked.s10)?,
        s11: api::convert_u32(regs.hw_stacked.s11)?,
        s12: api::convert_u32(regs.hw_stacked.s12)?,
        s13: api::convert_u32(regs.hw_stacked.s13)?,
        s14: api::convert_u32(regs.hw_stacked.s14)?,
        s15: api::convert_u32(regs.hw_stacked.s15)?,
        s16: api::convert_u32(regs.s16)?,
        s17: api::convert_u32(regs.s17)?,
        s18: api::convert_u32(regs.s18)?,
        s19: api::convert_u32(regs.s19)?,
        s20: api::convert_u32(regs.s20)?,
        s21: api::convert_u32(regs.s21)?,
        s22: api::convert_u32(regs.s22)?,
        s23: api::convert_u32(regs.s23)?,
        s24: api::convert_u32(regs.s24)?,
        s25: api::convert_u32(regs.s25)?,
        s26: api::convert_u32(regs.s26)?,
        s27: api::convert_u32(regs.s27)?,
        s28: api::convert_u32(regs.s28)?,
        s29: api::convert_u32(regs.s29)?,
        s30: api::convert_u32(regs.s30)?,
        s31: api::convert_u32(regs.s31)?,
    }))
}

fn load_thread_registers_fpu_hw(sp: u32) -> Result<ThreadRegs, i32> {
    let regs: GeneralRegsFpuStackedHw = api::read_mem(sp)?;

    let general_regs = GeneralRegs {
        r0: api::convert_u32(regs.r0)?,
        r1: api::convert_u32(regs.r1)?,
        r2: api::convert_u32(regs.r2)?,
        r3: api::convert_u32(regs.r3)?,
        r4: api::read_reg(jlink::RegName::R4 as u32)?,
        r5: api::read_reg(jlink::RegName::R5 as u32)?,
        r6: api::read_reg(jlink::RegName::R6 as u32)?,
        r7: api::read_reg(jlink::RegName::R7 as u32)?,
        r8: api::read_reg(jlink::RegName::R8 as u32)?,
        r9: api::read_reg(jlink::RegName::R9 as u32)?,
        r10: api::read_reg(jlink::RegName::R10 as u32)?,
        r11: api::read_reg(jlink::RegName::R11 as u32)?,
        r12: api::convert_u32(regs.r12)?,
        sp: sp + std::mem::size_of::<GeneralRegsFpuStackedHw>() as u32,
        lr: api::convert_u32(regs.lr)?,
        pc: api::convert_u32(regs.pc)?,
        xpsr: api::convert_u32(regs.xpsr)?,
        msp: api::read_reg(jlink::RegName::MSP as u32)?,
        psp: api::read_reg(jlink::RegName::PSP as u32)?,
        primask: api::read_reg(jlink::RegName::PRIMASK as u32)?,
        basepri: api::read_reg(jlink::RegName::BASEPRI as u32)?,
        faultmask: api::read_reg(jlink::RegName::FAULTMASK as u32)?,
        control: api::read_reg(jlink::RegName::CONTROL as u32)?,
    };

    Ok(ThreadRegs::SomeFpu(GeneralRegsFpu {
        general: general_regs,
        fpscr: api::convert_u32(regs.fpscr)?,
        s0: api::convert_u32(regs.s0)?,
        s1: api::convert_u32(regs.s1)?,
        s2: api::convert_u32(regs.s2)?,
        s3: api::convert_u32(regs.s3)?,
        s4: api::convert_u32(regs.s4)?,
        s5: api::convert_u32(regs.s5)?,
        s6: api::convert_u32(regs.s6)?,
        s7: api::convert_u32(regs.s7)?,
        s8: api::convert_u32(regs.s8)?,
        s9: api::convert_u32(regs.s9)?,
        s10: api::convert_u32(regs.s10)?,
        s11: api::convert_u32(regs.s11)?,
        s12: api::convert_u32(regs.s12)?,
        s13: api::convert_u32(regs.s13)?,
        s14: api::convert_u32(regs.s14)?,
        s15: api::convert_u32(regs.s15)?,
        s16: api::read_reg(jlink::RegName::S16 as u32)?,
        s17: api::read_reg(jlink::RegName::S17 as u32)?,
        s18: api::read_reg(jlink::RegName::S18 as u32)?,
        s19: api::read_reg(jlink::RegName::S19 as u32)?,
        s20: api::read_reg(jlink::RegName::S20 as u32)?,
        s21: api::read_reg(jlink::RegName::S21 as u32)?,
        s22: api::read_reg(jlink::RegName::S22 as u32)?,
        s23: api::read_reg(jlink::RegName::S23 as u32)?,
        s24: api::read_reg(jlink::RegName::S24 as u32)?,
        s25: api::read_reg(jlink::RegName::S25 as u32)?,
        s26: api::read_reg(jlink::RegName::S26 as u32)?,
        s27: api::read_reg(jlink::RegName::S27 as u32)?,
        s28: api::read_reg(jlink::RegName::S28 as u32)?,
        s29: api::read_reg(jlink::RegName::S29 as u32)?,
        s30: api::read_reg(jlink::RegName::S30 as u32)?,
        s31: api::read_reg(jlink::RegName::S31 as u32)?,
    }))
}
