use std::{collections::HashMap, fmt::Display, ops::Range};

use serde::{Deserialize, Serialize};

use crate::{Float, Integer};

pub type PropertyName = String;
// TODO: newtype
pub type PropertyMap = HashMap<PropertyName, Property>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Property {
    String(String),
    Int(Integer),
    Float(Float),
}

impl Property {
    pub fn is_in_range(&self, range: &Range<i64>) -> bool {
        match self {
            Property::Int(value) => range.contains(value),
            _ => false,
        }
    }
    pub fn is_in_range_float(&self, range: &Range<f64>) -> bool {
        match self {
            Property::Float(value) => range.contains(value),
            _ => false,
        }
    }
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

impl From<Integer> for Property {
    fn from(val: Integer) -> Self {
        Property::Int(val)
    }
}

impl From<Float> for Property {
    fn from(val: Float) -> Self {
        Property::Float(val)
    }
}
