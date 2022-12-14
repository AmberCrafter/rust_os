#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::unittest::test_runner)]
#![reexport_test_harness_main="test_main"]

use core::panic::PanicInfo;
use bootloader::BootInfo;

use rustos::{println, hlt_loop};
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    test_main();

    hlt_loop()
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

// set in crate::lib
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hlt_loop()
}

#[test_case]
fn test_println() {
    println!("test println!")
}