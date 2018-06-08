#![cfg_attr(target_env="kext", no_std)]
#![cfg_attr(target_env="kext", no_main)]
#![feature(const_fn, const_slice_len, untagged_unions)]
#[macro_use]
extern crate kext;
use kext::kernel;
use kext::c_types;

#[cfg(not(target_env="kext"))]
fn main() {
    println!("{}", std::mem::size_of::<kernel::kmod_info_t>());
}

#[no_mangle]
pub extern "C" fn kext_start(_ki: kernel::kmod_info_t, _d: c_types::c_void) -> kernel::kern_return_t {
    unsafe {
        kernel::raw::IOLog("test".as_ptr());
    }
    kernel::KERN_SUCCESS
}

#[no_mangle]
pub extern "C" fn kext_stop(_ki: kernel::kmod_info_t, _d: c_types::c_void) -> kernel::kern_return_t {
    kernel::KERN_SUCCESS
}

simple_kmod_info!(
    name: b"af.sd.kext",
    version: b"1.3",
    start: kext_start,
    stop: kext_stop);