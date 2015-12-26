use super::ll::attr_t;
use std::ops::{BitOr};

/* WILL BE REMOVED
#[macro_export]
macro_rules! attributes {
    ( $( $attr:expr ),+ ) => {
        &[$( $attr ),+] as &[Attribute]
    };
}
*/

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

pub enum Attribute {
    Underline,
    Bold,
}

pub trait ScalarAttribute {
    fn to_attr_t(&self) -> attr_t;
}

impl ScalarAttribute for Attribute {
    fn to_attr_t(&self) -> attr_t {
        match *self {
            Attribute::Underline => 131072,  // 2^17
            Attribute::Bold      => 2097152, // 2^21
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

/* WILL BE REMOVED
impl<'a> ScalarAttribute for &'a[Attribute] {
    fn to_attr_t(&self) -> attr_t {
        let mut result: attr_t = 0;
        for attribute in *self {
            result = result | attribute.to_attr_t();
        }
        result
    }
}
*/

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
