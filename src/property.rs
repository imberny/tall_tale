use std::{collections::HashMap, fmt::Display};

pub type PropertyName = String;
pub type PropertyMap = HashMap<PropertyName, Property>;

#[derive(PartialEq, Clone)]
pub enum Property {
    String(String),
    Int(i64),
    Float(f64),
}

impl Display for Property {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::String(s) => write!(fmt, "{}", s),
            Property::Int(i) => write!(fmt, "{}", i),
            Property::Float(f) => write!(fmt, "{}", f),
        }
    }
}

impl From<&str> for Property {
    fn from(val: &str) -> Self {
        Property::String(val.into())
    }
}

impl From<String> for Property {
    fn from(val: String) -> Self {
        Property::String(val)
    }
}

impl From<i64> for Property {
    fn from(val: i64) -> Self {
        Property::Int(val)
    }
}

impl From<f64> for Property {
    fn from(val: f64) -> Self {
        Property::Float(val)
    }
}
