#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]

mod library;

use core::panic::PanicInfo;
use library::bootor;

#[cfg(not(test))]
entry_point!(kernel::main);
#[cfg(not(test))]
mod kernel {
    use rustos::println;
    pub fn main() {
        println!("Hello world");

        rustos::init();
        
        use x86_64::registers::control::Cr3;
        let (level_4_page_table, _) = Cr3::read();
        println!("Level 4 page table at: {:?}", level_4_page_table);


        println!("It did not crash!");
        // loop {}
        rustos::hlt_loop()
    }
    
    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        super::println!("{}", info);
        // loop {}
        rustos::hlt_loop()
    }
}

#[cfg(test)]
entry_point!(tests::main);
#[cfg(test)]
mod tests {
    use rustos::hlt_loop;

    pub fn main() -> !{
        super::test_main();
        hlt_loop()
    }
    
    #[panic_handler]
    fn panic(info: &super::PanicInfo) -> ! {
        rustos::unittest::test_panic_handler(info)
    }
}