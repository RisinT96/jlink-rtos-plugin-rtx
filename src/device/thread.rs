use num_traits::FromPrimitive;

use crate::bindings::rtos::{OsPriority, OsRtxThread, OsThreadState};

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

        let thread_info: OsRtxThread = api::read_mem(address)?;

        let name_addr = api::convert_u32(thread_info.name)?;

        let mut name = String::new();
        if name_addr != 0 {
            name = api::read_string(name_addr, 256)?;
        }

        let priority: OsPriority = match FromPrimitive::from_i8(thread_info.priority) {
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

            priority: match priority {
                OsPriority::None => "None",
                OsPriority::Idle => "Idle",
                OsPriority::Low => "Low",
                OsPriority::Low1 => "Low + 1",
                OsPriority::Low2 => "Low + 2",
                OsPriority::Low3 => "Low + 3",
                OsPriority::Low4 => "Low + 4",
                OsPriority::Low5 => "Low + 5",
                OsPriority::Low6 => "Low + 6",
                OsPriority::Low7 => "Low + 7",
                OsPriority::BelowNormal => "BelowNormal",
                OsPriority::BelowNormal1 => "BelowNormal + 1",
                OsPriority::BelowNormal2 => "BelowNormal + 2",
                OsPriority::BelowNormal3 => "BelowNormal + 3",
                OsPriority::BelowNormal4 => "BelowNormal + 4",
                OsPriority::BelowNormal5 => "BelowNormal + 5",
                OsPriority::BelowNormal6 => "BelowNormal + 6",
                OsPriority::BelowNormal7 => "BelowNormal + 7",
                OsPriority::Normal => "Normal",
                OsPriority::Normal1 => "Normal + 1",
                OsPriority::Normal2 => "Normal + 2",
                OsPriority::Normal3 => "Normal + 3",
                OsPriority::Normal4 => "Normal + 4",
                OsPriority::Normal5 => "Normal + 5",
                OsPriority::Normal6 => "Normal + 6",
                OsPriority::Normal7 => "Normal + 7",
                OsPriority::AboveNormal => "AboveNormal",
                OsPriority::AboveNormal1 => "AboveNormal + 1",
                OsPriority::AboveNormal2 => "AboveNormal + 2",
                OsPriority::AboveNormal3 => "AboveNormal + 3",
                OsPriority::AboveNormal4 => "AboveNormal + 4",
                OsPriority::AboveNormal5 => "AboveNormal + 5",
                OsPriority::AboveNormal6 => "AboveNormal + 6",
                OsPriority::AboveNormal7 => "AboveNormal + 7",
                OsPriority::High => "High",
                OsPriority::High1 => "High + 1",
                OsPriority::High2 => "High + 2",
                OsPriority::High3 => "High + 3",
                OsPriority::High4 => "High + 4",
                OsPriority::High5 => "High + 5",
                OsPriority::High6 => "High + 6",
                OsPriority::High7 => "High + 7",
                OsPriority::Realtime => "Realtime",
                OsPriority::Realtime1 => "Realtime + 1",
                OsPriority::Realtime2 => "Realtime + 2",
                OsPriority::Realtime3 => "Realtime + 3",
                OsPriority::Realtime4 => "Realtime + 4",
                OsPriority::Realtime5 => "Realtime + 5",
                OsPriority::Realtime6 => "Realtime + 6",
                OsPriority::Realtime7 => "Realtime + 7",
                OsPriority::ISR => "ISR",
                _ => "Error",
            },

            state: match state {
                OsThreadState::Inactive => "Inactive",
                OsThreadState::Ready => "Ready",
                OsThreadState::Running => "Running",
                OsThreadState::Blocked => "Blocked",
                OsThreadState::Terminated => "Terminated",
                OsThreadState::WaitingDelay => "Waiting Delay",
                OsThreadState::WaitingJoin => "Waiting Join",
                OsThreadState::WaitingThreadFlags => "Waiting Thread Flags",
                OsThreadState::WaitingEventFlags => "Waiting Event Flags",
                OsThreadState::WaitingMutex => "Waiting Mutex",
                OsThreadState::WaitingSemaphore => "Waiting Semaphore",
                OsThreadState::WaitingMemoryPool => "Waiting Memory Pool",
                OsThreadState::WaitingMessageGet => "Waiting Message Get",
                OsThreadState::WaitingMessagePut => "Waiting Message Put",
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
