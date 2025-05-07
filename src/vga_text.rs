// Module for VGA text mode interaction 

use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// --- Color Definitions ---
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// --- Screen Character and Buffer ---
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;

/// Represents a character on the VGA screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode, // Use ColorCode struct
}

/// Represents the VGA text buffer.
#[repr(transparent)]
struct Buffer {
    // Use Volatile for memory-mapped I/O
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// --- Writer Implementation ---
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not part of printable ASCII range
                _ => self.write_byte(0xfe), // Print '■' character
            }
        }
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
}

// --- Formatting Macro Support ---
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// --- Global Writer Instance ---
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut Buffer) },
    });
}

// Helper function called by macros in lib.rs
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) { // Ensure pub
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Sets the foreground and background colors for subsequent prints.
pub fn set_text_color(foreground: Color, background: Color) {
    WRITER.lock().set_color(foreground, background);
}

/// Writes a raw byte slice directly (simple version, less safe)
/// Kept for compatibility/debugging if needed, prefer println!
// pub fn print_message(message: &[u8]) {
//     let mut writer = WRITER.lock();
//     for &byte in message {
//          if byte == b'\n' {
//              writer.new_line();
//          } else if byte >= 0x20 && byte <= 0x7e { // Printable ASCII
//              writer.write_byte(byte);
//          } else {
//              writer.write_byte(0xfe); // non-printable block
//          }
//     }
// }

pub fn print_message(message: &[u8]) {
    use core::fmt::Write;
    // Convert byte slice to string (assuming valid UTF-8 or handling errors)
    // For simple ASCII, direct byte writing might be okay, but let's use the fmt writer
    // Note: This assumes the byte slice is valid UTF-8. Handle potential errors if not.
    if let Ok(s) = core::str::from_utf8(message) {
        WRITER.lock().write_str(s).unwrap();
    } else {
        // Handle non-UTF8 bytes - maybe print hex or replacement char?
        for &byte in message {
            WRITER.lock().write_byte(byte);
        }
    }
    // Add a newline for clarity, similar to old print_debug
    // WRITER.lock().new_line(); 
}

// Simple function to clear the screen using the writer
pub fn clear_screen() { // Ensure pub
    let mut writer = WRITER.lock();
    for row in 0..BUFFER_HEIGHT {
        writer.clear_row(row);
    }
    writer.column_position = 0; // Reset cursor
}

// --- Testing (Optional) ---
// #[test_case]
// fn test_println_simple() {
//     println!("test_println_simple output");
// }
// 
// #[test_case]
// fn test_println_many() {
//     for _ in 0..200 {
//         println!("test_println_many output");
//     }
// }
// 
// #[test_case]
// fn test_println_output() {
//     let s = "Some test string that fits on a single line";
//     println!("{}", s);
//     for (i, c) in s.chars().enumerate() {
//         let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
//         assert_eq!(char::from(screen_char.ascii_character), c);
//     }
// } 