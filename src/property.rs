use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{Float, Integer};

pub type PropertyName = String;
pub type PropertyMap = HashMap<PropertyName, Property>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Property {
    String(String),
    Int(Integer),
    Float(Float),
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
