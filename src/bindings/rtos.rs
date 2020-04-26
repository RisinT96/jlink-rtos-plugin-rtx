//! Bindings auto generated from RTXv5 headers.
//! Also helper types/functions/traits.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
include!(concat!(env!("OUT_DIR"), "/rtx_bindings.rs"));

pub const osThreadState_t_osThreadWaitingDelay: i32 = (osThreadState_t_osThreadBlocked | 0x10i32);
pub const osThreadState_t_osThreadWaitingJoin: i32 = (osThreadState_t_osThreadBlocked | 0x20i32);
pub const osThreadState_t_osThreadWaitingThreadFlags: i32 = (osThreadState_t_osThreadBlocked | 0x30i32);
pub const osThreadState_t_osThreadWaitingEventFlags: i32 = (osThreadState_t_osThreadBlocked | 0x40i32);
pub const osThreadState_t_osThreadWaitingMutex: i32 = (osThreadState_t_osThreadBlocked | 0x50i32);
pub const osThreadState_t_osThreadWaitingSemaphore: i32 = (osThreadState_t_osThreadBlocked | 0x60i32);
pub const osThreadState_t_osThreadWaitingMemoryPool: i32 = (osThreadState_t_osThreadBlocked | 0x70i32);
pub const osThreadState_t_osThreadWaitingMessageGet: i32 = (osThreadState_t_osThreadBlocked | 0x80i32);
pub const osThreadState_t_osThreadWaitingMessagePut: i32 = (osThreadState_t_osThreadBlocked | 0x90i32);
