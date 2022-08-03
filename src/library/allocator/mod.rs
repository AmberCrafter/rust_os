pub mod dummy;
pub mod bump;
pub mod linked_list;
pub mod fixed_size_block;

use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, mapper::MapToError, Page, PageTableFlags}, VirtAddr};

// virtural memory start position
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100*1024;

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


pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked { inner: spin::Mutex::new(inner) }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    // let remainder = addr % align;
    // if remainder == 0 {
    //     addr
    // } else {
    //     addr-remainder+align
    // }


    // example
    // addr:  9 (0001_0001)
    // align: 4 (0000_0100)
    //
    // (addr + align -1): 0001_0100
    // (align - 1):       0000_0011
    // !(align - 1):      1111_1100
    // (addr + align -1) & !(align-1):  0001_0100 (12)

    (addr + align -1) & !(align-1)
}