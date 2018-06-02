#![feature(lang_items, core_intrinsics)]
#![feature(allocator_api, global_allocator)]
#![no_std]
use core::intrinsics;
pub mod c_types;
pub mod kernel;
pub mod alloc;

#[global_allocator]
static ALLOCATOR: alloc::KernelAllocator = alloc::KernelAllocator;

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32,
                               _column: u32) -> ! {
    unsafe { intrinsics::abort() }
}
