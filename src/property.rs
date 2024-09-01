use std::{collections::HashMap, fmt::Display, ops::Range};

use serde::{Deserialize, Serialize};

use crate::{entity::EntityType, Int, Real};

pub type PropertyName = String;
pub type PropertyDefMap = HashMap<PropertyName, PropertyType>;
// TODO: newtype
pub type PropertyMap = HashMap<PropertyName, Property>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PropertyType {
    Nil,
    Int,
    Real,
    String,
    Enum(String), // inner string is a label to the user-defined set of values
    // IntRange(Int, Int),
    // RealRange(Real, Real), // Let's not focus too much on number props for now
    Entity(EntityType), // useful for relationships and instructions
    Properties(Vec<PropertyName>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Property {
    String(String),
    Int(Int),
    Float(Real),
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

impl From<Int> for Property {
    fn from(val: Int) -> Self {
        Property::Int(val)
    }
}

impl From<Real> for Property {
    fn from(val: Real) -> Self {
        Property::Float(val)
    }
}
