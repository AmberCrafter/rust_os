#![no_std]
#![no_main]


// ----------------------------------------------------------------------------
// tester implement
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
// ----------------------------------------------------------------------------

use core::panic::PanicInfo;
use core::fmt::Write;

mod lib;
use lib::vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello macro{}", " !");
    // vga_buffer::test_hello_word();
    // vga_buffer::WRITER.lock().write_str("test message!").unwrap();
    println!("hello macro{}", " !");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    println!("trivial assertion... ");
    assert_eq!(1,1);
    println!("[Ok]");
}
