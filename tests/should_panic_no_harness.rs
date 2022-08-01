#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos::{library::qemu, entry_point, serial_println};


entry_point!(wrap_should_failed);

fn wrap_should_failed() {
    should_failed();
    serial_println!("[This test didn't panic!]");
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}

// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//         serial_println!("[This test didn't panic!]");
//         qemu::exit_qemu(qemu::QemuExitCode::Failed);
//     }
//     qemu::exit_qemu(qemu::QemuExitCode::Success);
// }

#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    serial_println!("[Ok]");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
    loop {}
}

// test case
// #[test_case]
fn should_failed() {
    serial_println!("tests/should_panic.rs::should_failed...");
    assert_eq!(0,1);
}