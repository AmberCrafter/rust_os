use x86_64;
pub fn hlt_loop() -> !{
    loop {
        x86_64::instructions::hlt();
    }
}


#[macro_export]
macro_rules! custom_entry_point {
    ($func:expr) => {
        use bootloader::BootInfo;
        #[no_mangle]
        pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
            $func();
            loop {}
        }
    };
}