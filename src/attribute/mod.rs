use super::ll::{attr_t, color_t};
use std::ops::{BitOr};

#[macro_export]
macro_rules! chtype_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec : Vec<chtype> = Vec::new();
            $( temp_vec.push($x.to_attr_t()); )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! string_as_chtype {
    ( $string:expr ) => {
        $string.chars().map(|c| c.to_attr_t()).collect::<Vec<chtype>>()
    }
}

#[macro_export]
macro_rules! color_pair {
    ( $num:expr ) => {
        ($num as attr_t) << 8
    }
}

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn to_color_t(&self) -> color_t {
        match *self {
            Color::Black   => 0,
            Color::Red     => 1,
            Color::Green   => 2,
            Color::Yellow  => 3,
            Color::Blue    => 4,
            Color::Magenta => 5,
            Color::Cyan    => 6,
            Color::White   => 7,
        }
    }
}

pub enum Attribute {
    Blink,
    Bold,
    Color,
    Dim,
    Italic,
    Normal,
    Reverse,
    Standout,
    Underline,
}

pub trait ScalarAttribute {
    fn to_attr_t(&self) -> attr_t;
}

impl ScalarAttribute for Attribute {
    fn to_attr_t(&self) -> attr_t {
        match *self {
            Attribute::Normal    => 0,
            Attribute::Color     => 65280,      // 0xFF00
            Attribute::Standout  => 65536,      // 2 << 16
            Attribute::Underline => 131072,     // 2 << 17
            Attribute::Reverse   => 262144,     // 2 << 18
            Attribute::Blink     => 524288,     // 2 << 19
            Attribute::Dim       => 1048576,    // 2 << 20
            Attribute::Bold      => 2097152,    // 2 << 21
            Attribute::Italic    => 2147483648, // 2 << 31
        }
    }
}

impl ScalarAttribute for attr_t {
    fn to_attr_t(&self) -> attr_t {
        *self
    }
}

impl ScalarAttribute for char {
    fn to_attr_t(&self) -> attr_t {
        *self as attr_t
    }
}

impl BitOr for Attribute {
    type Output = attr_t;
    fn bitor(self, rhs: Attribute) -> attr_t {
        self.to_attr_t() | rhs.to_attr_t()
    }
}

impl BitOr<Attribute> for char {
    type Output = attr_t;
    fn bitor(self, rhs: Attribute) -> attr_t {
        self.to_attr_t() | rhs.to_attr_t()
    }
}

impl BitOr<Attribute> for attr_t {
    type Output = attr_t;
    fn bitor(self, rhs: Attribute) -> attr_t {
        self.to_attr_t() | rhs.to_attr_t()
    }
}
