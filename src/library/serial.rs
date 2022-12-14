use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe {
            SerialPort::new(0x3f8)
        };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("[Error] Printing to serial port failed.")
}

// print to the host through the serial port
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::library::serial::_print(format_args!($($arg)*))
    };
}

// print to the host through the serial interface, with padding a new line
#[macro_export]
macro_rules! serial_println {
    () => {
        ($crate::library::serial::_print("\n"));
    };
    ($fmt:expr) => {
        ($crate::serial_print!(concat!($fmt, "\n")));
    };
    ($fmt:expr, $($arg:tt)*) => {
        ($crate::serial_print!(
            concat!($fmt, "\n"), $($arg)*
        ));
    };
}

