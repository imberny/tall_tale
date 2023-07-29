use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::{
    property::{Property, PropertyMap, PropertyName},
    story_node::Alias,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum Constraint {
    Has(PropertyName),
    Equals(PropertyName, Property),
    IsInRange(PropertyName, Range<i64>),
}

impl Constraint {
    pub fn is_in_range<N>(property_name: N, range: Range<i64>) -> Self
    where
        N: Into<PropertyName>,
    {
        Self::IsInRange(property_name.into(), range)
    }

    pub fn has<N>(property_name: N) -> Self
    where
        N: Into<PropertyName>,
    {
        Self::Has(property_name.into())
    }

    pub fn equals<N, P>(property_name: N, to: P) -> Self
    where
        N: Into<PropertyName>,
        P: Into<Property>,
    {
        Self::Equals(property_name.into(), to.into())
    }

    pub fn is_satisfied_by(&self, properties: &PropertyMap) -> bool {
        match self {
            Constraint::Has(prop_name) => properties.get(prop_name).is_some(),
            Constraint::Equals(prop_name, property) => properties
                .get(prop_name)
                .is_some_and(|ent_prop| property == ent_prop),
            Constraint::IsInRange(prop_name, range) => {
                properties.get(prop_name).is_some_and(|prop| match prop {
                    Property::Int(value) => range.contains(value),
                    _ => false,
                })
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AliasRelation {
    pub me: Alias,
    pub other: Alias,
    pub constraints: Vec<Constraint>,
}

impl AliasRelation {
    pub fn new<A, C>(me: A, other: A, constraints: C) -> Self
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        Self {
            me: me.into(),
            other: other.into(),
            constraints: Vec::from_iter(constraints),
        }
    }

    pub fn is_satisfied_by(&self, properties: &PropertyMap) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(properties))
    }
}
