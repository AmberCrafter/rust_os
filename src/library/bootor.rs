use x86_64;
pub fn hlt_loop() -> !{
    loop {
        x86_64::instructions::hlt();
    }
}


#[macro_export]
macro_rules! entry_point {
    ($func:expr) => {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            $func();
            loop {}
        }
    };
}