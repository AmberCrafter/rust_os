use volatile::Volatile;
use core::fmt;

// color enum
// In VGA Text Mod, color has only 4bit for foreground color and 3bit for background color
// which the 4-th bit is light version, thus the backround color didn't need
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
enum Color {
    Black = 0,      // 0000
    Blue = 1,       // 0001
    Green = 2,      // 0010
    Cyan = 3,       // 0011
    Red = 4,        // 0100
    Magenta = 5,    // 0101
    Brown = 6,      // 0110
    LightGray = 7,  // 0111
    DarkGray = 8,   // 1000
    LightBlue = 9,  // 1001
    LightGreen = 10,// 1010
    LightCyan = 11, // 1011
    LightRed = 12,  // 1100
    Pink = 13,      // 1101
    Yellow = 14,    // 1110
    White = 15      // 1111
}

// Color code
// #[repr(transparent)] mean alias this struct into target type (memory layout)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        // use 0b0111_1111 to make sure the ColorCode always valid
        Self (((background as u8) <<4 | (foreground as u8)) & 0b0111_1111)
    }
}

// ----------------------------------------------------------------------------
// Character type in VGA Text Mode
// VGA Text Mod memory layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode
}

// Screen layout
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Screen Buffer
// use volatile wrapper ScreenChar to prevent optimization modifed these segments
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}
// ----------------------------------------------------------------------------

// instance of VGA writer
// buffer direct to point the start of VGA Text Mode memory position
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.column_position>=BUFFER_WIDTH {
                    self.newline();
                }

                let row = BUFFER_HEIGHT-1;
                let col = self.column_position;

                let color_code = self.color_code;
                
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code
                });
                self.column_position+=1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // valid ascii char
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // invalid vase, replace as 0xfe
                _ => self.write_byte(0xfe)
            }
        }
    }

    pub fn newline(&mut self) {
        // shift up each row one line and clear last one
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let value = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(value);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar { ascii_character: b' ', color_code: self.color_code };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// formatting trait
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// global interface
use lazy_static::lazy_static;
use spin::Mutex;

use crate::library::serial::SERIAL1;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


// macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::library::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

// #[doc(hidden)]
// pub fn _print(args: fmt::Arguments) {
//     use core::fmt::Write;
//     WRITER.lock().write_fmt(args).unwrap();
// }

// test case
#[allow(unused)]
pub fn test_hello_word() {
    use core::fmt::Write;
    // init writer
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
    };

    writer.write_byte(b'H');
    writer.write_string("ello W");
    // writer.write_string("orld");
    write!(writer, "or{}. pi:{}", "ld", 3.1415926).unwrap();
}


#[test_case]
fn test_vga_print_is_show() {
    use x86_64::instructions::interrupts;
    use core::fmt::Write;
    let s = "Test VGA print is show on screen.";
    
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(
            writer, "\n{}", s
        ).expect("writeln falied");
        for (i, c) in s.chars().enumerate() {
            let screenchar = writer.buffer.chars[BUFFER_HEIGHT-2][i].read();
            assert_eq!(c, char::from(screenchar.ascii_character));
        }
    });
}