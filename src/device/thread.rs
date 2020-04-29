use crate::bindings::rtos;
use crate::bindings::rtos::osRtxThread_t;

use crate::host::api;

pub struct Thread {
    name: String,
    pub id: u32,
    stack_frame: u8,
    stack_base: u32,
    stack_size: u32,
    stack_pointer: u32,
    priority: &'static str,
    state: &'static str,
    pub(in crate::device) thread_next: u32,
    pub(in crate::device) delay_next: u32,
}

pub(in crate::device) struct ThreadReadyList {
    next_thread_addr: u32,
}

pub(in crate::device) struct ThreadDelayList {
    next_thread_addr: u32,
}

/* ------------------------------------- Implementations ------------------------------------------------------------ */

impl Thread {
    pub fn new(address: u32) -> Result<Thread, i32> {
        trace!("Loading Thread Info from {:#X}", address);

        let thread_info: osRtxThread_t = api::read_mem(address)?;

        let name_addr = api::convert_u32(thread_info.name as u32)?;

        let mut name = String::new();
        if name_addr as *const u32 != std::ptr::null() {
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
                rtos::osPriority_t_osPriorityNone => "None",
                rtos::osPriority_t_osPriorityIdle => "Idle",
                rtos::osPriority_t_osPriorityLow => "Low",
                rtos::osPriority_t_osPriorityLow1 => "Low1",
                rtos::osPriority_t_osPriorityLow2 => "Low2",
                rtos::osPriority_t_osPriorityLow3 => "Low3",
                rtos::osPriority_t_osPriorityLow4 => "Low4",
                rtos::osPriority_t_osPriorityLow5 => "Low5",
                rtos::osPriority_t_osPriorityLow6 => "Low6",
                rtos::osPriority_t_osPriorityLow7 => "Low7",
                rtos::osPriority_t_osPriorityBelowNormal => "BelowNormal",
                rtos::osPriority_t_osPriorityBelowNormal1 => "BelowNormal1",
                rtos::osPriority_t_osPriorityBelowNormal2 => "BelowNormal2",
                rtos::osPriority_t_osPriorityBelowNormal3 => "BelowNormal3",
                rtos::osPriority_t_osPriorityBelowNormal4 => "BelowNormal4",
                rtos::osPriority_t_osPriorityBelowNormal5 => "BelowNormal5",
                rtos::osPriority_t_osPriorityBelowNormal6 => "BelowNormal6",
                rtos::osPriority_t_osPriorityBelowNormal7 => "BelowNormal7",
                rtos::osPriority_t_osPriorityNormal => "Normal",
                rtos::osPriority_t_osPriorityNormal1 => "Normal1",
                rtos::osPriority_t_osPriorityNormal2 => "Normal2",
                rtos::osPriority_t_osPriorityNormal3 => "Normal3",
                rtos::osPriority_t_osPriorityNormal4 => "Normal4",
                rtos::osPriority_t_osPriorityNormal5 => "Normal5",
                rtos::osPriority_t_osPriorityNormal6 => "Normal6",
                rtos::osPriority_t_osPriorityNormal7 => "Normal7",
                rtos::osPriority_t_osPriorityAboveNormal => "AboveNormal",
                rtos::osPriority_t_osPriorityAboveNormal1 => "AboveNormal1",
                rtos::osPriority_t_osPriorityAboveNormal2 => "AboveNormal2",
                rtos::osPriority_t_osPriorityAboveNormal3 => "AboveNormal3",
                rtos::osPriority_t_osPriorityAboveNormal4 => "AboveNormal4",
                rtos::osPriority_t_osPriorityAboveNormal5 => "AboveNormal5",
                rtos::osPriority_t_osPriorityAboveNormal6 => "AboveNormal6",
                rtos::osPriority_t_osPriorityAboveNormal7 => "AboveNormal7",
                rtos::osPriority_t_osPriorityHigh => "High",
                rtos::osPriority_t_osPriorityHigh1 => "High1",
                rtos::osPriority_t_osPriorityHigh2 => "High2",
                rtos::osPriority_t_osPriorityHigh3 => "High3",
                rtos::osPriority_t_osPriorityHigh4 => "High4",
                rtos::osPriority_t_osPriorityHigh5 => "High5",
                rtos::osPriority_t_osPriorityHigh6 => "High6",
                rtos::osPriority_t_osPriorityHigh7 => "High7",
                rtos::osPriority_t_osPriorityRealtime => "Realtime",
                rtos::osPriority_t_osPriorityRealtime1 => "Realtime1",
                rtos::osPriority_t_osPriorityRealtime2 => "Realtime2",
                rtos::osPriority_t_osPriorityRealtime3 => "Realtime3",
                rtos::osPriority_t_osPriorityRealtime4 => "Realtime4",
                rtos::osPriority_t_osPriorityRealtime5 => "Realtime5",
                rtos::osPriority_t_osPriorityRealtime6 => "Realtime6",
                rtos::osPriority_t_osPriorityRealtime7 => "Realtime7",
                rtos::osPriority_t_osPriorityISR => "ISR",
                _ => "Error",
            },

            state: match thread_info.state as i32 {
                rtos::osThreadState_t_osThreadInactive => "Inactive",
                rtos::osThreadState_t_osThreadReady => "Ready",
                rtos::osThreadState_t_osThreadRunning => "Running",
                rtos::osThreadState_t_osThreadBlocked => "Blocked",
                rtos::osThreadState_t_osThreadTerminated => "Terminated",
                rtos::osThreadState_t_osThreadWaitingDelay => "Waiting Delay",
                rtos::osThreadState_t_osThreadWaitingJoin => "Waiting Join",
                rtos::osThreadState_t_osThreadWaitingThreadFlags => "Waiting Thread Flags",
                rtos::osThreadState_t_osThreadWaitingEventFlags => "Waiting Event Flags",
                rtos::osThreadState_t_osThreadWaitingMutex => "Waiting Mutex",
                rtos::osThreadState_t_osThreadWaitingSemaphore => "Waiting Semaphore",
                rtos::osThreadState_t_osThreadWaitingMemoryPool => "Waiting Memory Pool",
                rtos::osThreadState_t_osThreadWaitingMessageGet => "Waiting Message Get",
                rtos::osThreadState_t_osThreadWaitingMessagePut => "Waiting Message Put",
                _ => "Error",
            },
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
