#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]


pub mod library;
pub use library::unittest;
pub use library::bootloader;
use core::panic::PanicInfo;

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
    pub fn main() {
        super::test_main();
    }

    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        crate::unittest::test_panic_handler(info)
    }
}

