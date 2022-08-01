#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

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