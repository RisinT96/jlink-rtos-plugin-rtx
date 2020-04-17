use std::alloc::{GlobalAlloc, Layout};
use std::ptr::null_mut;

use crate::api;

struct ExternAllocator;

unsafe impl GlobalAlloc for ExternAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = api::alloc(layout.size());

        if (ptr as usize) % layout.align() != 0 {
            // Weird alignment, don't know how to properly align this shit.
            // Even if I did, wouldn't know how to dealloc the ptr later.
            api::free(ptr);
            return null_mut();
        }

        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        api::free(ptr);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ptr = api::realloc(ptr, new_size);
        if (ptr as usize) % layout.align() != 0 {
            // Weird alignment, don't know how to properly align this shit.
            // Even if I did, wouldn't know how to dealloc the ptr later.
            api::free(ptr);
            return null_mut();
        }

        ptr
    }
}


#[global_allocator]
static ALLOCATOR: ExternAllocator = ExternAllocator;
