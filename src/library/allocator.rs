use core::ptr::null_mut;

use alloc::alloc::{GlobalAlloc, Layout};
use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, mapper::MapToError, Page, PageTableFlags}, VirtAddr};

pub struct Dummy;
// virtural memory start position
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100*1024;

#[allow(unused)]
unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        panic!("dealloc should be never called")
    }
}

#[allow(unused)]
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) -> Result<(), MapToError<Size4KiB>> {
    // get valid page range of heap
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        // create a mapping relation between frame (Physical memory) and page (Virtual memory)
        // flags give the privileges
        // frame_allocator used to generate the level table if not exist.
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)
        };
    }
    Ok(())
}