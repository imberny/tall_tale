use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::{
    entity::Entity,
    property::{Property, PropertyName},
    story_node::Alias,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum PropertyConstraint {
    Has(PropertyName),
    IsInRange(PropertyName, Range<i64>),
}

impl PropertyConstraint {
    pub fn is_satisfied_by(&self, entity: &Entity) -> bool {
        match self {
            PropertyConstraint::Has(prop_name) => entity.get(prop_name).is_some(),
            PropertyConstraint::IsInRange(prop_name, range) => {
                entity.get(prop_name).is_some_and(|prop| match prop {
                    Property::Int(value) => range.contains(value),
                    _ => false,
                })
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RelationConstraint {
    pub me: Alias,
    pub other: Alias,
    pub constraint: PropertyConstraint,
}

impl RelationConstraint {
    pub fn new<T: Into<Alias>>(me: T, other: T, constraint: PropertyConstraint) -> Self {
        Self {
            me: me.into(),
            other: other.into(),
            constraint,
        }
    }
}
