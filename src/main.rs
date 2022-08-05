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

    // use crate::library::concurrency::task::keyboard;
    use rustos::library::concurrency::task::keyboard;
    
    use crate::library::concurrency::task::{simple_executor::SimpleExecutor, executor::Executor, Task};
    use rustos::{println, library::{memory, allocator}};
    use bootloader::BootInfo;
    use x86_64::{VirtAddr, structures::paging::Page};
    pub fn main(boot_info: &'static BootInfo) -> ! {
        println!("Hello world");

        rustos::init(&boot_info);
        
        let mut executor = Executor::new();
        executor.spawn(Task::new(example_task()));
        executor.spawn(Task::new(keyboard::print_keypresses()));
        executor.run();

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

    async fn async_number() -> u32 {
        42
    }

    async fn example_task() {
        let number = async_number().await;
        println!("async number: {}", number);
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