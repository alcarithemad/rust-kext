#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use c_types;

pub mod raw {
    use c_types;
    use super::kmod_info_t;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use self::raw::kern_return_t;
pub use self::raw::KERN_SUCCESS;
pub use self::raw::KMOD_INFO_VERSION;

type kext_init_fn = extern "C" fn(ki: kmod_info_t, d: c_types::c_void) -> kern_return_t;

#[repr(C, packed(4))]
pub struct kmod_info_t {
    pub next: u64,
    pub info_version: i32,
    pub id: u32,
    pub name: [u8; 64usize],
    pub version: [u8; 64usize],
    pub reference_count: i32,
    pub reference_list: u64,
    pub address: u64,
    pub size: u64,
    pub hdr_size: u64,
    pub start: kext_init_fn,
    pub stop: kext_init_fn,
}
