use super::ll::attr_t;

pub enum Attribute {
    Bold,
}

pub trait ScalarAttribute {
    fn to_attr_t(&self) -> attr_t;
}

impl ScalarAttribute for Attribute {
    fn to_attr_t(&self) -> attr_t {
        match self {
            Bold => 2097152 // 2^21
        }
    }
}
