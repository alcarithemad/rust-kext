#![no_std]
#![no_main]
extern crate kextauth;
use kextauth::kernel;
use kextauth::c_types;

#[no_mangle]
pub fn kext_start(ki: kernel::kmod_info_t, d: c_types::c_void) -> kernel::kern_return_t {
    unsafe {
        kernel::printf("test".as_bytes().as_ptr());
    }
    kernel::KERN_SUCCESS
}

#[no_mangle]
pub fn kext_stop(ki: kernel::kmod_info_t, d: c_types::c_void) -> kernel::kern_return_t {
    kernel::KERN_SUCCESS
}
