use core::alloc;
use c_types;
use kernel;

#[cfg(target_env="kext")]
pub struct KernelAllocator;

#[cfg(target_env="kext")]
unsafe impl alloc::GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut alloc::Opaque {
        kernel::raw::_MALLOC(
            layout.size(),
            0, // not sure what type is for
            0, // not sure what flags is for
        ) as *mut alloc::Opaque
    }
    unsafe fn dealloc(&self, ptr: *mut alloc::Opaque, _layout: alloc::Layout) {
        kernel::raw::_FREE(ptr as *mut c_types::c_void, 0)
    }
}

#[cfg(target_env="kext")]
#[lang = "oom"]
extern "C" fn oom(_err: alloc::AllocErr) -> ! {
    panic!("Out of memory!");
}
