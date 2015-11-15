use super::ll::attr_t;

#[macro_export]
macro_rules! attributes {
    ( $( $attr:expr ),+ ) => {
        &[$( $attr ),+] as &[Attribute]
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

impl<'a> ScalarAttribute for &'a[Attribute] {
    fn to_attr_t(&self) -> attr_t {
        let mut result: attr_t = 0;
        for attribute in *self {
            result = result | attribute.to_attr_t();
        }
        result
    }
}
