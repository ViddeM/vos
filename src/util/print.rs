use core::ptr::{read_volatile, write_volatile};

const VIDEO_MEMORY_ADDRESS: u32 = 0xb8000;
const NUM_COLS: u8 = 80;
const NUM_ROWS: u8 = 25;

pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

impl From<u8> for Color {
    fn from(val: u8) -> Self {
        match val {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            15 => Color::White,
            _ => {
                panic!("Invalid color received")
            }
        }
    }
}

impl From<Color> for u8 {
    fn from(col: Color) -> Self {
        match col {
            Color::Black => 0,
            Color::Blue => 1,
            Color::Green => 2,
            Color::Cyan => 3,
            Color::Red => 4,
            Color::Magenta => 5,
            Color::Brown => 6,
            Color::LightGray => 7,
            Color::DarkGray => 8,
            Color::LightBlue => 9,
            Color::LightGreen => 10,
            Color::LightCyan => 11,
            Color::LightRed => 12,
            Color::Pink => 13,
            Color::Yellow => 14,
            Color::White => 15,
        }
    }
}

pub struct CharColor {
    bg_color: Color,
    fg_color: Color,
}

impl From<u8> for CharColor {
    fn from(val: u8) -> Self {
        CharColor {
            bg_color: Color::from(val & 0x70), // Get the 3 highest bits
            fg_color: Color::from(val & 0x0F), // Get the 4 lowest bits
        }
    }
}

impl From<CharColor> for u8 {
    fn from(char_color: CharColor) -> u8 {
        let bg: u8 = char_color.bg_color.into();
        let fg: u8 = char_color.fg_color.into();
        return (bg << 4) | fg;
    }
}

struct Char {
    character: u8,
    color: u8,
}

impl From<u16> for Char {
    fn from(val: u16) -> Self {
        let color = (val >> 8) as u8;
        let character = val as u8;
        return Char { character, color };
    }
}

impl From<Char> for u16 {
    fn from(char: Char) -> Self {
        return ((char.color as u16) << 8) | char.character as u16;
    }
}

fn write_char(char: Char, row: u8, col: u8) {
    let mut lower = false;
    let aligned_col = if col % 2 == 1 {
        lower = true;
        col - 1
    } else {
        col
    };

    let address = (VIDEO_MEMORY_ADDRESS + (aligned_col + (aligned_col * row)) as u32) as *mut u32;
    let prev_data: u32 = unsafe { read_volatile::<u32>(address) };
    let prev_data = 0;
    let char_data: u32 = u16::from(char) as u32;
    let new_data = if lower {
        prev_data | char_data
    } else {
        prev_data | (char_data << 16)
    };
    unsafe { write_volatile(address, new_data) };
}

pub fn print_str(a: &str) {
    // unsafe { write_volatile(VIDEO_MEMORY_ADDRESS, 0x1F621F62) }
}

fn clear_row(row: u8) {
    for col in 0..NUM_COLS {
        write_char(
            Char {
                character: b" ",
                color: CharColor {
                    fg_color: Color::Black,
                    bg_color: Color::White,
                }
                .into(),
            },
            row,
            col,
        )
    }
}

pub fn clear_screen() {
    for row in 0..NUM_ROWS {
        clear_row(row)
    }
}
