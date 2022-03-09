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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct PrintChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct PrintBuffer {
    chars: [[PrintChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGABufferWriter {
    current_column: usize,
    color_code: ColorCode,
    buffer: &'static mut PrintBuffer,
}

impl VGABufferWriter {
    fn new(color_code: ColorCode) -> VGABufferWriter {
        VGABufferWriter {
            current_column: 0,
            color_code,
            buffer: unsafe { &mut *(0xb8000 as *mut PrintBuffer) },
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.current_column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.current_column;
                let color_code = self.color_code;

                self.buffer.chars[row][col] = PrintChar {
                    ascii_character: byte,
                    color_code,
                };

                self.current_column += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) -> ! {
        loop {}
    }
}

pub fn print_something() {
    let mut writer = VGABufferWriter {
        current_column: 0,
        color_code: ColorCode::new(Color::Magenta, Color::Yellow),
        buffer: unsafe { &mut *(0xb8000 as *mut PrintBuffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
