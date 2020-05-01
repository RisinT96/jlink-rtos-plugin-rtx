//! Bindings manually generated from RTXv5 headers.
//! Also helper types/functions/traits.

use num_derive::FromPrimitive;

/// Kernel state
#[derive(FromPrimitive)]
pub enum OsKernelState {
    Inactive = 0,
    Ready = 1,
    Running = 2,
    Locked = 3,
    Suspended = 4,
    Error = -1,
    Reserved = 2147483647,
}

#[derive(FromPrimitive)]
pub enum OsThreadState {
    Inactive = 0,
    Ready = 1,
    Running = 2,
    Blocked = 3,
    Terminated = 4,
    Error = -1,
    Reserved = 2147483647,
    WaitingDelay = 3 | 0x10,
    WaitingJoin = 3 | 0x20,
    WaitingThreadFlags = 3 | 0x30,
    WaitingEventFlags = 3 | 0x40,
    WaitingMutex = 3 | 0x50,
    WaitingSemaphore = 3 | 0x60,
    WaitingMemoryPool = 3 | 0x70,
    WaitingMessageGet = 3 | 0x80,
    WaitingMessagePut = 3 | 0x90,
}

impl std::fmt::Display for OsThreadState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
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
        };

        f.write_str(name)
    }
}

#[derive(FromPrimitive)]
pub enum OsThreadPriority {
    None = 0,
    Idle = 1,
    Low = 8,
    Low1 = 9,
    Low2 = 10,
    Low3 = 11,
    Low4 = 12,
    Low5 = 13,
    Low6 = 14,
    Low7 = 15,
    BelowNormal = 16,
    BelowNormal1 = 17,
    BelowNormal2 = 18,
    BelowNormal3 = 19,
    BelowNormal4 = 20,
    BelowNormal5 = 21,
    BelowNormal6 = 22,
    BelowNormal7 = 23,
    Normal = 24,
    Normal1 = 25,
    Normal2 = 26,
    Normal3 = 27,
    Normal4 = 28,
    Normal5 = 29,
    Normal6 = 30,
    Normal7 = 31,
    AboveNormal = 32,
    AboveNormal1 = 33,
    AboveNormal2 = 34,
    AboveNormal3 = 35,
    AboveNormal4 = 36,
    AboveNormal5 = 37,
    AboveNormal6 = 38,
    AboveNormal7 = 39,
    High = 40,
    High1 = 41,
    High2 = 42,
    High3 = 43,
    High4 = 44,
    High5 = 45,
    High6 = 46,
    High7 = 47,
    Realtime = 48,
    Realtime1 = 49,
    Realtime2 = 50,
    Realtime3 = 51,
    Realtime4 = 52,
    Realtime5 = 53,
    Realtime6 = 54,
    Realtime7 = 55,
    ISR = 56,
    Error = -1,
    Reserved = 2147483647,
}

impl std::fmt::Display for OsThreadPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            OsThreadPriority::None => "None",
            OsThreadPriority::Idle => "Idle",
            OsThreadPriority::Low => "Low",
            OsThreadPriority::Low1 => "Low + 1",
            OsThreadPriority::Low2 => "Low + 2",
            OsThreadPriority::Low3 => "Low + 3",
            OsThreadPriority::Low4 => "Low + 4",
            OsThreadPriority::Low5 => "Low + 5",
            OsThreadPriority::Low6 => "Low + 6",
            OsThreadPriority::Low7 => "Low + 7",
            OsThreadPriority::BelowNormal => "BelowNormal",
            OsThreadPriority::BelowNormal1 => "BelowNormal + 1",
            OsThreadPriority::BelowNormal2 => "BelowNormal + 2",
            OsThreadPriority::BelowNormal3 => "BelowNormal + 3",
            OsThreadPriority::BelowNormal4 => "BelowNormal + 4",
            OsThreadPriority::BelowNormal5 => "BelowNormal + 5",
            OsThreadPriority::BelowNormal6 => "BelowNormal + 6",
            OsThreadPriority::BelowNormal7 => "BelowNormal + 7",
            OsThreadPriority::Normal => "Normal",
            OsThreadPriority::Normal1 => "Normal + 1",
            OsThreadPriority::Normal2 => "Normal + 2",
            OsThreadPriority::Normal3 => "Normal + 3",
            OsThreadPriority::Normal4 => "Normal + 4",
            OsThreadPriority::Normal5 => "Normal + 5",
            OsThreadPriority::Normal6 => "Normal + 6",
            OsThreadPriority::Normal7 => "Normal + 7",
            OsThreadPriority::AboveNormal => "AboveNormal",
            OsThreadPriority::AboveNormal1 => "AboveNormal + 1",
            OsThreadPriority::AboveNormal2 => "AboveNormal + 2",
            OsThreadPriority::AboveNormal3 => "AboveNormal + 3",
            OsThreadPriority::AboveNormal4 => "AboveNormal + 4",
            OsThreadPriority::AboveNormal5 => "AboveNormal + 5",
            OsThreadPriority::AboveNormal6 => "AboveNormal + 6",
            OsThreadPriority::AboveNormal7 => "AboveNormal + 7",
            OsThreadPriority::High => "High",
            OsThreadPriority::High1 => "High + 1",
            OsThreadPriority::High2 => "High + 2",
            OsThreadPriority::High3 => "High + 3",
            OsThreadPriority::High4 => "High + 4",
            OsThreadPriority::High5 => "High + 5",
            OsThreadPriority::High6 => "High + 6",
            OsThreadPriority::High7 => "High + 7",
            OsThreadPriority::Realtime => "Realtime",
            OsThreadPriority::Realtime1 => "Realtime + 1",
            OsThreadPriority::Realtime2 => "Realtime + 2",
            OsThreadPriority::Realtime3 => "Realtime + 3",
            OsThreadPriority::Realtime4 => "Realtime + 4",
            OsThreadPriority::Realtime5 => "Realtime + 5",
            OsThreadPriority::Realtime6 => "Realtime + 6",
            OsThreadPriority::Realtime7 => "Realtime + 7",
            OsThreadPriority::ISR => "ISR",
            _ => "Error",
        };

        f.write_str(name)
    }
}

