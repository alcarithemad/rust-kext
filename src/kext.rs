#![cfg_attr(target_env="kext", no_std)]
#![cfg_attr(target_env="kext", no_main)]
#![feature(const_fn, const_slice_len, const_transmute, untagged_unions)]
#![feature(type_ascription)]
#![feature(proc_macro_hygiene)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
extern crate kext;
extern crate kext_macros;

use kext::kernel;
use kext::c_types;
use kext::simple_kmod_info;

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

// #[no_mangle]
// // #[return_as_is]
// pub static kmod_info: kernel::kmod_info_t = kernel::kmod_info_t{
//     next: 0,
//     info_version: kernel::KMOD_INFO_VERSION,
//     id: 0xffffffff,
//     name: bytes_to_u8_64!(b"af.sd.kext"),
//     version: bytes_to_u8_64!(b"1.3"),
//     reference_count: -1,
//     reference_list: 0,
//     address: 0,
//     size: 0,
//     hdr_size: 0,
//     start: kext_start,
//     stop: kext_stop,
// };

simple_kmod_info!(
    name: b"af.sd.kext",
    version: b"1.3",
    start: kext_start,
    stop: kext_stop
);