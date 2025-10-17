use libc::c_void;
use std::alloc::{GlobalAlloc, Layout};
use std::ptr;

pub type ChainId = u16;
pub type TypeId = u16;
pub type DiceThreadId = u64;
pub mod events;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DiceResult {
    Ok = 0,
    StopChain = 1,
    DropEvent = 2,
    HandlerOff = 3,
    Invalid = -1,
    Error = -2,
}

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug)]
pub struct Metadata {
    pub drop_: bool,
}

pub type PsCallbackF = Option<
    unsafe extern "C" fn(
        chain: Chain,
        ty: TypeId,
        event: *const c_void,
        md: *mut Metadata,
    ) -> DiceResult,
>;

unsafe extern "C" {
    pub fn ps_subscribe(chain: Chain, ty: TypeId, cb: PsCallbackF, prio: i32) -> i32;

}

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Chain {
    InterceptEvent = 1,
    InterceptBefore = 2,
    InterceptAfter = 3,
    CaptureEvent = 4,
    CaptureBefore = 5,
    CaptureAfter = 6,
}

pub trait DiceEvent: Sized + 'static {
    const ID: TypeId;

    #[inline]
    unsafe fn from_raw<'a>(ptr: *const c_void) -> Option<&'a Self> {
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(ptr as *const Self) })
        }
    }
}

pub mod thread {
    use super::*;
    unsafe extern "C" {
        pub fn self_id(mt: *mut Metadata) -> DiceThreadId;
        pub fn self_retired(mt: *mut Metadata) -> bool;
        pub fn self_tls(mt: *mut Metadata, global: *const c_void, size: usize) -> *mut c_void;

    }
}

extern "C" {
    fn mempool_alloc(size: usize) -> *mut core::ffi::c_void;
    fn mempool_realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    fn mempool_free(ptr: *mut core::ffi::c_void);
}

pub struct MempoolAllocator;

unsafe impl GlobalAlloc for MempoolAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Assumes mempool_alloc returns memory aligned for `layout.align()`.
        mempool_alloc(layout.size()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mempool_free(ptr as *mut core::ffi::c_void);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        mempool_realloc(ptr as *mut core::ffi::c_void, new_size) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = mempool_alloc(layout.size()) as *mut u8;
        if !p.is_null() {
            ptr::write_bytes(p, 0, layout.size());
        }
        p
    }
}
