use crate::{DiceEvent, TypeId};
use libc::{c_char, c_int, c_void};

// TODO: autogenerate this
pub mod raw {
    use crate::TypeId;
    pub const EVENT_MA_READ: TypeId = 30;
    pub const EVENT_MA_WRITE: TypeId = 31;
    pub const EVENT_MA_AREAD: TypeId = 32;
    pub const EVENT_MA_AWRITE: TypeId = 33;
    pub const EVENT_MA_RMW: TypeId = 34;
    pub const EVENT_MA_XCHG: TypeId = 35;
    pub const EVENT_MA_CMPXCHG: TypeId = 36;
    pub const EVENT_MA_CMPXCHG_WEAK: TypeId = 37;
    pub const EVENT_MA_FENCHE: TypeId = 38;

    pub const EVENT_MALLOC: TypeId = 50;
    pub const EVENT_CALLOC: TypeId = 51;
    pub const EVENT_REALLOC: TypeId = 52;
    pub const EVENT_FREE: TypeId = 53;
    pub const EVENT_POSIX_MEMALIGN: TypeId = 54;
    pub const EVENT_ALIGNED_ALLOC: TypeId = 55;
}

use raw::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MallocEvent {
    pub pc: *const (),
    pub size: usize,
    pub ret: *const (),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReadEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub addr: *mut c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub addr: *mut c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ma_val {
    pub u8_: u8,
    pub u16_: u16,
    pub u32_: u32,
    pub u64_: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AtomicReadEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub addr: *mut c_void,
    pub size: usize,
    pub mo: c_int,
    pub val: ma_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AtomicWriteEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub addr: *mut c_void,
    pub size: usize,
    pub mo: c_int,
    pub val: ma_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XCHGEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub addr: *mut c_void,
    pub size: usize,
    pub mo: c_int,
    pub val: ma_val,
    pub old: ma_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RMWEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub addr: *mut c_void,
    pub size: usize,
    pub mo: c_int,
    pub val: ma_val,
    pub old: ma_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CMPEXCHGEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub addr: *mut c_void,
    pub size: usize,
    pub mo: c_int,
    pub val: ma_val,
    pub cmp: ma_val,
    pub old: ma_val,
    pub ok: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceEvent {
    pub pc: *const c_void,
    pub func: *const c_char,
    pub mo: c_int,
}

impl DiceEvent for MallocEvent {
    const ID: TypeId = EVENT_MALLOC;
}

impl DiceEvent for AtomicReadEvent {
    const ID: TypeId = EVENT_MA_AREAD;
}

impl DiceEvent for AtomicWriteEvent {
    const ID: TypeId = EVENT_MA_AWRITE;
}

impl DiceEvent for XCHGEvent {
    const ID: TypeId = EVENT_MA_XCHG;
}

impl DiceEvent for RMWEvent {
    const ID: TypeId = EVENT_MA_RMW;
}

impl DiceEvent for CMPEXCHGEvent {
    const ID: TypeId = EVENT_MA_CMPXCHG;
}

impl DiceEvent for FenceEvent {
    const ID: TypeId = EVENT_MA_FENCHE;
}
