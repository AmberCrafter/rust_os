#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)] 
#![feature(const_mut_refs)]

extern crate alloc;

pub mod library;
pub use library::unittest;
use library::{interrupts, gdt, allocator, memory, concurrency::task::keyboard};
use linked_list_allocator::LockedHeap;
#[allow(unused)]
use core::panic::PanicInfo;
#[allow(unused)]
use bootloader::entry_point;
#[allow(unused)]
use bootloader::BootInfo;
use x86_64::VirtAddr;

#[global_allocator]
// static ALLOCATOR: library::allocator::Dummy = library::allocator::Dummy;
// static ALLOCATOR: LockedHeap = LockedHeap::empty();
// static ALLOCATOR: allocator::Locked<allocator::bump::BumpAllocator>
//     = allocator::Locked::new(allocator::bump::BumpAllocator::new());
// static ALLOCATOR: allocator::Locked<allocator::linked_list::LinkedListAllocator>
//     = allocator::Locked::new(allocator::linked_list::LinkedListAllocator::new());
static ALLOCATOR: allocator::Locked<allocator::fixed_size_block::FixedSizeBlockAllocator>
    = allocator::Locked::new(allocator::fixed_size_block::FixedSizeBlockAllocator::new());

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocatoion error: {:?}", layout)
}


pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize()
    };
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    // let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initializetion failed");

    unsafe {
        // used for linked_list_allocator crate
        // let heap_start = VirtAddr::new(allocator::HEAP_START as u64);
        // ALLOCATOR.lock().init(heap_start.as_mut_ptr(), allocator::HEAP_SIZE);  
        
        ALLOCATOR.lock().init(allocator::HEAP_START, allocator::HEAP_SIZE);
    };
    

    // keyboard::init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}



// Entry point for `cargo test`
#[cfg(test)]
entry_point!(tests::main);
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     test_main();
//     loop {}
// }

#[cfg(test)]
mod tests{
    use super::{print, println};
    use bootloader::BootInfo;

    use x86_64;

    pub fn main(boot_info: &'static BootInfo) -> ! {
        super::init(&boot_info); // init interrupts
        super::test_main();
        super::hlt_loop()
    }

    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        crate::unittest::test_panic_handler(info)
    }


    #[test_case]
    fn test_breakpoint_exception() {
        print!("src/lib::test_breakpoint_exception...");

        // invoke a breakpoint exception
        x86_64::instructions::interrupts::int3();
        println!("src/lib::test_breakpoint_exception...");
    }
}

