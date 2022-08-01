#![no_std]

#![cfg_attr(test, no_main)]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::libs::serial;
use crate::libs::qemu;
use crate::libs::vga_buffer;

use crate::serial_println;
use crate::serial_print;
use crate::print;
use crate::println;

use core::panic::PanicInfo;
// ----------------------------------------------------------------------------
// set auto invoke printing of testing message
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T 
where
    T: Fn()
{
    fn run(&self) -> () {
        serial_print!("{}... ", core::any::type_name::<T>());
        self();
        serial_println!("[Ok]");
    }
}


// tester implement
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[Failed]");
    println!("Error: {}\n", info);
    
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}

// ----------------------------------------------------------------------------

// Entry point for cargo test
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}