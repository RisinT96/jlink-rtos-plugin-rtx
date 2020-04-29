//! Bindings manually generated from RTXv5 headers.
//! Also helper types/functions/traits.

/// Kernel state
pub type OsKernelState = i32;
#[doc = "< Inactive."]
pub const osKernelState_t_osKernelInactive: OsKernelState = 0;
#[doc = "< Ready."]
pub const osKernelState_t_osKernelReady: OsKernelState = 1;
#[doc = "< Running."]
pub const osKernelState_t_osKernelRunning: OsKernelState = 2;
#[doc = "< Locked."]
pub const osKernelState_t_osKernelLocked: OsKernelState = 3;
#[doc = "< Suspended."]
pub const osKernelState_t_osKernelSuspended: OsKernelState = 4;
#[doc = "< Error."]
pub const osKernelState_t_osKernelError: OsKernelState = -1;
#[doc = "< Prevents enum down-size compiler optimization."]
pub const osKernelState_t_osKernelReserved: OsKernelState = 2147483647;

#[doc = " Thread state."]
pub type OsThreadState = i32;
#[doc = "< Inactive."]
pub const osThreadState_t_osThreadInactive: OsThreadState = 0;
#[doc = "< Ready."]
pub const osThreadState_t_osThreadReady: OsThreadState = 1;
#[doc = "< Running."]
pub const osThreadState_t_osThreadRunning: OsThreadState = 2;
#[doc = "< Blocked."]
pub const osThreadState_t_osThreadBlocked: OsThreadState = 3;
#[doc = "< Terminated."]
pub const osThreadState_t_osThreadTerminated: OsThreadState = 4;
#[doc = "< Error."]
pub const osThreadState_t_osThreadError: OsThreadState = -1;
#[doc = "< Prevents enum down-size compiler optimization."]
pub const osThreadState_t_osThreadReserved: OsThreadState = 2147483647;
pub const osThreadState_t_osThreadWaitingDelay: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x10;
pub const osThreadState_t_osThreadWaitingJoin: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x20;
pub const osThreadState_t_osThreadWaitingThreadFlags: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x30;
pub const osThreadState_t_osThreadWaitingEventFlags: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x40;
pub const osThreadState_t_osThreadWaitingMutex: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x50;
pub const osThreadState_t_osThreadWaitingSemaphore: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x60;
pub const osThreadState_t_osThreadWaitingMemoryPool: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x70;
pub const osThreadState_t_osThreadWaitingMessageGet: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x80;
pub const osThreadState_t_osThreadWaitingMessagePut: OsThreadState =
    osThreadState_t_osThreadBlocked | 0x90;

