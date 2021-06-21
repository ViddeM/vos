#[derive(Clone, Copy, Debug)]
pub struct Char {
    pub character: u8,
    pub color: u8,
}

impl From<u16> for Char {
    fn from(val: u16) -> Self {
        let color = (val >> 8) as u8;
        let character = val as u8;
        return Char { character, color };
    }
}

impl From<&Char> for u16 {
    fn from(char: &Char) -> Self {
        return ((char.color as u16) << 8) | char.character as u16;
    }
}
