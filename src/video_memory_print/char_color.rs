use crate::video_memory_print::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct CharColor {
    pub bg_color: Color,
    pub fg_color: Color,
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
