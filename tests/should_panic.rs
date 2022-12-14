#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos::{library::qemu, serial_print, serial_println, hlt_loop};
use bootloader::entry_point;
use bootloader::BootInfo;


entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    test_main();
    hlt_loop()
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[This test didn't panic!]");
        qemu::exit_qemu(qemu::QemuExitCode::Failed);
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    serial_println!("[Ok]");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
    hlt_loop()
}

// test case
#[test_case]
fn should_failed() {
    serial_print!("tests/should_panic.rs::should_failed...");
    assert_eq!(0,1);
}