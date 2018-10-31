#![feature(lang_items)]
#![feature(core_intrinsics)] // required for core::intrinsics::abort() in panic handler
#![feature(repr_packed)] // required for some bindgen'd types
#![cfg_attr(target_env="kext", no_std)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
extern crate kext_macros;
use core::intrinsics;
use core::panic::PanicInfo;
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
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        if let Some(loc) = info.location() {
            kernel::raw::IOLog("%s:%d %d".as_ptr(), loc.file(), loc.line(), loc.column());
        }
        intrinsics::abort()
    }
}