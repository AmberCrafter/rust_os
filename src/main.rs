#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]

mod library;

use core::panic::PanicInfo;
use library::bootor;

#[cfg(not(test))]
entry_point!(kernel::main);
#[cfg(not(test))]
mod kernel {
    use rustos::println;
    pub fn main() {
        println!("Hello world");

        rustos::init();
        x86_64::instructions::interrupts::int3();

        println!("It did not crash!");
    }
    
    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        super::println!("{}", info);
        loop {}
    }
}

#[cfg(test)]
entry_point!(tests::main);
#[cfg(test)]
mod tests {
    pub fn main() {
        super::test_main();
    }
    
    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        rustos::unittest::test_panic_handler(info)
    }
}