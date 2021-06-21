#[derive(Clone, Copy, Debug)]
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
