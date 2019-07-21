
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Char {
    pub ch: u8,
    pub color: ColorCode,
}

impl Char {
    pub fn new(ch: u8, color: ColorCode) -> Char {
        Char { ch, color }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Purple = 5,
    Cyan = 6,
    LightGray = 7,
    DarkGray = 8,
    LightRed = 9,
    LightGreen = 10,
    LightYellow = 11,
    LightBlue = 12,
    LightPurple = 13,
    LightCyan = 14,
    White = 15,
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fg: Color, bg: Color, blink: bool) -> ColorCode {
        ColorCode((fg as u8) | (((bg as u8) & 7) << 4) | ((blink as u8) << 7))
    }
}