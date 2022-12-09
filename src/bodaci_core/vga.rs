use core::fmt;

pub const WIDTH: u16 = 80;
pub const HEIGHT: u16 = 25;

pub const BUFFER_PTR: *mut u16 = 0xb8000 as *mut _;

pub static mut POSITION: Position = Position::new();

/// (Foreground, Background)
pub type ColorPair = (Color, Color);
pub const DEFAULT_COLOR: ColorPair = (Color::White, Color::Black);

#[derive(Debug, Clone)]
pub struct Formatter {
    pub pos: Position,
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        write(data);
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
    pub const fn new() -> Self {
        Self { x: 0, y: 0 }
    }

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

pub fn write_byte(data: char, color: &ColorPair) {
    let pos = current_position_mut();

    match data {
        '\n' => newline(),
        _ => {
            unsafe { *BUFFER_PTR.offset(pos.offset()) = entry_to_bytes(data, color) };
            pos.x += 1;
        }
    }
}

pub fn write<T>(data: T)
where
    T: AsRef<str>,
{
    data.as_ref()
        .as_bytes()
        .iter()
        .for_each(|byte| write_byte(*byte as char, &DEFAULT_COLOR));
}

pub fn _write_fmt(data: fmt::Arguments) {
    let pos = current_position_mut();
    let mut formatter = Formatter { pos: pos.clone() };
    fmt::write(&mut formatter, data).unwrap()
}

pub fn write_colored<T>(data: T, color: ColorPair)
where
    T: AsRef<str>,
{
    data.as_ref()
        .as_bytes()
        .iter()
        .for_each(|byte| write_byte(*byte as char, &color));
}

pub fn newline() {
    let pos = current_position_mut();
    pos.y += 1;
    pos.x = 0;
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::vga::_write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (crate::vga::print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[inline]
pub fn current_position() -> &'static Position {
    unsafe { &POSITION }
}

#[inline]
pub fn current_position_mut() -> &'static mut Position {
    unsafe { &mut POSITION }
}

#[inline]
fn entry_to_bytes(data: char, color: &ColorPair) -> u16 {
    data as u16 | (color_pair_to_bytes(color) as u16) << 8
}

#[inline]
fn color_pair_to_bytes(pair: &ColorPair) -> u8 {
    (pair.0 as u8) | (pair.1 as u8) << 4
}
