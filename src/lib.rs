#![feature(lang_items, core_intrinsics)]
#![feature(allocator_api, global_allocator)]
#![feature(repr_packed)]
#![feature(const_fn, const_slice_len, untagged_unions)]
#![cfg_attr(target_env="kext", no_std)]
use core::intrinsics;
pub mod c_types;
pub mod kernel;
pub mod alloc;

#[cfg(target_env="kext")]
#[global_allocator]
static ALLOCATOR: alloc::KernelAllocator = alloc::KernelAllocator;

#[cfg(target_env="kext")]
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[cfg(target_env="kext")]
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[cfg(target_env="kext")]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32,
                               column: u32) -> ! {
    unsafe { 
        kernel::raw::IOLog("%s:%d %d".as_ptr(), file.as_ptr(), line, column);
        intrinsics::abort()
    }
}