use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, IntEnum, Copy, Clone, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => match color {
            ResistorColor::Black => "Black".to_owned(),
            ResistorColor::Blue => "Blue".to_owned(),
            ResistorColor::Brown => "Brown".to_owned(),
            ResistorColor::Green => "Green".to_owned(),
            ResistorColor::Grey => "Grey".to_owned(),
            ResistorColor::Orange => "Orange".to_owned(),
            ResistorColor::Red => "Red".to_owned(),
            ResistorColor::Violet => "Violet".to_owned(),
            ResistorColor::White => "White".to_owned(),
            ResistorColor::Yellow => "Yellow".to_owned(),
        },
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
