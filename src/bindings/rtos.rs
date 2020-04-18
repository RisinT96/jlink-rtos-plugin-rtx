//! Bindings auto generated from RTXv5 headers.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
include!(concat!(env!("OUT_DIR"), "/rtx_bindings.rs"));

use std::convert::TryFrom;

use crate::gdb::api;

#[derive(Debug)]
struct Thread<'a> {
    name: String,
    id: u8,
    stack_frame: u8,
    stack_base: u32,
    stack_size: u32,
    stack_pointer: u32,
    priority: &'a str,
    state: &'a str,
}

impl TryFrom<osRtxThread_t> for Thread<'_> {
    type Error = i32;

    fn try_from(thread_info: osRtxThread_t) -> Result<Self, Self::Error> {
        if let Ok(name) = api::read_string(thread_info.name as u32, 256) {
            Ok(Thread {
                name: name,
                id: thread_info.id,
                stack_frame: thread_info.stack_frame,
                stack_base: thread_info.stack_mem as u32,
                stack_size: thread_info.stack_size,
                stack_pointer: thread_info.sp,

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
                    _ => "Error",
                },
            })
        } else {
            return Err(api::GDB_ERR);
        }
    }
}
