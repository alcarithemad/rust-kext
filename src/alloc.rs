use core::alloc;
use c_types;
use kernel;

#[cfg(target_env="kext")]
pub struct KernelAllocator;

#[cfg(target_env="kext")]
unsafe impl alloc::GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        kernel::raw::_MALLOC(
            layout.size(),
            0, // not sure what type is for
            0, // not sure what flags is for
        ) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: alloc::Layout) {
        kernel::raw::_FREE(ptr as *mut c_types::c_void, 0)
    }
}

#[cfg(target_env="kext")]
#[lang = "oom"]
extern "C" fn oom(_err: alloc::Layout) -> ! {
    panic!("Out of memory!");
}
