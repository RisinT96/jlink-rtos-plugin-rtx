use num_traits::FromPrimitive;

use crate::bindings::rtos::{OsRtxThread, OsThreadPriority, OsThreadState};
use crate::device::core::{FloatReg, GeneralRegs, GeneralRegsFpu, Reg};

use crate::host::api;

/* ------------------------------------- Types ---------------------------------------------------------------------- */

pub enum ThreadRegs {
    Some(GeneralRegs),
    SomeFpu(GeneralRegsFpu),
    None,
}

pub struct Thread {
    name: String,
    pub id: u32,
    stack_frame: u8,
    stack_base: u32,
    stack_size: u32,
    stack_pointer: u32,
    priority: OsThreadPriority,
    state: OsThreadState,
    pub(in crate::device) thread_next: u32,
    pub(in crate::device) delay_next: u32,
    regs: ThreadRegs,
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
    r0: Reg,
    r1: Reg,
    r2: Reg,
    r3: Reg,
    r12: Reg,
    lr: Reg,
    pc: Reg,
    xpsr: Reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Structure of thread's registers when they're stacked on context switch.
struct GeneralRegsStacked {
    r4: Reg,
    r5: Reg,
    r6: Reg,
    r7: Reg,
    r8: Reg,
    r9: Reg,
    r10: Reg,
    r11: Reg,
    hw_stacked: GeneralRegsStackedHw,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Structure of thread's registers with FPU enabled, when they're stacked on IRQ, the rest are kept in the CPU.
struct GeneralRegsFpuStackedHw {
    r0: Reg,
    r1: Reg,
    r2: Reg,
    r3: Reg,
    r12: Reg,
    lr: Reg,
    pc: Reg,
    xpsr: Reg,
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
    fpscr: Reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Structure of thread's registers when they're stacked on context switch.
struct GeneralRegsFpuStacked {
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
    r4: Reg,
    r5: Reg,
    r6: Reg,
    r7: Reg,
    r8: Reg,
    r9: Reg,
    r10: Reg,
    r11: Reg,
    hw_stacked: GeneralRegsFpuStackedHw,
}

/* ------------------------------------- Implementations ------------------------------------------------------------ */

impl Thread {
    pub fn new(address: u32) -> Result<Thread, i32> {
        trace!("Loading Thread Info from {:#X}", address);

        let thread_info: OsRtxThread = api::read_mem(address)?;

        let name_addr = api::convert_u32(thread_info.name)?;

        let mut name = String::new();
        if name_addr != 0 {
            name = api::read_string(name_addr, 256)?;
        }

        let priority: OsThreadPriority = match FromPrimitive::from_i8(thread_info.priority) {
            Some(val) => val,
            _ => return Err(api::GDB_ERR),
        };

        let state: OsThreadState = match FromPrimitive::from_u8(thread_info.state) {
            Some(val) => val,
            _ => return Err(api::GDB_ERR),
        };

        Ok(Thread {
            name: name,

            id: address,
            stack_frame: thread_info.stack_frame,
            stack_base: api::convert_u32(thread_info.stack_mem)?,
            stack_size: api::convert_u32(thread_info.stack_size)?,
            stack_pointer: api::convert_u32(thread_info.sp)?,

            thread_next: api::convert_u32(thread_info.thread_next)?,
            delay_next: api::convert_u32(thread_info.delay_next)?,

            priority: priority,
            state: state,
            regs: ThreadRegs::None,
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

        let thread = Thread::new(self.next_thread_addr).unwrap();

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

        let thread = Thread::new(self.next_thread_addr).unwrap();

        self.next_thread_addr = api::convert_u32(thread.delay_next).unwrap();

        return Some(thread);
    }
}
