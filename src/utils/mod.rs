use std::alloc::System;
use std::alloc::{GlobalAlloc, Layout};
use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::usize;
#[global_allocator]
pub static GLOBAL: Trallocator<System> = Trallocator::new(System);
// pub static GLOBAL: System = System;

pub trait Placeholder {
    fn reset(&self);
    fn get(&self) -> usize;
}

impl Placeholder for System {
    fn reset(&self) {}
    fn get(&self) -> usize {
        0
    }
}

thread_local!(pub static STACK_END: Cell<usize> = Cell::new(usize::MAX));

#[macro_export]
macro_rules! stack_ptr {
    () => ({
        use std::arch::asm;

        let x: usize;
        unsafe {
            asm!("mov {}, rsp", out(reg) x, options(nomem, nostack));
        }
        x
    })
}

#[macro_export]
macro_rules! tick {
    () => {{
        use crate::stack_ptr;
        use crate::utils::STACK_END;
        let stack_end = stack_ptr!();
        STACK_END.with(|c| {
            let best = std::cmp::min(c.get(), stack_end);
            c.set(best);
        });
    }};
}

pub fn measure<T, F: FnOnce() -> T>(callback: F) -> (T, usize) {
    STACK_END.with(|c| c.set(usize::MAX));
    let stack_start = stack_ptr!();
    let r = callback();
    let stack_end = STACK_END.with(|c| c.get());
    if stack_start < stack_end {
        panic!("tick!() was never called");
    }
    (r, stack_start - stack_end)
}

pub struct Trallocator<A: GlobalAlloc>(pub A, AtomicU64, AtomicU64);

unsafe impl<A: GlobalAlloc> GlobalAlloc for Trallocator<A> {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let size = l.size() as u64;
        let current = self.1.fetch_add(size, Ordering::SeqCst).wrapping_add(size);

        // Update peak if current usage exceeds previous peak
        let mut peak = self.2.load(Ordering::SeqCst);
        while current > peak {
            match self
                .2
                .compare_exchange(peak, current, Ordering::SeqCst, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(actual_peak) => peak = actual_peak,
            }
        }

        unsafe { self.0.alloc(l) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, l: Layout) {
        unsafe { self.0.dealloc(ptr, l) };
        self.1.fetch_sub(l.size() as u64, Ordering::SeqCst);
    }
}

impl<A: GlobalAlloc> Trallocator<A> {
    pub const fn new(a: A) -> Self {
        Trallocator(a, AtomicU64::new(0), AtomicU64::new(0))
    }

    pub fn reset(&self) {
        self.1.store(0, Ordering::SeqCst);
        self.2.store(0, Ordering::SeqCst);
    }
    pub fn get(&self) -> usize {
        self.2.load(Ordering::SeqCst) as usize
    }
}
