pub use dice_sys as sys;
pub use dice_sys::{Chain, DiceResult, DiceThreadId, Metadata, TypeId};

pub mod events {
    pub use crate::sys::events::*;
}

pub mod thread {
    use std::ptr::NonNull;

    use dice_sys::{DiceThreadId, Metadata};

    pub fn self_id(mt: &mut Metadata) -> DiceThreadId {
        let id = unsafe { dice_sys::thread::self_id(mt) };
        id
    }

    pub fn self_tls_ptr<T>(meta: &mut Metadata, global: *const T) -> Option<NonNull<T>> {
        let p = unsafe {
            dice_sys::thread::self_tls(
                meta as *mut Metadata,
                (global as *const T).cast::<libc::c_void>(),
                std::mem::size_of::<T>(),
            )
        };
        NonNull::new(p.cast::<T>())
    }

    pub fn self_tls<'a, T>(meta: &'a mut Metadata, global: &'static T) -> Option<&'a T> {
        self_tls_ptr(meta, global).map(|nn| unsafe { nn.as_ref() })
    }

    pub fn with_self_tls<T, R>(
        meta: &mut Metadata,
        global: &'static T,
        f: impl FnOnce(&T) -> R,
    ) -> Option<R> {
        self_tls(meta, global).map(f)
    }

    pub unsafe fn self_tls_mut<'a, T>(
        meta: &'a mut Metadata,
        global: *const T,
    ) -> Option<&'a mut T> {
        self_tls_ptr(meta, global).map(|mut nn| nn.as_mut())
    }

    pub fn with_self_tls_mut<T, R>(
        meta: &mut Metadata,
        global: *const T,
        f: impl FnMut(&mut T) -> R,
    ) -> Option<R> {
        unsafe { self_tls_mut(meta, global).map(f) }
    }
}

#[macro_export]
macro_rules! subscribe_scoped {
    ($chain:expr, $prio:expr, |$e:ident: &$t:ty, $m:ident| $body:block) => {{
        // no capture guard
        let _guard: fn(&$t, &mut $crate::Metadata) -> $crate::DiceResult =
            |$e: &$t, $m: &mut $crate::Metadata| $body;

        extern "C" fn __trampoline(
            chain: $crate::Chain,
            _ty: $crate::TypeId,
            event: *const core::ffi::c_void,
            md: *mut $crate::Metadata,
        ) -> $crate::DiceResult {
            let Some(ev_ref) = (unsafe { <$t as $crate::sys::DiceEvent>::from_raw(event) }) else {
                return $crate::DiceResult::Invalid;
            };
            let __chain = chain;

            let $e: &$t = ev_ref;
            let Some($m) = (unsafe { md.as_mut() }) else {
                return $crate::DiceResult::Invalid;
            };

            $body
        }

        unsafe {
            $crate::sys::ps_subscribe(
                $chain,
                <$t as $crate::sys::DiceEvent>::ID,
                Some(__trampoline),
                $prio,
            )
        }
    }};
}

#[macro_export]
macro_rules! subscribe {
    ($chain:expr, $prio:expr, |$e:ident: &$t:ty, $m:ident| $body:block) => {
        const _: () = {
            #[allow(non_snake_case)]
            #[::ctor::ctor]
            fn __dice_subscribe_ctor() {
                use ::std::sync::Once;
                static INIT: Once = Once::new();

                INIT.call_once(|| {
                    let _ = $crate::subscribe_scoped!($chain, $prio, |$e: &$t, $m| $body);
                });
            }
        };
    };
}

mod test {
    use crate::{events::*, thread::*, Chain, DiceResult};
    use dice_sys::MempoolAllocator;

    #[global_allocator]
    static GLOBAL: MempoolAllocator = MempoolAllocator;

    subscribe!(Chain::CaptureBefore, 9999, |event: &MallocEvent, meta| {
        let _thread_id = self_id(meta);
        // println!("dicers: before: malloc: t: {}, {}", thread_id, event.size);
        DiceResult::Ok
    });

    subscribe!(Chain::CaptureAfter, 9999, |event: &MallocEvent, meta| {
        let _thread_id = self_id(meta);
        // println!("dicers: after: malloc: t: {}, {}", thread_id, event.size);
        DiceResult::Ok
    });

    struct MiniLotto { 
        counter: i32,
        queue: Vec<i32>,
    }

    static mut MINI_LOTTO: MiniLotto = MiniLotto { counter: 0, queue: Vec::new() };

    subscribe!(
        Chain::CaptureBefore,
        9999,
        |event: &AtomicReadEvent, meta| {
            let _thread_id = self_id(meta);
            with_self_tls_mut(meta, &raw const MINI_LOTTO, |mini| {
                mini.counter += 1;
                println!("WOW0 {:?}", _thread_id);
                mini.queue.push(mini.counter);
                println!("WOW1 {:?}", _thread_id);

                println!("dicers_queue: {:?}", mini.queue);
            });
            DiceResult::Ok
        }
    );

    subscribe!(
        Chain::CaptureAfter,
        9999,
        |event: &AtomicReadEvent, meta| {
            let _thread_id = self_id(meta);
            // println!("dicers: after atomic_read: t: {}", thread_id);
            DiceResult::Ok
        }
    );



    subscribe!(
        Chain::CaptureBefore,
        9999,
        |event: &AtomicWriteEvent, meta| {
            let _thread_id = self_id(meta);
            // with_self_tls_mut(meta, &raw const MINI_LOTTO, |mini| {
            //     mini.queue.push(10);
            //     println!("mini_lotto queue: {:?}", mini.queue);
            // });
            DiceResult::Ok
        }
    );

    subscribe!(
        Chain::CaptureAfter,
        9999,
        |event: &AtomicWriteEvent, meta| {
            let _thread_id = self_id(meta);
            // println!("dicers: after atomic_write: t: {}", thread_id);
            DiceResult::Ok
        }
    );

    // #[ctor::dtor]
    // fn destroy_thread_locals() { 

    // }
}
