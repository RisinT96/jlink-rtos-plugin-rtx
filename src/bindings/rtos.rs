//! Bindings auto generated from RTXv5 headers.
//! Also helper types/functions/traits.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
include!(concat!(env!("OUT_DIR"), "/rtx_bindings.rs"));

const osThreadState_t_osThreadWaitingDelay: i32 = (osThreadState_t_osThreadBlocked | 0x10i32);
const osThreadState_t_osThreadWaitingJoin: i32 = (osThreadState_t_osThreadBlocked | 0x20i32);
const osThreadState_t_osThreadWaitingThreadFlags: i32 = (osThreadState_t_osThreadBlocked | 0x30i32);
const osThreadState_t_osThreadWaitingEventFlags: i32 = (osThreadState_t_osThreadBlocked | 0x40i32);
const osThreadState_t_osThreadWaitingMutex: i32 = (osThreadState_t_osThreadBlocked | 0x50i32);
const osThreadState_t_osThreadWaitingSemaphore: i32 = (osThreadState_t_osThreadBlocked | 0x60i32);
const osThreadState_t_osThreadWaitingMemoryPool: i32 = (osThreadState_t_osThreadBlocked | 0x70i32);
const osThreadState_t_osThreadWaitingMessageGet: i32 = (osThreadState_t_osThreadBlocked | 0x80i32);
const osThreadState_t_osThreadWaitingMessagePut: i32 = (osThreadState_t_osThreadBlocked | 0x90i32);

use std::convert::TryFrom;

use crate::host::api;

/* ------------------------------------- Types ---------------------------------------------------------------------- */

pub struct Thread {
    name: String,
    pub id: u32,
    stack_frame: u8,
    stack_base: u32,
    stack_size: u32,
    stack_pointer: u32,
    priority: &'static str,
    state: &'static str,
    pub(in crate::bindings::rtos) thread_next: u32,
    pub(in crate::bindings::rtos) delay_next: u32,
}

struct ThreadReadyList {
    next_thread_addr: u32,
}

struct ThreadDelayList {
    next_thread_addr: u32,
}

#[derive(Default)]
pub struct RtxInfo {
    pub threads: Vec<Thread>,
}

/* ------------------------------------- Thread Implementations ----------------------------------------------------- */
impl Thread {
    pub fn new(address: u32) -> Result<Thread, i32> {
        trace!("Loading Thread Info from {:#X}", address);

        let thread_info: osRtxThread_t = api::read_mem(address)?;

        let name_addr = api::convert_u32(thread_info.name as u32)?;

        let mut name = String::new();
        if (name_addr as *const u32 != std::ptr::null()) {
            name = api::read_string(name_addr, 256)?;
        }

        Ok(Thread {
            name: name,

            id: address,
            stack_frame: thread_info.stack_frame,
            stack_base: api::convert_u32(thread_info.stack_mem as u32)?,
            stack_size: api::convert_u32(thread_info.stack_size)?,
            stack_pointer: api::convert_u32(thread_info.sp)?,

            thread_next: api::convert_u32(thread_info.thread_next as u32)?,
            delay_next: api::convert_u32(thread_info.delay_next as u32)?,

            priority: match thread_info.priority as i32 {
                osPriority_t_osPriorityNone => "None",
                osPriority_t_osPriorityIdle => "Idle",
                osPriority_t_osPriorityLow => "Low",
                osPriority_t_osPriorityLow1 => "Low1",
                osPriority_t_osPriorityLow2 => "Low2",
                osPriority_t_osPriorityLow3 => "Low3",
                osPriority_t_osPriorityLow4 => "Low4",
                osPriority_t_osPriorityLow5 => "Low5",
                osPriority_t_osPriorityLow6 => "Low6",
                osPriority_t_osPriorityLow7 => "Low7",
                osPriority_t_osPriorityBelowNormal => "BelowNormal",
                osPriority_t_osPriorityBelowNormal1 => "BelowNormal1",
                osPriority_t_osPriorityBelowNormal2 => "BelowNormal2",
                osPriority_t_osPriorityBelowNormal3 => "BelowNormal3",
                osPriority_t_osPriorityBelowNormal4 => "BelowNormal4",
                osPriority_t_osPriorityBelowNormal5 => "BelowNormal5",
                osPriority_t_osPriorityBelowNormal6 => "BelowNormal6",
                osPriority_t_osPriorityBelowNormal7 => "BelowNormal7",
                osPriority_t_osPriorityNormal => "Normal",
                osPriority_t_osPriorityNormal1 => "Normal1",
                osPriority_t_osPriorityNormal2 => "Normal2",
                osPriority_t_osPriorityNormal3 => "Normal3",
                osPriority_t_osPriorityNormal4 => "Normal4",
                osPriority_t_osPriorityNormal5 => "Normal5",
                osPriority_t_osPriorityNormal6 => "Normal6",
                osPriority_t_osPriorityNormal7 => "Normal7",
                osPriority_t_osPriorityAboveNormal => "AboveNormal",
                osPriority_t_osPriorityAboveNormal1 => "AboveNormal1",
                osPriority_t_osPriorityAboveNormal2 => "AboveNormal2",
                osPriority_t_osPriorityAboveNormal3 => "AboveNormal3",
                osPriority_t_osPriorityAboveNormal4 => "AboveNormal4",
                osPriority_t_osPriorityAboveNormal5 => "AboveNormal5",
                osPriority_t_osPriorityAboveNormal6 => "AboveNormal6",
                osPriority_t_osPriorityAboveNormal7 => "AboveNormal7",
                osPriority_t_osPriorityHigh => "High",
                osPriority_t_osPriorityHigh1 => "High1",
                osPriority_t_osPriorityHigh2 => "High2",
                osPriority_t_osPriorityHigh3 => "High3",
                osPriority_t_osPriorityHigh4 => "High4",
                osPriority_t_osPriorityHigh5 => "High5",
                osPriority_t_osPriorityHigh6 => "High6",
                osPriority_t_osPriorityHigh7 => "High7",
                osPriority_t_osPriorityRealtime => "Realtime",
                osPriority_t_osPriorityRealtime1 => "Realtime1",
                osPriority_t_osPriorityRealtime2 => "Realtime2",
                osPriority_t_osPriorityRealtime3 => "Realtime3",
                osPriority_t_osPriorityRealtime4 => "Realtime4",
                osPriority_t_osPriorityRealtime5 => "Realtime5",
                osPriority_t_osPriorityRealtime6 => "Realtime6",
                osPriority_t_osPriorityRealtime7 => "Realtime7",
                osPriority_t_osPriorityISR => "ISR",
                _ => "Error",
            },

            state: match thread_info.state as i32 {
                osThreadState_t_osThreadInactive => "Inactive",
                osThreadState_t_osThreadReady => "Ready",
                osThreadState_t_osThreadRunning => "Running",
                osThreadState_t_osThreadBlocked => "Blocked",
                osThreadState_t_osThreadTerminated => "Terminated",
                (osThreadState_t_osThreadWaitingDelay) => "Waiting Delay",
                (osThreadState_t_osThreadWaitingJoin) => "Waiting Join",
                (osThreadState_t_osThreadWaitingThreadFlags) => "Waiting Thread Flags",
                (osThreadState_t_osThreadWaitingEventFlags) => "Waiting Event Flags",
                (osThreadState_t_osThreadWaitingMutex) => "Waiting Mutex",
                (osThreadState_t_osThreadWaitingSemaphore) => "Waiting Semaphore",
                (osThreadState_t_osThreadWaitingMemoryPool) => "Waiting Memory Pool",
                (osThreadState_t_osThreadWaitingMessageGet) => "Waiting Message Get",
                (osThreadState_t_osThreadWaitingMessagePut) => "Waiting Message Put",
                _ => "Error",
            },
        })
    }
}

