use super::qemu;
// use super::serial;
// use super::vga_buffer;

use crate::library::bootor::hlt_loop;
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
    hlt_loop()
}

pub fn test_should_panic_handler(info: &PanicInfo) -> ! {
    println!("[Ok]");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
    hlt_loop()
}

// ----------------------------------------------------------------------------