//! Wraps the RTXv5 kernel with an easy to use object

use crate::bindings::rtos;
use crate::bindings::rtos::osRtxInfo_t;
use crate::host::api;

use crate::device::thread::{Thread, ThreadDelayList, ThreadReadyList};

pub struct RtxInfo {
    pub threads: Vec<Thread>,
}

/* ------------------------------------- Implementations ------------------------------------------------------------ */

impl RtxInfo {
    pub fn new(address: u32) -> Result<RtxInfo, i32> {
        trace!("Loading RTOS Info from {:#X}", address);
        let rtx_info: osRtxInfo_t = api::read_mem(address)?;

        let version = api::convert_u32(rtx_info.version)?;
        let os_id = api::convert_u32(rtx_info.os_id as u32)?;

        // Load RTX Name, if possible
        if os_id as *const i8 != std::ptr::null() {
            let name = api::read_string(os_id, 256)?;
            debug!("Found RTX '{}', Version {}", name, version);
        } else {
            debug!("Found RTX Version {}", version);
        }

        let mut threads: Vec<Thread> = Vec::new();

        // If we ran before the kernel was even initialized, exit, there are zero threads.
        match rtx_info.kernel.state as i32 {
            rtos::osKernelState_t_osKernelInactive | rtos::osKernelState_t_osKernelError => {
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
}