impl std::fmt::Display for Thread {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if (self.name.len() != 0) {
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
        if (self.next_thread_addr as *const u32 == std::ptr::null()) {
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
        if (self.next_thread_addr as *const u32 == std::ptr::null()) {
            return None;
        }

        let thread = Thread::new(self.next_thread_addr).unwrap();

        self.next_thread_addr = api::convert_u32(thread.delay_next).unwrap();

        return Some(thread);
    }
}

/* ------------------------------------- RtxInfo Implementations ---------------------------------------------------- */
impl RtxInfo {
    pub fn new(address: u32) -> Result<RtxInfo, i32> {
        trace!("Loading RTOS Info from {:#X}", address);
        let rtx_info: osRtxInfo_t = api::read_mem(address)?;

        let version = api::convert_u32(rtx_info.version)?;
        let os_id = api::convert_u32(rtx_info.os_id as u32)?;

        let mut name = String::new();

        // Load RTX Name, if possible
        if (os_id as *const i8 != std::ptr::null()) {
            name = api::read_string(os_id, 256)?;
            debug!("Found RTX '{}', Version {}", name, version);
        } else {
            debug!("Found RTX Version {}", version);
        }

        let mut threads: Vec<Thread> = Vec::new();

        // If we ran before the kernel was even initialized, exit, there are zero threads.
        match rtx_info.kernel.state as i32 {
            osKernelState_t_osKernelInactive | osKernelState_t_osKernelError => {
                debug!("RTX Kernel not initialized yet!");
                return Ok(RtxInfo { threads });
            }
            _ => (),
        };

        debug!("Loading currently running thread");
        threads.push(Thread::new(api::convert_u32(
            rtx_info.thread.run.curr as u32,
        )?)?);
        debug!("Currently running thread: {}", threads[0]);

        debug!("Loading ready list threads");
        for thread in
            ThreadReadyList::new(api::convert_u32(rtx_info.thread.ready.thread_list as u32)?)
        {
            debug!("Found thread: {}", thread);
            threads.push(thread);
        }

        debug!("Loading delay list threads");
        for thread in ThreadDelayList::new(api::convert_u32(rtx_info.thread.delay_list as u32)?) {
            debug!("Found thread: {}", thread);
            threads.push(thread);
        }

        debug!("Loading wait list threads");
        for thread in ThreadDelayList::new(api::convert_u32(rtx_info.thread.wait_list as u32)?) {
            debug!("Found thread: {}", thread);
            threads.push(thread);
        }

        Ok(RtxInfo { threads })
    }

    pub fn threads_num(&self) -> usize {
        self.threads.len()
    }
}
