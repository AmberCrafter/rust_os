#[macro_export]
macro_rules! entry_point {
    ($func:expr) => {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            $func();
            loop{}
        }
    };
}