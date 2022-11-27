use core::fmt;
pub const WIDTH: u16 = 80;
pub const HEIGHT: u16 = 25;

pub const BUFFER_PTR: *mut u16 = 0xb8000 as *mut _;

/// (Foreground, Background)
pub type ColorPair = (Color, Color);
pub const DEFAULT_COLOR: ColorPair = (Color::White, Color::Black);

#[derive(Debug, Clone)]
pub struct Formatter {
    pub pos: Position,
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        write(data, &mut self.pos);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Position {
    pub fn offset(&self) -> isize {
        (self.y * WIDTH + self.x) as isize
    }
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Color {
    Black = 0x0,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGray = 0x07,
    DarkGray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0a,
    LightCyan = 0x0b,
    LightRed = 0x0c,
    Pink = 0x0d,
    Yellow = 0x0e,
    White = 0x0f,
}

pub fn write_byte(data: u8, color: &ColorPair, pos: &mut Position) {
    unsafe { *BUFFER_PTR.offset(pos.offset()) = entry_to_bytes(data, color) }
    pos.x += 1;
}

pub fn write<T>(data: T, pos: &mut Position)
where
    T: AsRef<str>,
{
    data.as_ref()
        .as_bytes()
        .iter()
        .for_each(|byte| write_byte(*byte, &DEFAULT_COLOR, pos));
}

pub fn write_fmt(data: fmt::Arguments, pos: &mut Position) -> fmt::Result {
    let mut formatter = Formatter { pos: pos.clone() };
    Ok(fmt::write(&mut formatter, data)?)
}

pub fn write_colored<T>(data: T, color: ColorPair, pos: &mut Position)
where
    T: AsRef<str>,
{
    data.as_ref()
        .as_bytes()
        .iter()
        .for_each(|byte| write_byte(*byte, &color, pos));
}

#[inline]
fn entry_to_bytes(data: u8, color: &ColorPair) -> u16 {
    data as u16 | (color_pair_to_bytes(color) as u16) << 8
}

#[inline]
fn color_pair_to_bytes(pair: &ColorPair) -> u8 {
    (pair.0 as u8) | (pair.1 as u8) << 4
}