/// OS Runtime Information structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osRtxInfo {
    /// Os Name ( *const c_char)
    pub os_id: u32,
    /// OS Version
    pub version: u32,
    pub kernel: KernelInfo,
    /// Tick Timer IRQ Number
    pub tick_irqn: i32,
    pub thread: ThreadInfo,
    pub timer: TimerInfo,
    pub isr_queue: IsrQueueInfo,
    pub post_process: PostProcessInfo,
    pub mem: MemInfo,
    pub mpi: MpiInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernelInfo {
    pub state: u8,
    pub blocked: u8,
    pub pend_sv: u8,
    pub reserved: u8,
    pub tick: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ThreadInfo {
    pub run: ThreadRunInfo,
    pub ready: osRtxObject,
    pub idle: u32,
    pub delay_list: u32,
    pub wait_list: u32,
    pub terminate_list: u32,
    pub robin: ThreadRoundRobinInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ThreadRunInfo {
    pub curr: u32,
    pub next: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osRtxObject {
    pub id: u8,
    pub state: u8,
    pub flags: u8,
    pub reserved: u8,
    pub name: u32,
    pub thread_list: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ThreadRoundRobinInfo {
    pub thread: u32,
    pub tick: u32,
    pub timeout: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimerInfo {
    pub list: u32,
    pub thread: u32,
    pub mq: u32,
    pub tick: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IsrQueueInfo {
    pub max: u16,
    pub cnt: u16,
    pub in_: u16,
    pub out: u16,
    pub data: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PostProcessInfo {
    pub thread: u32,
    pub event_flags: u32,
    pub semaphore: u32,
    pub memory_pool: u32,
    pub message: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemInfo {
    pub stack: u32,
    pub mp_data: u32,
    pub mq_data: u32,
    pub common: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MpiInfo {
    pub stack: u32,
    pub thread: u32,
    pub timer: u32,
    pub event_flags: u32,
    pub mutex: u32,
    pub semaphore: u32,
    pub memory_pool: u32,
    pub message_queue: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OsRtxThread {
    /// Object Identifier
    pub id: u8,
    /// Object State
    pub state: u8,
    pub flags: u8,
    pub attr: u8,
    /// Object name (c-string pointer)
    pub name: u32,
    /// Pointer to next thread in object list
    pub thread_next: u32,
    /// Pointer to previous thread in object list
    pub thread_prev: u32,
    /// Pointer to next thread in delay list
    pub delay_next: u32,
    /// Pointer to previous thread in delay list
    pub delay_prev: u32,
    /// thread waiting to join
    pub thread_join: u32,
    /// Delay time
    pub delay: u32,
    /// Thread priority
    pub priority: i8,
    /// Base priority
    pub priority_base: i8,
    /// Stack Frame (EXC_RETURN[7..0])
    pub stack_frame: u8,
    pub flag_options: u8,
    pub wait_flags: u32,
    pub thread_flags: u32,
    pub mutex_list: u32,
    pub stack_mem: u32,
    pub stack_size: u32,
    /// Current Stack Pointer
    pub sp: u32,
    pub thread_addr: u32,
    pub tz_memory: u32,
    pub tz_module: u32,
}
