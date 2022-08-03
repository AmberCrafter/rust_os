use core::ptr::null_mut;
use alloc::alloc::{GlobalAlloc, Layout};

pub struct Dummy;

#[allow(unused)]
unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        panic!("dealloc should be never called")
    }
}