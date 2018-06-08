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

pub const fn transmute<S>(x: S) -> [u8; 64] {
    #[allow(unions_with_drop_fields)]
    union U<T> {
        a: T,
        b: [u8; 64]
    }
    unsafe { U { a: x }.b }
}

#[macro_export]
macro_rules! simple_kmod_info {
    (name: $name:expr,
    version: $version:expr,
    start: $start:ident,
    stop: $stop:ident) => {
        #[no_mangle]
        pub static kmod_info: kernel::kmod_info_t = kernel::kmod_info_t{
            next: 0,
            info_version: kernel::KMOD_INFO_VERSION,
            id: 0xffffffff,
            name: $crate::kernel::transmute::<[u8; $name.len()]>(*$name),
            version: $crate::kernel::transmute::<[u8; $version.len()]>(*$version),
            reference_count: -1,
            reference_list: 0,
            address: 0,
            size: 0,
            hdr_size: 0,
            start: $start,
            stop: $stop,
        };
    };
}
