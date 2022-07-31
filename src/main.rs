#![no_std]
#![no_main]

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

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}