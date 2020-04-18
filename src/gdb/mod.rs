pub mod api;
pub mod log;

mod allocator;

/* ------------------------------------- Global Config--------------------------------------------------------------- */

#[global_allocator]
static ALLOCATOR: allocator::GdbAllocator = allocator::GdbAllocator;
