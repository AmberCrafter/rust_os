// view
// heap:    |-----------------------------|
//           ^ next
// step1:   |aaaaabbbbb-------------------|
//                     ^ next
// step2:   |aaaaabbbbbccccc--------------|
//                          ^ next
// step3:   |aaaaa-----ccccc--------------|
//                          ^ next
// step4:   |aaaaa-----cccccddddddd-------|
//                                 ^ next
// step5:   |aaaaa------------------------|
//                                 ^ next
// ...
// reset:   |----------------------------|
//           ^ next
//
// > due to allocations still has 1 (occupy by aaaaa)
// > thus this heap doesn't reuse those free space
// > until all allocations be free (allocations = 0).
//

use alloc::alloc::{GlobalAlloc, Layout};
use spin;

use crate::println;

pub struct BumpAllocator {
    heap_start: usize, 
    heap_end: usize,
    next: usize, 
    allocations: usize
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator { heap_start: 0, heap_end: 0, next: 0, allocations: 0 }
    }


    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start+heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for super::Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        // align address
        let alloc_start = super::align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return core::ptr::null_mut()
        };
        if alloc_end>bump.heap_end {
            core::ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();
        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}

