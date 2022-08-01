#![no_std]
#![no_main]


// ----------------------------------------------------------------------------
// tester preprocess
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]
// ----------------------------------------------------------------------------

use core::panic::PanicInfo;
use core::fmt::Write;

mod lib;
use lib::vga_buffer;
use lib::qemu;

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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[Failed]");
    println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}



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
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[Ok]");
    }
}


// tester implement
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}
// ----------------------------------------------------------------------------


// test case
#[test_case]
fn trivial_assertion() {
    assert_eq!(1,1);
}
