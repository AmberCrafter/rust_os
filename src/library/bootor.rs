
#[macro_export]
macro_rules! entry_point {
    ($func:expr) => {
        use x86_64;
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            $func();
            loop {
                x86_64::instructions::hlt();
            }
        }
    };
}

#[macro_export]
macro_rules! entry_point_infinit_print {
    ($func:expr) => {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            use crate::print;
            $func();
            loop{
                print!("-");
            }
        }
    };
}