#![feature(const_fn)]
#![feature(const_let)]
#![cfg_attr(target_env="kext", no_std)]
#![cfg_attr(target_env="kext", no_main)]
extern crate kextauth;
use kextauth::kernel;
use kextauth::c_types;

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

#[no_mangle]
pub static mut kmod_info: kernel::kmod_info_t = kernel::kmod_info_t{
    next: 0,
    info_version: kernel::KMOD_INFO_VERSION,
    id: 0xffffffff,
    name: *b"af.sd.kext\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: *b"1.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    reference_count: -1,
    reference_list: 0,
    address: 0,
    size: 0,
    hdr_size: 0,
    start: kext_start,
    stop: kext_stop,
};

