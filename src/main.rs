#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

extern crate  alloc;

mod library;

use core::panic::PanicInfo;
// use library::bootor;
use bootloader::entry_point;
#[allow(unused)]
use bootloader::BootInfo;

#[cfg(not(test))]
entry_point!(kernel::main);
#[cfg(not(test))]
mod kernel {
    use alloc::{boxed::Box, vec::Vec, vec, rc::Rc};
    use rustos::{println, library::{memory, allocator}};
    use bootloader::BootInfo;
    use x86_64::{VirtAddr, structures::paging::Page};
    pub fn main(boot_info: &'static BootInfo) -> ! {
        println!("Hello world");

        rustos::init(&boot_info);
        
        let heap_value = Box::new(41);
        println!("heap_value at {:p}", heap_value);

        let mut vec = Vec::new();
        for i in 0..500 {
            vec.push(i);
        }
        println!("vec at {:p}", vec.as_slice());

        let reference_counted = Rc::new(vec![1,2,3]);
        let cloned_reference = reference_counted.clone();
        println!("[Clone Rc] current reference count is {}", Rc::strong_count(&cloned_reference));
        core::mem::drop(reference_counted);
        println!("[Drop Rc] current reference count is {}", Rc::strong_count(&cloned_reference));

        println!("It did not crash!");
        // loop {}
        rustos::hlt_loop()
    }
    
    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        super::println!("{}", info);
        // loop {}
        rustos::hlt_loop()
    }
}

#[cfg(test)]
entry_point!(tests::main);
#[cfg(test)]
mod tests {
    use rustos::hlt_loop;
    use bootloader::BootInfo;

    pub fn main(_boot_info: &'static BootInfo) -> !{
        super::test_main();
        hlt_loop()
    }
    
    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        rustos::unittest::test_panic_handler(info)
    }
}