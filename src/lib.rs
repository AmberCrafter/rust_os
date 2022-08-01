#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]

pub mod library;
use library::interrupts;
pub use library::unittest;
use core::panic::PanicInfo;

pub fn init() {
    interrupts::init_idt();
}


/// Entry point for `cargo test`
#[cfg(test)]
entry_point!(tests::main);
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     test_main();
//     loop {}
// }

#[cfg(test)]
mod tests{
    use x86_64;

    pub fn main() {
        super::init(); // init interrupts
        super::test_main();
    }

    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        crate::unittest::test_panic_handler(info)
    }


    #[test_case]
    fn test_breakpoint_exception() {
        // invoke a breakpoint exception
        x86_64::instructions::interrupts::int3();
    }
}