#[doc = " Priority values."]
pub type OsPriority = i32;
#[doc = "< No priority (not initialized)."]
pub const osPriority_t_osPriorityNone: OsPriority = 0;
#[doc = "< Reserved for Idle thread."]
pub const osPriority_t_osPriorityIdle: OsPriority = 1;
#[doc = "< Priority: low"]
pub const osPriority_t_osPriorityLow: OsPriority = 8;
#[doc = "< Priority: low + 1"]
pub const osPriority_t_osPriorityLow1: OsPriority = 9;
#[doc = "< Priority: low + 2"]
pub const osPriority_t_osPriorityLow2: OsPriority = 10;
#[doc = "< Priority: low + 3"]
pub const osPriority_t_osPriorityLow3: OsPriority = 11;
#[doc = "< Priority: low + 4"]
pub const osPriority_t_osPriorityLow4: OsPriority = 12;
#[doc = "< Priority: low + 5"]
pub const osPriority_t_osPriorityLow5: OsPriority = 13;
#[doc = "< Priority: low + 6"]
pub const osPriority_t_osPriorityLow6: OsPriority = 14;
#[doc = "< Priority: low + 7"]
pub const osPriority_t_osPriorityLow7: OsPriority = 15;
#[doc = "< Priority: below normal"]
pub const osPriority_t_osPriorityBelowNormal: OsPriority = 16;
#[doc = "< Priority: below normal + 1"]
pub const osPriority_t_osPriorityBelowNormal1: OsPriority = 17;
#[doc = "< Priority: below normal + 2"]
pub const osPriority_t_osPriorityBelowNormal2: OsPriority = 18;
#[doc = "< Priority: below normal + 3"]
pub const osPriority_t_osPriorityBelowNormal3: OsPriority = 19;
#[doc = "< Priority: below normal + 4"]
pub const osPriority_t_osPriorityBelowNormal4: OsPriority = 20;
#[doc = "< Priority: below normal + 5"]
pub const osPriority_t_osPriorityBelowNormal5: OsPriority = 21;
#[doc = "< Priority: below normal + 6"]
pub const osPriority_t_osPriorityBelowNormal6: OsPriority = 22;
#[doc = "< Priority: below normal + 7"]
pub const osPriority_t_osPriorityBelowNormal7: OsPriority = 23;
#[doc = "< Priority: normal"]
pub const osPriority_t_osPriorityNormal: OsPriority = 24;
#[doc = "< Priority: normal + 1"]
pub const osPriority_t_osPriorityNormal1: OsPriority = 25;
#[doc = "< Priority: normal + 2"]
pub const osPriority_t_osPriorityNormal2: OsPriority = 26;
#[doc = "< Priority: normal + 3"]
pub const osPriority_t_osPriorityNormal3: OsPriority = 27;
#[doc = "< Priority: normal + 4"]
pub const osPriority_t_osPriorityNormal4: OsPriority = 28;
#[doc = "< Priority: normal + 5"]
pub const osPriority_t_osPriorityNormal5: OsPriority = 29;
#[doc = "< Priority: normal + 6"]
pub const osPriority_t_osPriorityNormal6: OsPriority = 30;
#[doc = "< Priority: normal + 7"]
pub const osPriority_t_osPriorityNormal7: OsPriority = 31;
#[doc = "< Priority: above normal"]
pub const osPriority_t_osPriorityAboveNormal: OsPriority = 32;
#[doc = "< Priority: above normal + 1"]
pub const osPriority_t_osPriorityAboveNormal1: OsPriority = 33;
#[doc = "< Priority: above normal + 2"]
pub const osPriority_t_osPriorityAboveNormal2: OsPriority = 34;
#[doc = "< Priority: above normal + 3"]
pub const osPriority_t_osPriorityAboveNormal3: OsPriority = 35;
#[doc = "< Priority: above normal + 4"]
pub const osPriority_t_osPriorityAboveNormal4: OsPriority = 36;
#[doc = "< Priority: above normal + 5"]
pub const osPriority_t_osPriorityAboveNormal5: OsPriority = 37;
#[doc = "< Priority: above normal + 6"]
pub const osPriority_t_osPriorityAboveNormal6: OsPriority = 38;
#[doc = "< Priority: above normal + 7"]
pub const osPriority_t_osPriorityAboveNormal7: OsPriority = 39;
#[doc = "< Priority: high"]
pub const osPriority_t_osPriorityHigh: OsPriority = 40;
#[doc = "< Priority: high + 1"]
pub const osPriority_t_osPriorityHigh1: OsPriority = 41;
#[doc = "< Priority: high + 2"]
pub const osPriority_t_osPriorityHigh2: OsPriority = 42;
#[doc = "< Priority: high + 3"]
pub const osPriority_t_osPriorityHigh3: OsPriority = 43;
#[doc = "< Priority: high + 4"]
pub const osPriority_t_osPriorityHigh4: OsPriority = 44;
#[doc = "< Priority: high + 5"]
pub const osPriority_t_osPriorityHigh5: OsPriority = 45;
#[doc = "< Priority: high + 6"]
pub const osPriority_t_osPriorityHigh6: OsPriority = 46;
#[doc = "< Priority: high + 7"]
pub const osPriority_t_osPriorityHigh7: OsPriority = 47;
#[doc = "< Priority: realtime"]
pub const osPriority_t_osPriorityRealtime: OsPriority = 48;
#[doc = "< Priority: realtime + 1"]
pub const osPriority_t_osPriorityRealtime1: OsPriority = 49;
#[doc = "< Priority: realtime + 2"]
pub const osPriority_t_osPriorityRealtime2: OsPriority = 50;
#[doc = "< Priority: realtime + 3"]
pub const osPriority_t_osPriorityRealtime3: OsPriority = 51;
#[doc = "< Priority: realtime + 4"]
pub const osPriority_t_osPriorityRealtime4: OsPriority = 52;
#[doc = "< Priority: realtime + 5"]
pub const osPriority_t_osPriorityRealtime5: OsPriority = 53;
#[doc = "< Priority: realtime + 6"]
pub const osPriority_t_osPriorityRealtime6: OsPriority = 54;
#[doc = "< Priority: realtime + 7"]
pub const osPriority_t_osPriorityRealtime7: OsPriority = 55;
#[doc = "< Reserved for ISR deferred thread."]
pub const osPriority_t_osPriorityISR: OsPriority = 56;
#[doc = "< System cannot determine priority or illegal priority."]
pub const osPriority_t_osPriorityError: OsPriority = -1;
#[doc = "< Prevents enum down-size compiler optimization."]
pub const osPriority_t_osPriorityReserved: OsPriority = 2147483647;

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
pub struct osRtxThread {
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
